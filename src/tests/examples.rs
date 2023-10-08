use std::io::{self, BufReader, IoSlice, Write};
use std::ops::RangeBounds;
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::thread::{self, JoinHandle, Result as ThreadResult};
use std::time::Duration;

use crate::tests::TrimLines;
use crate::Io;

#[derive(Clone, Copy, Debug)]
enum Status {
    Ok,
    Panic,
    Timeout,
}

#[allow(single_use_lifetimes)]
pub fn test_with_examples<F1, R>(problem: F1, examples: &'static str, range: R, timeout: Duration)
where
    F1: 'static + Clone + FnOnce(&mut Io<BufReader<&[u8]>, &mut EchoWriter<Vec<u8>>>) + Send + Sync,
    R: RangeBounds<usize>,
{
    let mut is_all_ok = true;
    let mut splitted = examples.split_whitespace();
    let io_separator = splitted.next().unwrap().to_owned();
    let case_seperator = splitted.next().unwrap();

    let handles: Vec<_> = examples
        .split(case_seperator)
        .skip(1)
        .enumerate()
        .filter(|(j, _case)| range.contains(j))
        .map(|(_j, case)| {
            let problem = problem.clone();
            let io_separator = io_separator.clone();
            let example: Vec<&str> = case.split(&io_separator).collect();
            assert_eq!(example.len(), 2);
            let input = example[0].to_owned();
            let trimmed_input = input.trim_lines();
            let expected = example[1].trim_lines();
            let handle = thread::spawn(move || {
                let mut output = EchoWriter::new(Vec::new(), "\x1b[0;96m", "\x1b[0m", "> ");
                let result = {
                    let output = &mut output;
                    for line in trimmed_input.lines() {
                        println!("\x1b[0;94m< {line}\x1b[0m");
                    }
                    thread::scope(move |scope| {
                        scope
                            .spawn(move || {
                                let mut io =
                                    Io::new(BufReader::new(trimmed_input.as_bytes()), output);
                                problem(&mut io);
                            })
                            .join()
                    })
                };
                let output = String::from_utf8(output.into_writer()).unwrap();
                let status = if result.is_ok() {
                    Status::Ok
                } else {
                    Status::Panic
                };
                (output, status)
            });
            //let handle = if should_use_multiple_threads {
            //    handle
            /*} else {
                let data = match handle.join_with_timeout(timeout) {
                    Some(data) => data.unwrap(),
                    None => (String::new(), Status::Timeout),
                };
                thread::spawn(move || data)
            };*/
            (input, expected, handle)
        })
        .collect();

    for (input, expected, handle) in handles {
        let (output, status) = match handle.join_with_timeout(timeout) {
            Some(data) => data.unwrap(),
            None => (String::new(), Status::Timeout),
        };
        let is_invalid_output = output != expected;
        let is_this_ok = match status {
            Status::Ok => true,
            Status::Panic | Status::Timeout => false,
        };
        if is_invalid_output || !is_this_ok {
            let mut diff = String::new();
            let mut output_lines = output.trim_end().split('\n').enumerate();
            let mut answer_lines = expected.trim_end().split('\n').enumerate();
            loop {
                let (output_line, answer_line) = (output_lines.next(), answer_lines.next());
                if output_line.is_none() && answer_line.is_none() {
                    break;
                }
                match (output_line, answer_line) {
                    (None, None) => break,
                    (Some(output_line), Some(answer_line)) => {
                        if output_line == answer_line {
                            diff = format!("{diff}\x1b[0;33m    {}\x1b[0m\n", output_line.1);
                        } else {
                            diff = format!(
                                "{diff}\x1b[0;31m  - {}\x1b[0m (line {})\n",
                                output_line.1,
                                output_line.0 + 1
                            );
                            diff = format!(
                                "{diff}\x1b[0;32m  + {}\x1b[0m (line {})\n",
                                answer_line.1,
                                answer_line.0 + 1
                            );
                        }
                    }
                    (Some(output_line), None) => {
                        diff = format!(
                            "{diff}\x1b[0;31m  - {}\x1b[0m (line {})\n",
                            output_line.1,
                            output_line.0 + 1
                        );
                    }
                    (None, Some(answer_line)) => {
                        diff = format!(
                            "{diff}\x1b[0;32m  + {}\x1b[0m (line {})\n",
                            answer_line.1,
                            answer_line.0 + 1
                        );
                    }
                }
            }
            #[allow(clippy::explicit_write, clippy::write_literal)]
            writeln!(
                io::stderr(),
                "\x1b[0;31mError: {}.\x1b[0m\nInput:\n{}Output:\n{}Answer:\n{}Diff:\n{}",
                match status {
                    Status::Ok =>
                        if is_invalid_output {
                            "invalid output"
                        } else {
                            ""
                        },
                    Status::Panic => "panicked",
                    Status::Timeout => "timeout",
                },
                input
                    .trim_lines()
                    .lines()
                    .map(|line| format!("    {line}\n"))
                    .collect::<String>(),
                output
                    .lines()
                    .map(|line| format!("    {line}\n"))
                    .collect::<String>(),
                expected
                    .lines()
                    .map(|line| format!("    {line}\n"))
                    .collect::<String>(),
                diff
            )
            .unwrap();
            is_all_ok = false;
        }
    }
    assert!(is_all_ok);
}

pub trait ThreatJoinWithTimeout {
    type Output;
    #[allow(clippy::missing_errors_doc)]
    fn join_with_timeout(self, duration: Duration) -> Option<ThreadResult<Self::Output>>;
}

impl<T> ThreatJoinWithTimeout for JoinHandle<T>
where
    T: 'static + Send,
{
    type Output = T;
    fn join_with_timeout(self, duration: Duration) -> Option<ThreadResult<Self::Output>> {
        let (send, recv) = channel();
        let _thread = thread::spawn(move || {
            let result = self.join();
            send.send(result).unwrap();
        });
        match recv.recv_timeout(duration) {
            Ok(result) => Some(result),
            Err(RecvTimeoutError::Timeout) => None,
            Err(RecvTimeoutError::Disconnected) => {
                panic!("unexpected channel sender disconnection")
            }
        }
    }
}

pub struct EchoWriter<T> {
    writer: T,
    prefix: &'static str,
    suffix: &'static str,
    line_prefix: &'static str,
    new_line_and_line_prefix: String,
    is_new_line: bool,
}

impl<T> EchoWriter<T> {
    pub fn new(
        writer: T,
        prefix: &'static str,
        suffix: &'static str,
        line_prefix: &'static str,
    ) -> Self {
        Self {
            writer,
            prefix,
            suffix,
            line_prefix,
            new_line_and_line_prefix: format!("\n{prefix}"),
            is_new_line: true,
        }
    }

    pub fn into_writer(self) -> T {
        self.writer
    }

    pub fn echo(&mut self, buf: &[u8]) {
        let line_prefix = if self.is_new_line && !buf.is_empty() {
            self.is_new_line = false;
            self.line_prefix
        } else {
            ""
        };
        let output = String::from_utf8_lossy(buf);
        let output = output.replace('\n', &self.new_line_and_line_prefix);
        let output = match output.strip_suffix(&self.line_prefix) {
            Some(output) => {
                if output.ends_with('\n') {
                    self.is_new_line = true;
                    output
                } else {
                    output
                }
            }
            None => &output,
        };
        print!("{}{line_prefix}{output}{}", self.prefix, self.suffix);
    }
}

impl<T> Write for EchoWriter<T>
where
    T: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.echo(buf);
        self.writer.write(buf)
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        for buf in bufs {
            self.echo(buf);
        }
        self.writer.write_vectored(bufs)
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.echo(buf);
        self.writer.write_all(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
