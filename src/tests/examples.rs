use std::io::{self, BufReader, Write};
use std::thread;

use crate::tests::TrimLines;
use crate::Io;

#[allow(single_use_lifetimes)]
pub fn test_with_examples<F1>(
    problem: F1,
    examples: &'static str,
    should_use_multiple_threads: bool,
) where
    F1: 'static + Clone + FnOnce(&mut Io<BufReader<&[u8]>, &mut Vec<u8>>) + Send + Sync,
{
    let mut is_ok = true;
    let mut splitted = examples.split_whitespace();
    let io_separator = splitted.next().unwrap().to_owned();
    let case_seperator = splitted.next().unwrap();

    let handles: Vec<_> = examples
        .split(case_seperator)
        .skip(1)
        .map(|case| {
            let problem = problem.clone();
            let io_separator = io_separator.clone();
            let handle = thread::spawn(move || {
                let example: Vec<&str> = case.split(&io_separator).collect();
                assert_eq!(example.len(), 2);
                let input = example[0].to_owned();
                let trimmed_input = input.trim_lines();
                for line in trimmed_input.lines() {
                    println!("< {line}");
                }
                let mut output = Vec::new();
                let result = {
                    let output = &mut output;
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
                let output = String::from_utf8(output).unwrap();
                let expected = example[1].trim_lines();
                (input, output, expected, result.is_ok())
            });
            if should_use_multiple_threads {
                handle
            } else {
                let data = handle.join().unwrap();
                thread::spawn(move || data)
            }
        })
        .collect();

    for handle in handles {
        let (input, output, expected, is_succeed) = handle.join().unwrap();
        let is_invalid_output = output != expected;
        if is_invalid_output || !is_succeed {
            let mut diff = String::new();
            let mut output_lines = output.trim_end().split('\n');
            let mut answer_lines = expected.trim_end().split('\n');
            loop {
                let (output_line, answer_line) = (output_lines.next(), answer_lines.next());
                if output_line.is_none() && answer_line.is_none() {
                    break;
                }
                match (output_line, answer_line) {
                    (None, None) => break,
                    (Some(output_line), Some(answer_line)) => {
                        if output_line == answer_line {
                            diff = diff + "\x1b[0;33m    " + output_line + "\x1b[0m\n";
                        } else {
                            diff = diff + "\x1b[0;31m  - " + output_line + "\x1b[0m\n";
                            diff = diff + "\x1b[0;32m  + " + answer_line + "\x1b[0m\n";
                        }
                    }
                    (Some(output_line), None) => {
                        diff = diff + "\x1b[0;31m  - " + output_line + "\x1b[0m\n";
                    }
                    (None, Some(answer_line)) => {
                        diff = diff + "\x1b[0;32m  + " + answer_line + "\x1b[0m\n";
                    }
                }
            }
            #[allow(clippy::explicit_write, clippy::write_literal)]
            writeln!(
                io::stderr(),
                "\x1b[0;31mError: {}.\x1b[0m\nInput:\n{}Output:\n{}Answer:\n{}Diff:\n{}",
                if !is_succeed {
                    "panicked"
                } else if is_invalid_output {
                    "invalid output"
                } else {
                    ""
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
            is_ok = false;
        }
    }
    assert!(is_ok);
}
