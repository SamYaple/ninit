#![feature(
    alloc_error_handler,
    lang_items,
    panic_info_message,
)]
#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::ToString;

use core::{
    fmt,
    panic::PanicInfo,
    arch::asm,
    alloc::{
        GlobalAlloc,
        Layout,
    },
};

// /////////////////////////////////////////////////////
// Glue code because our target is `x86_64-unknown-none`
// /////////////////////////////////////////////////////
// We exit with different exit_codes from all of these functions. We don't
// handle any errors, we do build and link correctly. Exit code debugging, fun!
#[global_allocator]
static G: Allocator = Allocator;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    sys_exit(42);
}

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {
    sys_exit(43);
}

#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    sys_exit(44);
}

#[alloc_error_handler]
fn alloc_error(_: core::alloc::Layout) -> ! {
    sys_exit(45);
}

// //////////////////////////////////////
// Memory Allocator using mmap and munmap
// //////////////////////////////////////
pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = match sys_mmap(layout.size()) {
            Ok(addr) => addr,
            Err(_)   => sys_exit(43),
        };
        ret
    }

    // We do need to _define_ a dealloc function, but we don't actually need to
    // deallocate anything. The OS we run this against will cleanup for us when
    // we exit.
    // Yay memory leaks!
    unsafe fn dealloc(&self, _: *mut u8, _: Layout) { }
    //unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    //    let len = layout.size();
    //    asm!(
    //        "syscall",
    //        in("rax") SYS_MUNMAP,
    //        in("rdi") ptr,
    //        in("rsi") len,

    //        lateout("rcx") _,
    //        lateout("r11") _,
    //    );
    //}
}

// //////////////////////////////
// TODO: terrible print macros, should be refactored
// //////////////////////////////
macro_rules! print {
    ($($arg:tt)*) => (print(format_args!($($arg)*)));
}

macro_rules! println {
    ($($arg:tt)*) => (println(format_args!($($arg)*)));
}

macro_rules! eprint {
    ($($arg:tt)*) => (eprint(format_args!($($arg)*)));
}

macro_rules! eprintln {
    ($($arg:tt)*) => (eprintln(format_args!($($arg)*)));
}

fn print(args: fmt::Arguments) {
    let s = args.to_string();
    sys_write(STDOUT, &s).unwrap();
}

fn println(args: fmt::Arguments) {
    let s = args.to_string();
    print!{"{}\n", s};
}

fn eprint(args: fmt::Arguments) {
    let s = args.to_string();
    sys_write(STDERR, &s).unwrap();
}

fn eprintln(args: fmt::Arguments) {
    let s = args.to_string();
    eprint!{"{}\n", s};
}

// ///////////////////////
// BEGIN CODE I CARE ABOUT
//   anything above this comment block is machinery
//   that was required for the code below to be useful
// ///////////////////////

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // This function only serves to GOTO main() currently
    main();
    // TODO: maybe I should go back to calling sys_exit(0) from here
    unreachable!("main() should not have returned, you should call sys_exit()");
}

// x86_64 only...
const SYS_WRITE:  u64 =   1;
const SYS_MMAP:   u64 =   9;
//const SYS_MUNMAP: u64 =  11;
const SYS_EXIT:   u64 =  60;

//const STDIN:  u64 = 0;
const STDOUT: u64 = 1;
const STDERR: u64 = 2;

#[derive(Debug)]
struct Error(i64);

fn round_up(value: usize, increment: usize) -> usize {
    if value == 0 {
        return 0;
    }
    increment * ((value - 1) / increment + 1)
}


fn sys_mmap(min_size: usize) -> Result<*mut u8, Error> {
    // round size to 4096 bytes
    let size = round_up(min_size, 4096);

    let ret: i64;
    unsafe {
        asm!(
            "syscall",
            inout("rax") SYS_MMAP => ret,
            in("rdi") core::ptr::null_mut::<u64>(), // allocate any space
            in("rsi") size,
            in("rdx") 0b0000_0011, // prot (read/write)
            in("r10") 0b0010_0001, // flags ANON | SHARED
            in("r8")  !0u64,       // fd is -1 because of MAP_ANON
            in("r9")  0,           // offset

            lateout("rcx")   _,
            lateout("r11")   _,
        );
    }
    if ret < 0 {
        return Err(Error(-ret));
    }
    Ok(ret as *mut u8)
}

fn sys_write(fd: u64, text: &str) -> Result<(), Error>{
    let len = text.len();
    let ret: i64;
    unsafe {
        asm!(
            "syscall",
            inout("rax")   SYS_WRITE => ret,
            in("rdi")      fd,
            in("rsi")      text.as_ptr(),
            in("rdx")      len,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }
    if ret < 0 {
        return Err(Error(-ret));
    }
    Ok(())
}

fn sys_exit(exit_code: u8) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax")      SYS_EXIT,
            in("rdi")      exit_code as u64,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }
    unreachable!("We called SYS_EXIT; The OS should have killed us by now");
}

fn main() {
    println!( "STDOUT: this works?"    );
    print!(   "STDOUT: this works?!\n" );
    eprintln!("STDERR: works?!?"       );
    eprint!(  "STDERR: works?!?\n"     );

    sys_exit(0);
}
