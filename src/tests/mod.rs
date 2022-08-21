#![warn(
    clippy::many_single_char_names,
    clippy::missing_errors_doc,
    non_snake_case,
    unused_imports
)]

mod channel_io;
mod examples;
mod interactive;
mod trim_lines;
mod util;

#[cfg(all(test, feature = "libtests"))]
mod libtests;

pub use channel_io::*;
pub use examples::*;
pub use interactive::*;
pub use trim_lines::*;
pub use util::*;
