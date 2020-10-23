#![no_std]
#![allow(unused)]

extern crate alloc;

use alloc::vec::Vec;

use chtk_rt::rtcalls as rt;

pub fn abort(msg: &str) -> ! {
    let buf = c_string(msg);
    unsafe {
        rt::chtk_abort(buf.as_ptr());
    }
}

pub struct Context(*mut rt::Context);

impl Context {
    fn get() -> Context {
        let ctx = unsafe { rt::chtk_get_context() };
        assert!(!ctx.is_null());
        Context(ctx)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { rt::chtk_free_context(self.0) }
    }
}

fn c_string(s: &str) -> Vec<rt::c_char> {
    panic!()
}
