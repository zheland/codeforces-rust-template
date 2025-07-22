# My Template for CodeForces contests for Rust language

## About

This is a template I made for participating in CodeForces contests.

It contains plenty of I/O trait, and auxiliary traits and functions including:
- Automated solution testing against provided examples and tests.
- Automated interactive testing support.
- Helper per-word IO reader and writer.
- `Readable` trait implemented for different parsable values and collections.
- `Writable` trait implemented for different writable values and collections.
- Often-used auxiliary methods like `dedup_count`, `gcd`, `lcm`.
- Prime siege with factorization methods.
- Modular arithmetic helper methods.
- And other auxiliary functions, traits and methods in src/extensions directory.

## Usage

- The `template.rs` file is a template for problem solution files.
- Files `a1.rs`, `a2.rs`, ..., `h1.rs,` `h2.rs`
  are the files in which it is supposed to solve contest problems.
- The extensions folder contains a large number of additional modules,
  that are not included in the `template.rs`.
- Use `restart.sh` to reset all problem source files to `template.rs`
  and to run all the libraries tests.
- Use `watch.sh` to start watch script that run tests for source files when
  modified.
- Copy examples using "firefox-copy-codeforces-tests" extension to `EXAMPLES`.
- Solve problem by modifying `solver` function.
- Whenever you save the file `watch.sh` will automatically check that the
  solution answers matches ones specified in `EXAMPLES` string.

## I/O examples
```rust
let x: usize = re.re();                       // Reads `usize` value.
let (a, b, c): (u64, u64, u64) = re.re();     // Reads three whitespace-separated `u64` values.
let arr: [u8; 16] = re.re();                  // Reads sixteen u8 values.
let vec1: Vec<u32> = vec![re.re(); x];        // Reads vector with length = `x`.
let vec1: Vec<(u32, u32)> = vec![re.re(); x]; // Reads tuples vector with length = `x`.
let vec2: Vec<(u32, u32)> = re.rec(&x);       // Reads tuples vector with length = `x`.
let str1: &str = re.re();                     // Reads whitespace-surrounded string.
let str2: &str = re.rec(&4);                  // Reads 4-bytes as string.

wr.li("Hello");                 // Write line "Hello".
wr.wo(a).wo(b);                 // Start new line "{a} {b}".
wr.wo(b).wo(c).ln();            // Finish line with " {b} {c}".
wr.wo((1..=9).wo()).ln();       // Write line "1 2 3 4 5 6 7 8 9"
wr.wo((1..=9).jo()).ln();       // Write line "123456789"
wr.wo((1, "ha", 2).wo()).ln();  // Write line "1 ha 2"
wr.fl();                        // Flushes buffer, important for interactive problems.
```

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
