use core::{borrow::Borrow, marker::ConstParamTy};
use embassy_sync::{
    blocking_mutex::raw::NoopRawMutex,
    channel::{Channel, Receiver},
};
use static_cell::StaticCell;

use super::api::EscApi;

#[derive(ConstParamTy, PartialEq, Eq, Clone)]
pub struct ModeInfo {
    pub min_us: u16,
    pub max_us: u16,
    pub freq_hz: u16,
}


impl ModeInfo {
    pub const PWM: ModeInfo =  ModeInfo {
        freq_hz: 490,
        min_us: 1000,
        max_us: 2000,
    };

    pub const ONSHOT125: ModeInfo =  ModeInfo {
        freq_hz: 3900,
        min_us: 125,
        max_us: 250,
    };
    
    pub const ONSHOT42: ModeInfo = ModeInfo {
        freq_hz: 11900,
        min_us: 42,
        max_us: 84,
    };

    pub const MULTISHOT: ModeInfo = ModeInfo {
        freq_hz: 31900,
        min_us: 5,
        max_us: 25,
    };
}