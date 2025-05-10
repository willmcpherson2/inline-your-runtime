#![no_std]
#![allow(internal_features, non_upper_case_globals)]
#![feature(linkage, rustc_attrs)]

extern crate alloc;

use alloc::{
    alloc::{GlobalAlloc, Layout},
    boxed::Box,
};
use core::panic::PanicInfo;
use libc::{aligned_alloc, c_void, free};

struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { aligned_alloc(layout.align(), layout.size()) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { free(ptr as *mut c_void) };
    }
}

#[global_allocator]
static GLOBAL: Allocator = Allocator;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[rustc_std_internal_symbol]
#[linkage = "weak"]
static __rust_no_alloc_shim_is_unstable: u8 = 0;

#[rustc_std_internal_symbol]
#[linkage = "weak"]
fn __rust_alloc_error_handler(size: usize, align: usize) {
    panic!("allocation failed: size: {}, align: {}", size, align);
}

#[no_mangle]
pub extern "C" fn foo() -> Box<i32> {
    Box::new(42)
}
