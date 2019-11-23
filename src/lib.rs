//! Fast and well-tested implementations of edit distance/string similarity
//! metrics:
//! - Levenshtein,
//! - Damerau-Levenshtein,
//! - Hamming,
//! - Jaro,
//! - Jaro-Winkler.
//!
//!
//! # Usage
//!
//! Cargo.toml:
//! ```toml
//! [dependencies]
//! eddie = "0.1"
//!  ```
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
//! # Performance
//!
//! At the moment Eddie has the fastest implementations among the alternatives from crates.io
//! that have Unicode support. For any given algorithm you can expect at least 1.5-2x speedup
//! for typical use cases.
//!
//! For the detailed measurements tables see [GitHub page][1].
//!
//! [1]: https://github.com/thaumant/eddie#performance

mod utils;
mod leven;
mod damlev;
mod hamming;
mod jaro;
mod jarwin;

pub use crate::leven::Levenshtein;
pub use crate::damlev::DamerauLevenshtein;
pub use crate::hamming::Hamming;
pub use crate::jaro::Jaro;
pub use crate::jarwin::JaroWinkler;
