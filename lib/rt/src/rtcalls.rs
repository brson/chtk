#![allow(unused)]

use alloc::boxed::Box;
use alloc::str;
use core::slice;

use chtk_sys_api::Sys;
use chtk_sys_impl::SysImpl;

#[allow(non_camel_case_types)]
pub type c_char = i8;

pub struct Context;

#[no_mangle]
pub unsafe extern fn chtk_malloc(bytes: usize) -> *mut u8 {
    panic!()
}

#[no_mangle]
pub unsafe extern fn chtk_free(ptr: *mut u8) {
    panic!()
}

/// Aborts execution
///
/// The message may or may not be logged usefully,
/// depending on the environment.
///
/// # Fatal
///
/// `msg` must be valid UTF-8

#[no_mangle]
pub unsafe extern fn chtk_abort(msg: *const c_char, len: usize) -> ! {
    let msg = msg as *const u8;
    let msg = slice::from_raw_parts(msg, len);
    let msg = str::from_utf8(&*msg).expect("utf-8");
    SysImpl::abort(msg)
}

#[no_mangle]
pub unsafe extern fn chtk_get_context() -> *mut Context {
    panic!()
}

#[no_mangle]
pub unsafe extern fn chtk_free_context(ctx: *mut Context) {
    panic!()
}
