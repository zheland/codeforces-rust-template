use core::cmp::min;
use core::convert::Infallible;
use core::fmt::Write as FmtWrite;
use core::num::NonZero;
use core::str::from_utf8;
use core::time::Duration;
use memchr::memmem;
use std::io::{Error as IoError, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::thread::available_parallelism;
use std::time::{Instant, SystemTime};
use tokio::io::{
    AsyncBufRead, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader,
    sink,
};
use tokio::process::{Child, Command};
use tokio::sync::Semaphore;
use tokio::time::error::Elapsed;
use tokio::time::{MissedTickBehavior, interval, timeout};
use tokio::{fs, task};

use color_eyre::eyre::{Error as EyreError, eyre};
use hashbrown::HashMap;
use tokio_util::sync::CancellationToken;

const FILE_SIZE_HARD_LIMIT: u64 = 0x1_0000;
const FILE_SIZE_SOFT_LIMIT: u64 = 0xF000;
const CHECK_INTERVAL: Duration = Duration::from_millis(100);
const DEBOUNCE_DELAY: Duration = Duration::from_millis(10);
const SECTION1_STYLE: &str = "\x1b[0;34m";
const SECTION2_STYLE: &str = "\x1b[0;36m";
const ADDED_STYLE: &str = "\x1b[0;32m";
const OK_STYLE: &str = "\x1b[0;32m";
const REMOVED_STYLE: &str = "\x1b[0;31m";
const ERROR_STYLE: &str = "\x1b[0;31m";
const WARN_STYLE: &str = "\x1b[0;33m";
const NOTE_STYLE: &str = "\x1b[0;90m";
const DEFAULT_STYLE: &str = "\x1b[0m";

mod preset_tests_deps {
    use {rand as _, rand_chacha as _};
}

pub fn main() -> Result<(), EyreError> {
    env_logger::init();
    color_eyre::install()?;
    let cancelled = CancellationToken::new();
    ctrlc::set_handler({
        let cancelled = cancelled.clone();
        move || {
            log::info!("Received Ctrl+C!");
            cancelled.cancel();
        }
    })?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(app(cancelled))?;

    Ok(())
}

async fn app(cancellation_token: CancellationToken) -> Result<(), EyreError> {
    let None = cancellation_token
        .run_until_cancelled_owned(Box::pin(watcher()))
        .await
        .transpose()?;

    Ok(())
}

async fn watcher() -> Result<Infallible, EyreError> {
    let start_time = Instant::now();
    let mut files_modified_times: HashMap<PathBuf, SystemTime> = HashMap::new();
    let mut interval = interval(CHECK_INTERVAL);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        let mut stdout = std::io::stdout();
        let mut has_updates = false;
        let mut files = fs::read_dir("src/tasks").await?;
        while let Some(entry) = files.next_entry().await? {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let metadata = fs::metadata(&path).await?;
            let modified = metadata.modified()?;
            if let Some(prev_modified) = files_modified_times.get(&path)
                && (*prev_modified == modified || prev_modified.elapsed()? < DEBOUNCE_DELAY)
            {
                continue;
            }

            has_updates = true;
            write!(stdout, "\x1B[2K\r")?;
            match build_and_check(&path).await {
                Ok(()) => {}
                Err(err) => log::error!("{err:?}"),
            }
            let _: Option<_> = files_modified_times.insert(path, modified);
        }

        if !has_updates {
            let elapsed = start_time.elapsed().as_secs_f64();
            write!(stdout, "Stand-By: {elapsed:8.3}s\r")?;
            stdout.flush()?;
            let _ = interval.tick().await;
        }
    }
}

