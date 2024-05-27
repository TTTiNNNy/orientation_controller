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

    pub const ON_SHOT125: ModeInfo =  ModeInfo {
        freq_hz: 3900,
        min_us: 125,
        max_us: 250,
    };
    
    pub const ON_SHOT42: ModeInfo = ModeInfo {
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

pub trait PwmInfo {
    fn get_info(&self) -> ModeInfo;
}




#[derive(ConstParamTy, PartialEq, Eq)]
pub struct Pwm<T: embedded_hal::pwm::SetDutyCycle + PwmInfo> {
    output: T,
}

impl<T: embedded_hal::pwm::SetDutyCycle + PwmInfo> Pwm<T> {
    pub fn new(pin_out: T) -> Self {
        Self { output: pin_out }
    }
}

impl<T: embedded_hal::pwm::SetDutyCycle + PwmInfo> EscApi for Pwm<T> {
    async fn set_power(&mut self, power_percent: u8) {
        self.output.set_duty_cycle(
            self.output.get_info().min_us
                + (power_percent as u16
                    * (self.output.get_info().max_us - self.output.get_info().min_us))
                    / 100,
        );
    }
}
