#![no_std]
#![allow(internal_features, non_upper_case_globals)]
#![feature(rustc_attrs)]

extern crate alloc;

use alloc::vec;
use alloc::{
    alloc::{GlobalAlloc, Layout},
    boxed::Box,
    ffi::CString,
    vec::Vec,
};
use core::panic::PanicInfo;
use libc::{aligned_alloc, c_void, exit, free, puts};

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

fn print_exit(message: &'static str, code: i32) -> ! {
    if let Ok(message) = CString::new(message) {
        unsafe { puts(message.as_ptr()) };
    }
    unsafe { exit(code) }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(message) = info.message().as_str() {
        print_exit(message, 1)
    } else {
        print_exit("internal runtime error", 2)
    }
}

#[rustc_std_internal_symbol]
fn __rust_alloc_error_handler(_size: usize, _align: usize) -> ! {
    print_exit("memory allocation failed", 3);
}

#[rustc_std_internal_symbol]
static __rust_no_alloc_shim_is_unstable: u8 = 0;

pub struct Foo {
    numbers: Vec<i32>,
}

#[no_mangle]
pub extern "C" fn new_foo() -> Box<Foo> {
    Box::new(Foo {
        numbers: vec![1, 2, 3],
    })
}

#[no_mangle]
pub extern "C" fn sum_foo(foo: &Foo) -> i32 {
    foo.numbers.iter().sum()
}

#[no_mangle]
pub extern "C" fn free_foo(_foo: Box<Foo>) {}
