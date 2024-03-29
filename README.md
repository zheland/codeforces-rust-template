# Template for Codeforces contests for Rust language

## About

This is a template I made for the Codeforces contests.
It contains many auxiliary functions for I/O, math and algorithms.

## Usage

- The `template.rs` file is a template for problem solution files.
- Files `a1.rs`, `a2.rs`, ..., `h1.rs,` `h2.rs`
  are the files in which it is supposed to solve contest problems.
- Files `z1.rs`, `z2.rs` are the additional files that, for example,
  can be used to solve problems from the archive or for experiments.
- The extensions folder contains a large number of additional modules,
  that are not included in the `template.rs` due to the size limit of 64kilobytes.
- Use `restart.sh` to reset all binary source files to `template.rs`
  and to run all the libraries tests.
- Use `watch.sh` to start watch script that run tests for modified source files.

## Tips
- Use `re!(VecWord).0` to read the word, e.g. a sequence of ASCII characters
  surrounded by whitespace characters;
- Use `io.li(a.wo())` to write space-separated values of the `a` variable,
  if the `a` variable is a tuple, iterator or iterable collection.
- Check out `src/tests/libtests` directory to explore the use of different types,
  traits and functions.

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any
additional terms or conditions.
