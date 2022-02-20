#![warn(
    clippy::all,
    clippy::pedantic,
    rust_2018_idioms,
    rust_2021_compatibility,
    missing_copy_implementations,
    missing_debug_implementations,
    single_use_lifetimes,
    trivial_casts,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![allow(
    clippy::many_single_char_names,
    non_snake_case,
    unused_imports,
    unused_labels,
    unused_macros,
    dead_code
)]

mod problem;
#[cfg(test)]
mod tests;

pub use problem::*;

fn main() {
    problem::main();
}
