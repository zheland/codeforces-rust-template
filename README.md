# Template for Codeforces contests for Rust language

## About

This crate provides several abstractions for readme and documentation front page content
as well as multiple readme and documentation parsing and transformation functions.
With them, readme and documentation can be converted
to a set of markup nodes that are expected to be the same.
Their equality can be checked with the `assert_sync` function,
which also provides useful diagnostic messages about the differences found.

Documentation parser accepts not only inner doc-comments (`//!`) but also
inner doc-attributes (`#[!cfg(...)]` and `#[!cfg_attr(...)]`).
This is useful when some doc-tests require certain features to compile and run.

## Usage

- The `template.rs` file is a template for problem solution files.
- Files `a1.rs`, `a2.rs`, ... `h1.rs,` `h2.rs`
  are the files in which it is supposed to solve contest problems.
- Files `z1.rs`, `z2.rs` are the additional files that, for example,
  can be used to solve problems from the archive or for experiments.
- The extensions folder contains a large number of additional modules,
  that are not included in the `template.rs` due to the size limit of 64kilobytes.
- Use `restart.sh` to reset all binary source files to `template.rs`;
- Use `watch.sh` to start watch script that run tests for modified source files;

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any
additional terms or conditions.
