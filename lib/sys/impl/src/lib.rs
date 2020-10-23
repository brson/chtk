#![no_std]

use chtk_sys_api::Sys;

pub struct SysImpl(imp::Sys);

impl Sys for SysImpl {
    fn abort(msg: &str) -> ! {
        imp::Sys::abort(msg)
    }
}

#[cfg(feature = "chtk_sys_ckb")]
mod imp {
    pub use chtk_sys_ckb::CkbSys as Sys;
}

#[cfg(feature = "chtk_sys_near")]
mod imp {
    pub use chtk_sys_near::NearSys as Sys;
}

#[cfg(feature = "chtk_sys_polkadot")]
mod imp {
    pub use chtk_sys_polkadot::PolkadotSys as Sys;
}

#[cfg(feature = "chtk_sys_solana")]
mod imp {
    pub use chtk_sys_solana::SolanaSys as Sys;
}
