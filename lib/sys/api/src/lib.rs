#![no_std]

pub trait Sys: Send + Unpin + 'static {
    fn abort(msg: &str) -> !;
}
