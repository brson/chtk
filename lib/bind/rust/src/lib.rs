#![no_std]
#![allow(unused)]

extern crate alloc;

use alloc::vec::Vec;

use chtk_rt::rtcalls as rt;

pub fn abort(msg: &str) -> ! {
    let buf = c_string(msg);
    rt::chtk_abort(buf.as_ptr());
}

pub struct Context;

impl Context {
    fn get() -> Context {
        panic!()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        panic!()
    }
}

fn c_string(s: &str) -> Vec<rt::c_char> {
    panic!()
}
