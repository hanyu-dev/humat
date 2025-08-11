#![doc = include_str!("../README.md")]
#![no_std]

pub mod format;
pub mod unit;

pub use format::Formatter;

// pub fn t() {
//     let formatted =
// Formatter::BINARY.format_u128(core::hint::black_box(1_234_567_890_u128));

//     core::hint::black_box(formatted);
// }
