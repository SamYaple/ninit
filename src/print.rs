extern crate alloc;
use alloc::string::ToString;
use core::fmt;
use super::{
    STDOUT,
    STDERR,
    sys_write,
};

// //////////////////////////////
// TODO: terrible print macros, should be refactored
// //////////////////////////////
pub fn print(args: fmt::Arguments) {
    let s = args.to_string();
    sys_write(STDOUT, &s).unwrap();
}

pub fn eprint(args: fmt::Arguments) {
    let s = args.to_string();
    sys_write(STDERR, &s).unwrap();
}

macro_rules! print {
    ($($arg:tt)*) => (crate::print::print(format_args!($($arg)*)));
}

macro_rules! eprint {
    ($($arg:tt)*) => (crate::print::eprint(format_args!($($arg)*)));
}

pub fn println(args: fmt::Arguments) {
    let s = args.to_string();
    print!{"{}\n", s};
}

pub fn eprintln(args: fmt::Arguments) {
    let s = args.to_string();
    eprint!{"{}\n", s};
}

macro_rules! println {
    ($($arg:tt)*) => (crate::print::println(format_args!($($arg)*)));
}

macro_rules! eprintln {
    ($($arg:tt)*) => (crate::print::eprintln(format_args!($($arg)*)));
}

