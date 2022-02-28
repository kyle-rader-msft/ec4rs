#![doc = include_str!("../README.md")]
#![allow(dead_code)]
#![deny(missing_docs)]

mod ecreader;
mod linereader;
mod properties;
pub mod property;
mod section;
#[cfg(test)]
mod tests;

/// The semantic version of the EditorConfig spec this library complies with.
pub const EC_VERSION: &str = "0.14.0";

pub use properties::Properties;
pub use section::Section;
