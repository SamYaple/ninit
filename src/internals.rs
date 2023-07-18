use core::{
    panic::PanicInfo,
    alloc::Layout,
};

use super::sys_exit;

// /////////////////////////////////////////////////////
// Glue code because our target is `x86_64-unknown-none`
// /////////////////////////////////////////////////////
// We exit with different exit_codes from all of these functions. We don't
// handle any errors, we do build and link correctly. Exit code debugging, fun!
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    //eprintln!{"Print macros are available here, but not the other functions"};
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
fn alloc_error(_: Layout) -> ! {
    sys_exit(45);
}
