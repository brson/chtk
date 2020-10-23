#![allow(unused)]

use alloc::boxed::Box;

use chtk_sys_api::Sys;
use chtk_sys_impl::SysImpl;

#[allow(non_camel_case_types)]
pub type c_char = i8;

pub struct Context;

#[no_mangle]
pub extern fn chtk_malloc(bytes: usize) -> *mut u8 {
    panic!()
}

#[no_mangle]
pub extern fn chtk_free(ptr: *mut u8) {
    panic!()
}

#[no_mangle]
pub extern fn chtk_abort(msg: *const c_char) -> ! {
    panic!()
}

#[no_mangle]
pub extern fn chtk_get_context() -> *mut Context {
    panic!()
}

#[no_mangle]
pub extern fn chtk_free_context() {
    panic!()
}
