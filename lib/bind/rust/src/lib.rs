#![no_std]
#![allow(unused)]

extern crate alloc;

use alloc::vec::Vec;

use chtk_rt::rtcalls as rt;

pub fn abort(msg: &str) -> ! {
    let ptr = msg.as_ptr() as *const rt::c_char;
    let len  = msg.len();
    unsafe {
        rt::chtk_abort(ptr, len);
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
