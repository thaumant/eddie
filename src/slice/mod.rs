//! Distance/similarity metric implementations for generic slices `&[T]`.
//!
//! Implementations in this module are significantly faster than those from `eddie::str`,
//! but will produce incorrect results for UTF-8 and other variable width character encodings.
//!
//!
//! # Basic Usage
//!
//! Levenshtein:
//! ```rust
//! use eddie::slice::Levenshtein;
//! let lev = Levenshtein::new();
//! let dist = lev.distance(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
//! assert_eq!(dist, 2);
//! ```
//!
//! Damerau-Levenshtein:
//! ```rust
//! use eddie::slice::DamerauLevenshtein;
//! let damlev = DamerauLevenshtein::new();
//! let dist = damlev.distance(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
//! assert_eq!(dist, 1);
//! ```
//!
//! Hamming:
//! ```rust
//! use eddie::slice::Hamming;
//! let hamming = Hamming::new();
//! let dist = hamming.distance(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
//! assert_eq!(dist, Some(2));
//! ```
//!
//! Jaro:
//! ```rust
//! use eddie::slice::Jaro;
//! let jaro = Jaro::new();
//! let sim = jaro.similarity(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
//! dbg!(sim);
//! assert!((sim - 0.93).abs() < 0.01);
//! ```
//!
//! Jaro-Winkler:
//! ```rust
//! use eddie::slice::JaroWinkler;
//! let jarwin = JaroWinkler::new();
//! let sim = jarwin.similarity(&[1, 2, 3, 4, 5], &[1, 3, 2, 4, 5]);
//! dbg!(sim);
//! assert!((sim - 0.93).abs() < 0.01);
//! ```


mod leven;
mod damlev;
mod hamming;
mod jaro;
mod jarwin;
mod matrix;

pub use leven::Levenshtein;
pub use damlev::DamerauLevenshtein;
pub use hamming::Hamming;
pub use jaro::Jaro;
pub use jarwin::JaroWinkler;