async fn build_and_check(path: &Path) -> Result<(), EyreError> {
    let task = path
        .file_name()
        .ok_or(eyre!("unexpected path {}", path.display()))?
        .to_str()
        .ok_or(eyre!("non-Unicode file name {}", path.display()))?
        .strip_suffix(".rs")
        .ok_or(eyre!("file {} doesn't have `rs` extension", path.display()))?;

    let metadata = fs::metadata(&path).await?;
    let file_size = metadata.len();
    for (qualifier, limit, style) in [
        ("too ", FILE_SIZE_HARD_LIMIT, ERROR_STYLE),
        ("", FILE_SIZE_SOFT_LIMIT, WARN_STYLE),
    ] {
        if file_size > limit {
            let mut stdout = std::io::stdout();
            writeln!(
                stdout,
                "{style}File {} is {qualifier}big: \
                 {file_size} bytes > {limit} bytes{DEFAULT_STYLE}",
                path.display()
            )?;
            break;
        }
    }

    let mut cmd = Command::new("cargo");
    let &mut _ = cmd.kill_on_drop(true);
    let &mut _ = cmd.arg("build");
    let &mut _ = cmd.env("RUST_MIN_STACK", "268435456");
    let &mut _ = cmd.args(["--profile", "test-solver", "--features", "stderr"]);
    let &mut _ = cmd.args(["--bin", task]);
    run_cmd_checked(cmd).await?;

    let mut cmd = Command::new("cargo");
    let &mut _ = cmd.kill_on_drop(true);
    let &mut _ = cmd.arg("build");
    let &mut _ = cmd.args(["--profile", "test-bridge", "--features", "stderr,dev-mode"]);
    let &mut _ = cmd.args(["--bin", task]);
    run_cmd_checked(cmd).await?;

    check(task).await?;

    Ok(())
}

async fn run_cmd_checked(mut command: Command) -> Result<(), EyreError> {
    let status = command.status().await?;
    if status.success() {
        Ok(())
    } else if let Some(code) = status.code() {
        Err(eyre!("solver build failed with {code}"))
    } else {
        Err(eyre!("solver build was terminated"))
    }
}

