//! Pure Rust implementation of Ryū, an algorithm to quickly convert floating
//! point numbers to decimal strings.
//!
//! The PLDI'18 paper [*Ryū: fast float-to-string conversion*][paper] by Ulf
//! Adams includes a complete correctness proof of the algorithm. The paper is
//! available under the creative commons CC-BY-SA license.
//!
//! This Rust implementation is a line-by-line port of Ulf Adams' implementation
//! in C, [https://github.com/ulfjack/ryu][upstream]. The [`ryu::raw`][raw]
//! module exposes exactly the API and formatting of the C implementation as
//! unsafe pure Rust functions. There is additionally a safe API as demonstrated
//! in the example code below. The safe API uses the same underlying Ryū
//! algorithm but diverges from the formatting of the C implementation to
//! produce more human-readable output, for example `0.3` rather than `3E-1`.
//!
//! [paper]: https://dl.acm.org/citation.cfm?id=3192369
//! [upstream]: https://github.com/ulfjack/ryu
//! [raw]: raw/index.html
//!
//! # Example
//!
//! ```edition2018
//! fn main() {
//!     let mut buffer = ryu::Buffer::new();
//!     let printed = buffer.format(1.234);
//!     assert_eq!(printed, "1.234");
//! }
//! ```
//!
//! ## Performance
//!
//! You can run upstream's benchmarks with:
//!
//! ```console
//! $ git clone https://github.com/ulfjack/ryu c-ryu
//! $ cd c-ryu
//! $ bazel run -c opt //ryu/benchmark
//! ```
//!
//! And the same benchmark against our implementation with:
//!
//! ```console
//! $ git clone https://github.com/dtolnay/ryu rust-ryu
//! $ cd rust-ryu
//! $ cargo run --example upstream_benchmark --release
//! ```
//!
//! These benchmarks measure the average time to print a 32-bit float and average
//! time to print a 64-bit float, where the inputs are distributed as uniform random
//! bit patterns 32 and 64 bits wide.
//!
//! The upstream C code, the unsafe direct Rust port, and the safe pretty Rust API
//! all perform the same, taking around 21 nanoseconds to format a 32-bit float and
//! 31 nanoseconds to format a 64-bit float.
//!
//! There is also a Rust-specific benchmark comparing this implementation to the
//! standard library which you can run with:
//!
//! ```console
//! $ cargo bench
//! ```
//!
//! The benchmark shows Ryu approximately 4-10x faster than the standard library
//! across a range of f32 and f64 inputs. Measurements are in nanoseconds per
//! iteration; smaller is better.
//!
//! | type=f32 | 0.0  | 0.1234 | 2.718281828459045 | f32::MAX |
//! |:--------:|:----:|:------:|:-----------------:|:--------:|
//! | RYU      | 3ns  | 28ns   | 23ns              | 22ns     |
//! | STD      | 40ns | 106ns  | 128ns             | 110ns    |
//!
//! | type=f64 | 0.0  | 0.1234 | 2.718281828459045 | f64::MAX |
//! |:--------:|:----:|:------:|:-----------------:|:--------:|
//! | RYU      | 3ns  | 50ns   | 35ns              | 32ns     |
//! | STD      | 39ns | 105ns  | 128ns             | 202ns    |
//!
//! ## Formatting
//!
//! This library tends to produce more human-readable output than the standard
//! library's to\_string, which never uses scientific notation. Here are two
//! examples:
//!
//! - *ryu:* 1.23e40, *std:* 12300000000000000000000000000000000000000
//! - *ryu:* 1.23e-40, *std:* 0.000000000000000000000000000000000000000123
//!
//! Both libraries print short decimals such as 0.0000123 without scientific
//! notation.

#![no_std]
#![doc(html_root_url = "https://docs.rs/ryu/0.2.8")]
#![cfg_attr(feature = "cargo-clippy", allow(renamed_and_removed_lints))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        cast_lossless,
        cyclomatic_complexity,
        many_single_char_names,
        needless_pass_by_value,
        unreadable_literal,
    )
)]

#[cfg(feature = "no-panic")]
extern crate no_panic;

mod buffer;
mod common;
mod d2s;
#[cfg(not(feature = "small"))]
mod d2s_full_table;
mod d2s_intrinsics;
#[cfg(feature = "small")]
mod d2s_small_table;
mod digit_table;
mod f2s;
mod pretty;

pub use buffer::{Buffer, Float};

/// Unsafe functions that exactly mirror the API of the C implementation of Ryū.
pub mod raw {
    pub use d2s::d2s_buffered_n;
    pub use f2s::f2s_buffered_n;
    pub use pretty::d2s_buffered_n as pretty_d2s_buffered_n;
    pub use pretty::f2s_buffered_n as pretty_f2s_buffered_n;
}
