//! Distance/similarity metric implementations for
//! UTF-8 encoded `&str` and `&String` values.
//!
//! All implementations here are reexported in the root module.
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


mod leven;
mod damlev;
mod hamming;
mod jaro;
mod jarwin;

pub use leven::Levenshtein;
pub use damlev::DamerauLevenshtein;
pub use hamming::Hamming;
pub use jaro::Jaro;
pub use jarwin::JaroWinkler;
