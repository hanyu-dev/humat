#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod format;
pub mod unit;

pub use format::Formatter;
