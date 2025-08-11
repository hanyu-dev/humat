#![no_main]

use libfuzzer_sys::fuzz_target;

#[derive(Debug)]
#[derive(arbitrary::Arbitrary)]
enum Input {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(usize),
    F32(f32),
    F64(f64),
}

fuzz_target!(|input: Input| {
    match input {
        Input::U8(value) => {
            let _ = humat::Formatter::SI.format(value).to_string();
            let _ = humat::Formatter::BINARY.format(value).to_string();
            let _ = humat::Formatter::CHINESE.format(value).to_string();
        }
        Input::U16(value) => {
            let _ = humat::Formatter::SI.format(value).to_string();
            let _ = humat::Formatter::BINARY.format(value).to_string();
            let _ = humat::Formatter::CHINESE.format(value).to_string();
        }
        Input::U32(value) => {
            let _ = humat::Formatter::SI.format(value).to_string();
            let _ = humat::Formatter::BINARY.format(value).to_string();
            let _ = humat::Formatter::CHINESE.format(value).to_string();
        }
        Input::U64(value) => {
            let _ = humat::Formatter::SI.format(value).to_string();
            let _ = humat::Formatter::BINARY.format(value).to_string();
            let _ = humat::Formatter::CHINESE.format(value).to_string();
        }
        Input::U128(value) => {
            let _ = humat::Formatter::SI.format(value).to_string();
            let _ = humat::Formatter::BINARY.format(value).to_string();
            let _ = humat::Formatter::CHINESE.format(value).to_string();
        }
        Input::Usize(value) => {
            let _ = humat::Formatter::SI.format(value).to_string();
            let _ = humat::Formatter::BINARY.format(value).to_string();
            let _ = humat::Formatter::CHINESE.format(value).to_string();
        }
        Input::I8(value) => {
            let _ = humat::Formatter::SI.format(value).to_string();
            let _ = humat::Formatter::BINARY.format(value).to_string();
            let _ = humat::Formatter::CHINESE.format(value).to_string();
        }
        Input::I16(value) => {
            let _ = humat::Formatter::SI.format(value).to_string();
            let _ = humat::Formatter::BINARY.format(value).to_string();
            let _ = humat::Formatter::CHINESE.format(value).to_string();
        }
        Input::I32(value) => {
            let _ = humat::Formatter::SI.format(value).to_string();
            let _ = humat::Formatter::BINARY.format(value).to_string();
            let _ = humat::Formatter::CHINESE.format(value).to_string();
        }
        Input::I64(value) => {
            let _ = humat::Formatter::SI.format(value).to_string();
            let _ = humat::Formatter::BINARY.format(value).to_string();
            let _ = humat::Formatter::CHINESE.format(value).to_string();
        }
        Input::I128(value) => {
            let _ = humat::Formatter::SI.format(value).to_string();
            let _ = humat::Formatter::BINARY.format(value).to_string();
            let _ = humat::Formatter::CHINESE.format(value).to_string();
        }
        Input::Isize(value) => {
            let _ = humat::Formatter::SI.format(value).to_string();
            let _ = humat::Formatter::BINARY.format(value).to_string();
            let _ = humat::Formatter::CHINESE.format(value).to_string();
        }
        Input::F32(value) => {
            let _ = humat::Formatter::SI.format(value).to_string();
            let _ = humat::Formatter::BINARY.format(value).to_string();
            let _ = humat::Formatter::CHINESE.format(value).to_string();
        }
        Input::F64(value) => {
            let _ = humat::Formatter::SI.format(value).to_string();
            let _ = humat::Formatter::BINARY.format(value).to_string();
            let _ = humat::Formatter::CHINESE.format(value).to_string();
        }
    }
});
