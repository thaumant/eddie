//! Fast and well-tested implementations of edit distance/string similarity metrics:
//! - [Levenshtein][1],
//! - [Damerau-Levenshtein][2],
//! - [Hamming][3],
//! - [Jaro][4],
//! - [Jaro-Winkler][5].
//!
//! [1]: struct.Levenshtein.html
//! [2]: struct.DamerauLevenshtein.html
//! [3]: struct.Hamming.html
//! [4]: struct.Jaro.html
//! [5]: struct.JaroWinkler.html
//!
//!
//! # Installation
//!
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! eddie = "0.3"
//! ```
//!
//!
//! # Basic Usage
//!
//! Levenshtein:
//! ```rust
//! use eddie::Levenshtein;
//! let lev = Levenshtein::new();
//! let dist = lev.distance("martha", "marhta");
//! assert_eq!(dist, 2);
//! ```
//!
//! Damerau-Levenshtein:
//! ```rust
//! use eddie::DamerauLevenshtein;
//! let damlev = DamerauLevenshtein::new();
//! let dist = damlev.distance("martha", "marhta");
//! assert_eq!(dist, 1);
//! ```
//!
//! Hamming:
//! ```rust
//! use eddie::Hamming;
//! let hamming = Hamming::new();
//! let dist = hamming.distance("martha", "marhta");
//! assert_eq!(dist, Some(2));
//! ```
//!
//! Jaro:
//! ```rust
//! use eddie::Jaro;
//! let jaro = Jaro::new();
//! let sim = jaro.similarity("martha", "marhta");
//! assert!((sim - 0.94).abs() < 0.01);
//! ```
//!
//! Jaro-Winkler:
//! ```rust
//! use eddie::JaroWinkler;
//! let jarwin = JaroWinkler::new();
//! let sim = jarwin.similarity("martha", "marhta");
//! assert!((sim - 0.96).abs() < 0.01);
//! ```
//!
//!
//! # Complementary metrics
//!
//! The main metric methods are complemented with inverted and/or relative versions.
//! The naming convention across the crate is following:
//! - `distance` — a number of edits required to transform one string to the other;
//! - `rel_dist` — a distance between two strings, relative to string length (inversion of similarity);
//! - `similarity` — similarity between two strings (inversion of relative distance).
//!
//!
//! ## Strings vs slices
//!
//! The crate exposes two modules containing two sets of implementations:
//! - `eddie::str` for comparing UTF-8 encoded `&str` and `&String` values.
//!   Implementations are reexported in the root module.
//! - `eddie::slice` for comparing generic slices `&[T]`.
//!   Implementations in this module are significantly faster than those from `eddie::str`,
//!   but will produce incorrect results for UTF-8 and other variable width character encodings.
//!
//! Usage example:
//!
//! ```rust
//! use eddie::slice::Levenshtein;
//!
//! let lev = Levenshtein::new();
//! let dist = lev.distance(&[1, 2, 3, 4], &[1, 3, 2, 4]);
//! assert_eq!(dist, 2);
//! ```
//!
//! [6]: https://doc.rust-lang.org/std/primitive.char.html
//!
//!
//! # Performance
//!
//! At the moment Eddie has the fastest implementations among the alternatives from crates.io
//! that have Unicode support.
//!
//! For example, when comparing common english words you can expect
//! at least 1.5-2x speedup for any given algorithm except Hamming.
//!
//! For the detailed measurements tables see [Benchmarks][7] page.
//!
//! [7]: http://github.com/thaumant/eddie/tree/master/benchmarks.md

mod utils;

pub mod slice;
pub mod str;

pub use crate::str::Levenshtein;
pub use crate::str::DamerauLevenshtein;
pub use crate::str::Hamming;
pub use crate::str::Jaro;
pub use crate::str::JaroWinkler;
