#![feature(
    alloc_error_handler,
    lang_items,
    panic_info_message,
)]
#![no_std]
#![no_main]

use core::arch::asm;

mod allocator;
mod internals;
#[macro_use] mod print;

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
    print!(   "        this works?!\n" );
    eprintln!("STDERR: works?!?"       );
    eprint!(  "        works?!?\n"     );

    sys_exit(0);
}
