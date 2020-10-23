#![no_std]
#![allow(unused)]

use chtk_sys_api::Sys;

pub struct NearSys;

impl Sys for NearSys {
    fn abort(msg: &str) -> ! {
        panic!()
    }
}