async fn check(task_name: &str) -> Result<(), EyreError> {
    let timeout = bridge_request(task_name, "timeout-seconds").await?;
    let timeout = Duration::from_secs_f64(from_utf8(&timeout)?.trim().parse()?);

    let test_gen_timeout = bridge_request(task_name, "test-gen-timeout-seconds").await?;
    let test_gen_timeout = Duration::from_secs_f64(from_utf8(&test_gen_timeout)?.trim().parse()?);

    let memory_limit_bytes = bridge_request(task_name, "memory-limit-bytes").await?;
    let memory_limit_bytes: u64 = from_utf8(&memory_limit_bytes)?.trim().parse()?;

    let mut examples_child = bridge_piped(task_name, "examples")?;
    let mut tests_child = bridge_piped(task_name, "tests")?;
    let interactive_presets_child = bridge_piped(task_name, "interactive-presets")?;

    let settings = Settings {
        timeout,
        test_gen_timeout,
        memory_limit_bytes,
    };

    let num_cpus = available_parallelism().map(NonZero::get).unwrap_or(1);
    let limited_time_task_semaphore = Arc::new(Semaphore::new(num_cpus));

    let mut tasks = Vec::new();
    for (tests_child, kind) in [(&mut examples_child, "EXPL"), (&mut tests_child, "TEST")] {
        let tests = tests_child.stdout.take();
        let tests = tests.ok_or(eyre!("unexpected stdout absence"))?;

        let mut tests = BufReader::new(tests);
        let Some(test_separator) = read_non_empty_line(&mut tests).await? else {
            break;
        };
        let io_separator = read_non_empty_line(&mut tests)
            .await?
            .ok_or_else(|| eyre!("failed to find IO separator"))?;

        let mut buffer = vec![];
        let mut is_eof = false;
        copy_trimmed_until_separator(
            &mut tests,
            &mut sink(),
            &mut buffer,
            test_separator.as_bytes(),
            &mut is_eof,
        )
        .await?;

        let mut test_no = 1;
        while !is_eof {
            let _permit = limited_time_task_semaphore.acquire();
            let (input_reader, input_writer) = tokio::io::simplex(8192);
            let (output_reader, output_writer) = tokio::io::simplex(8192);
            let task = task_name.to_owned();
            let task = task::spawn(async move {
                test_with_preset(&task, input_reader, output_reader, &settings).await
            });

            for (mut writer, separator) in [
                (input_writer, &io_separator),
                (output_writer, &test_separator),
            ] {
                copy_trimmed_until_separator(
                    &mut tests,
                    &mut writer,
                    &mut buffer,
                    separator.as_bytes(),
                    &mut is_eof,
                )
                .await?;
                writer.shutdown().await?;
            }

            tasks.push((task, kind, test_no));
            test_no += 1;
        }
    }

    // TODO: Interactive task handler.
    let _unused = interactive_presets_child;

    let mut stdout = std::io::stdout();
    for (task, kind, no) in tasks {
        write!(
            stdout,
            "{SECTION1_STYLE}{task_name} {kind}{no:02}{DEFAULT_STYLE}: "
        )?;
        stdout.flush()?;
        match task.await? {
            Ok((stdin, is_stdin_full)) => {
                let mut stdin = stdin.replace('\n', "  ");
                if stdin.len() > 48 || !is_stdin_full {
                    let until = stdin.len().min(45);
                    stdin = format!(
                        "{}...",
                        str_from_valid_part(&stdin.as_bytes()[..until]).to_owned()
                    );
                }

                writeln!(stdout, "{OK_STYLE}OK{DEFAULT_STYLE}  {stdin}")?;
            }
            Err(err) => writeln!(stdout, "{err}")?,
        }
    }

    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct Settings {
    timeout: Duration,
    test_gen_timeout: Duration,
    memory_limit_bytes: u64,
}

// async fn run_solver(task: &str, ) -> Result<(), EyreError> {
//     Ok(())
// }

fn bridge_cmd(task: &str, arg: &str) -> Command {
    let mut cmd = Command::new(format!("target/test-bridge/{task}"));
    let &mut _ = cmd.arg(arg).kill_on_drop(true);
    cmd
}

async fn bridge_request(task: &str, arg: &str) -> Result<Vec<u8>, EyreError> {
    let stdout = bridge_cmd(task, arg)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .await?
        .stdout;
    Ok(stdout)
}

fn bridge_piped(task: &str, arg: &str) -> Result<Child, EyreError> {
    let child = bridge_cmd(task, arg)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;
    Ok(child)
}

#[expect(clippy::too_many_lines, reason = "ignore for now")] // FIXME
async fn test_with_preset<I, E>(
    task: &str,
    mut input: I,
    expected: E,
    settings: &Settings,
) -> Result<(String, bool), EyreError>
where
    I: 'static + Unpin + Send + AsyncRead,
    E: 'static + Unpin + Send + AsyncRead,
{
    const BYTES_BUFFERED: usize = 64 * 1024;

    let expected_reader_task = task::spawn(async_read_all(expected));

    let (mut solver, stdin, is_stdin_full, stdout_reader_task, stderr_reader_task) =
        timeout(settings.test_gen_timeout, async move {
            let mut stdin_buf = Vec::new();
            let mut is_stdin_full = false;
            while stdin_buf.len() < BYTES_BUFFERED {
                if input.read_buf(&mut stdin_buf).await? == 0 {
                    is_stdin_full = true;
                    break;
                }
            }

            let mut solver = Command::new(format!("target/test-solver/{task}"))
                .kill_on_drop(true)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;

            let pid = solver
                .id()
                .ok_or_else(|| eyre!("{ERROR_STYLE}Solver pid was lost{DEFAULT_STYLE}"))?;

            let output = Command::new("prlimit")
                .arg("--pid")
                .arg(format!("{pid}"))
                .arg(format!("--rss={}", settings.memory_limit_bytes))
                .output()
                .await?;

            if !output.status.success() {
                return Err(eyre!(
                    "{ERROR_STYLE}prlimit failed with status {}{DEFAULT_STYLE}",
                    output.status
                ));
            }

            let mut stdin = solver.stdin.take().unwrap();
            let stdout = solver.stdout.take().unwrap();
            let stderr = solver.stderr.take().unwrap();

            match stdin.write_all(stdin_buf.as_ref()).await {
                Ok(()) => {}
                #[allow(clippy::wildcard_enum_match_arm)]
                Err(err) => match err.kind() {
                    ErrorKind::BrokenPipe => {}
                    _ => return Err(err.into()),
                },
            }
            let stdin_writer_task = task::spawn(async_copy_all(input, stdin));
            let stdout_reader_task = task::spawn(async_read_all(stdout));
            let stderr_reader_task = task::spawn(async_read_all(stderr));

            stdin_writer_task.await??;

            Result::<_, EyreError>::Ok((
                solver,
                stdin_buf,
                is_stdin_full,
                stdout_reader_task,
                stderr_reader_task,
            ))
        })
        .await
        .map_err(|Elapsed { .. }| eyre!("{ERROR_STYLE}Test generation timeout{DEFAULT_STYLE}"))??;

    let (stdout, stderr) = timeout(settings.timeout, async move {
        let stdout = stdout_reader_task.await??;
        let stderr = stderr_reader_task.await??;
        Result::<_, EyreError>::Ok((stdout, stderr))
    })
    .await
    .map_err(|Elapsed { .. }| eyre!("{ERROR_STYLE}Solver timeout{DEFAULT_STYLE}"))??;

    let expected = expected_reader_task.await??;

    let stdout = String::from_utf8(stdout)?;
    let stderr = String::from_utf8(stderr)?;
    let expected = String::from_utf8(expected)?;

    let stdout = stdout.trim();
    let stderr = stderr.trim();
    let expected = expected.trim();

    let stderr_note = if stderr.is_empty() { " (empty)" } else { "\n" };
    // let stdout_note = if stderr.is_empty() { " (empty)" } else { "\n" };

    let status = solver
        .try_wait()
        .map_err(|IoError { .. }| eyre!("{ERROR_STYLE}Solver stalled{DEFAULT_STYLE}"))?
        .ok_or_else(|| eyre!("{ERROR_STYLE}Exit status not available{DEFAULT_STYLE}"))?;

    let stdin = str_from_valid_part(&stdin);

    if status.success() && stdout == expected {
        Ok((stdin.to_owned(), is_stdin_full))
    } else {
        let (stdin_len, is_stdin_full) = memmem::find_iter(stdin.as_bytes(), "\n")
            .nth(3)
            .map_or((stdin.len(), is_stdin_full), |pos| (pos, false));
        let stdin = &stdin[0..stdin_len];
        let (stdin_note, stdin_suffix) = if is_stdin_full {
            ("", "")
        } else {
            (" (reduced)", "...")
        };

        let mut report = String::new();
        if status.success() {
            writeln!(report, "{ERROR_STYLE}Wrong answer{DEFAULT_STYLE}")?;
        } else if let Some(code) = status.code() {
            writeln!(
                report,
                "{ERROR_STYLE}Solver failed with status {code}{DEFAULT_STYLE}",
            )?;
        } else {
            writeln!(report, "{ERROR_STYLE}Solver failed{DEFAULT_STYLE}")?;
        }

        let mut diff = String::new();
        write_diff(&mut diff, stdout, expected)?;

        writeln!(
            report,
            "{SECTION2_STYLE}Stdin{DEFAULT_STYLE}:{stdin_note}\n{stdin}{stdin_suffix}"
        )?;
        write!(report, "{SECTION2_STYLE}Stdout{DEFAULT_STYLE}:\n{diff}")?;
        writeln!(
            report,
            "{SECTION2_STYLE}Stderr{DEFAULT_STYLE}:{stderr_note}{stderr}"
        )?;
        // writeln!(
        //     report,
        //     "{SECTION2_STYLE}Stdout{DEFAULT_STYLE}:{stdout_note}{stdout}"
        // )?;
        Err(eyre!("{}", report.trim()))
    }
}

async fn async_copy_all<R, W>(mut reader: R, mut writer: W) -> Result<(), EyreError>
where
    W: Unpin + AsyncWrite,
    R: Unpin + AsyncRead,
{
    let mut input_buf = Vec::new();
    while reader.read_buf(&mut input_buf).await? != 0 {
        writer.write_all(&input_buf).await?;
        input_buf.clear();
    }
    writer.shutdown().await?;

    Ok(())
}

async fn async_read_all<R>(mut reader: R) -> Result<Vec<u8>, EyreError>
where
    R: Unpin + AsyncRead,
{
    let mut buf = Vec::new();
    let _: usize = reader.read_to_end(&mut buf).await?;
    Ok(buf)
}

fn write_diff(target: &mut String, given: &str, expected: &str) -> Result<(), EyreError> {
    let mut given_it = given.split('\n').enumerate().peekable();
    let mut expected_it = expected.split('\n').enumerate().peekable();

    loop {
        let (given, expected) = (given_it.next(), expected_it.next());
        match (given, expected) {
            (None, None) => break,
            (Some(given), Some(expected)) => {
                if given == expected {
                    writeln!(target, "{}", given.1)?;
                } else {
                    // "\x1b[0;31m  - {}\x1b[0m (line {})", given.0 + 1
                    writeln!(
                        target,
                        "{REMOVED_STYLE}{}{DEFAULT_STYLE} {NOTE_STYLE}(bad){DEFAULT_STYLE}",
                        given.1,
                    )?;
                    // "\x1b[0;32m  + {}\x1b[0m (line {})", expected.0 + 1
                    writeln!(target, "{ADDED_STYLE}{}{DEFAULT_STYLE}", expected.1)?;
                }
            }
            (Some(given), None) => {
                writeln!(target, "{REMOVED_STYLE}{}{DEFAULT_STYLE} (bad)", given.1,)?;
            }
            (None, Some(expected)) => {
                writeln!(target, "{ADDED_STYLE}{}{DEFAULT_STYLE}", expected.1)?;
            }
        }
    }

    Ok(())
}

async fn read_non_empty_line<R>(reader: &mut R) -> Result<Option<String>, EyreError>
where
    R: Unpin + AsyncBufRead,
{
    let mut line = String::new();
    while line.is_empty() {
        let len = reader.read_line(&mut line).await?;
        if len == 0 {
            return Ok(None);
        }
        #[expect(clippy::assigning_clones, reason = "wrong hint")]
        (line = line.trim().to_owned());
    }
    Ok(Some(line))
}

async fn copy_trimmed_until_separator<R, W>(
    reader: &mut R,
    writer: &mut W,
    buffer: &mut Vec<u8>,
    separator: &[u8],
    is_eof: &mut bool,
) -> Result<(), EyreError>
where
    R: Unpin + AsyncRead,
    W: Unpin + AsyncWrite,
{
    let mut is_start_trimmed = false;
    loop {
        if *is_eof {
            let mut trimmed = buffer.trim_ascii_end();
            if !is_start_trimmed && !trimmed.is_empty() {
                trimmed = trimmed.trim_ascii_start();
            }
            writer.write_all(trimmed).await?;
            let _unused = buffer.drain(..);
            break;
        }
        if let Some(pos) = memmem::find(buffer, separator) {
            let mut trimmed = buffer[0..pos].trim_ascii_end();
            if !is_start_trimmed {
                trimmed = trimmed.trim_ascii_start();
            }
            writer.write_all(trimmed).await?;
            let _unused = buffer.drain(0..pos + separator.len());
            break;
        }

        if let Some(until) = buffer.len().checked_sub(separator.len()) {
            let mut trimmed = buffer[0..until].trim_ascii_end();
            let until = trimmed.len();
            if !is_start_trimmed && !trimmed.is_empty() {
                trimmed = trimmed.trim_ascii_start();
                is_start_trimmed = true;
            }
            writer.write_all(trimmed).await?;
            let _unused = buffer.drain(0..until);
        }
        let mut read_buf = [0; 8192];
        let len = reader.read(&mut read_buf).await?;
        buffer.extend_from_slice(&read_buf[0..len]);
        *is_eof |= len == 0;
    }

    Ok(())
}

fn str_from_valid_part(bytes: &[u8]) -> &str {
    from_utf8(bytes).unwrap_or_else(|err| from_utf8(&bytes[..err.valid_up_to()]).unwrap())
}
