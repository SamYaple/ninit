use core::alloc::{
    GlobalAlloc,
    Layout,
};

use super::{
    sys_exit,
    sys_mmap,
    //sys_munmap,
};

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = match sys_mmap(layout.size()) {
            Ok(addr) => addr,
            Err(_)   => sys_exit(41),
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

#[global_allocator]
static THISCANBENAMEDANYTHING: Allocator = Allocator;

