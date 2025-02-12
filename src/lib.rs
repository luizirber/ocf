#![doc = include_str!("../README.md")]

/* declare mod */
pub mod basic;
pub mod error;
pub mod level;
pub mod seek;
pub mod seeksend;
pub mod send;
pub(crate) mod utils;

/* reexport for convinent usage of niffler */
pub use crate::basic::compression::Format;
pub use crate::basic::*;
pub use crate::error::Error;
pub use crate::level::Level;
