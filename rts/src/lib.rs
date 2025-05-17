#![no_std]
#![allow(internal_features)]
#![feature(rustc_attrs, linkage)]

extern crate alloc;

use alloc::vec;
use alloc::{
    alloc::{GlobalAlloc, Layout},
    boxed::Box,
    vec::Vec,
};
use libc::{abort, aligned_alloc, c_void, free};

#[allow(unused_imports)]
use core::panic::PanicInfo;

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

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { abort() }
}

#[rustc_std_internal_symbol]
#[linkage = "weak"]
fn __rust_alloc_error_handler(_size: usize, _align: usize) -> ! {
    unsafe { abort() }
}

#[rustc_std_internal_symbol]
#[linkage = "weak"]
#[allow(non_upper_case_globals)]
static __rust_alloc_error_handler_should_panic: u8 = 0;

#[rustc_std_internal_symbol]
#[linkage = "weak"]
#[allow(non_upper_case_globals)]
static __rust_no_alloc_shim_is_unstable: u8 = 0;

pub struct Foo {
    numbers: Vec<i32>,
}

impl Foo {
    #[no_mangle]
    pub extern "C" fn new_foo() -> Box<Foo> {
        Box::new(Foo {
            numbers: vec![1, 2, 3],
        })
    }

    #[no_mangle]
    pub extern "C" fn sum_foo(self: &Foo) -> i32 {
        self.numbers.iter().sum()
    }

    #[no_mangle]
    pub extern "C" fn free_foo(_foo: Box<Foo>) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rts() {
        let foo = Foo::new_foo();
        let result = foo.sum_foo();
        assert_eq!(result, 6);
        Foo::free_foo(foo);
    }
}
