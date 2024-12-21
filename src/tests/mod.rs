mod build;
#[cfg(feature = "test-terminator")]
mod terminator;
#[doc(hidden)]
pub mod r#test_macro;

pub use build::{build, BuildError};
#[cfg(feature = "test-terminator")]
pub use terminator::{TestFailure, TestTerminator};
