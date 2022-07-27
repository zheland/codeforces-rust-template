use std::io::{self, BufReader, Write};
use std::thread;

use crate::tests::TrimLines;
use crate::Io;

#[allow(single_use_lifetimes)]
pub fn test_with_examples<F1>(problem: F1, examples: &'static str)
where
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
            thread::spawn(move || {
                let example: Vec<&str> = case.split(&io_separator).collect();
                assert_eq!(example.len(), 2);
                let input = example[0].to_owned();
                let mut output = Vec::new();
                let mut io = Io::new(BufReader::new(input.as_bytes()), &mut output);
                problem(&mut io);
                let output = String::from_utf8(output).unwrap();
                let expected = example[1].trim_lines();
                (input, output, expected)
            })
        })
        .collect();

    for handle in handles {
        let (input, output, expected) = handle.join().unwrap();
        if output != expected {
            let mut diff = String::new();
            let mut output_lines = output.trim_end().split("\n");
            let mut answer_lines = expected.trim_end().split("\n");
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
            writeln!(
                io::stderr(),
                "{}\nInput:\n{}Output:\n{}Answer:\n{}Diff:\n{}",
                "\x1b[0;31mError: Invalid output.\x1b[0m",
                input
                    .trim_lines()
                    .lines()
                    .map(|line| format!("    {}\n", line))
                    .collect::<String>(),
                output
                    .lines()
                    .map(|line| format!("    {}\n", line))
                    .collect::<String>(),
                expected
                    .lines()
                    .map(|line| format!("    {}\n", line))
                    .collect::<String>(),
                diff
            )
            .unwrap();
            is_ok = false;
        }
    }
    if !is_ok {
        panic!();
    }
}
