#![feature(
    alloc_error_handler,
    lang_items,
    panic_info_message,
)]
#![no_std]
#![no_main]

mod syscall;
mod allocator;
mod internals;
#[macro_use] mod print;

use syscall::{
    sys_write,
    sys_mmap,
    sys_exit,
    STDOUT,
    STDERR,
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // TODO: pre/post tweaks and error handling might go here.
    // This function only serves to GOTO main() currently.
    main();
    sys_exit(0);
}

fn main() {
    // These macros demonstrate a functional allocator, at least in debug.
    // During release builds these strings (and the underlying need for
    // allocation) might get optimized away.

    // These macros are making direct syscalls. So that means if you see
    // anything print at all, that means most everything is working!

    // print macros always send to STDOUT
    println!( "STDOUT: this works?"    );
    print!(   "        this works?!\n" );

    // eprint macros always send to STDERR
    eprintln!("STDERR: works?!?"       );
    eprint!(  "        works?!?\n"     );

    sys_exit(0);
}
