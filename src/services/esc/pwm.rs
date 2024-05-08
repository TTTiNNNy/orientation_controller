use core::{borrow::Borrow, marker::ConstParamTy};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::{Channel, Receiver}};
use static_cell::StaticCell;

use crate::bsp::glue::pwm::ModeInfo;
use super::api::EscApi;
use crate::bsp::glue::pwm::PwmInfo;

#[derive(ConstParamTy, PartialEq, Eq)]

struct Pwm <T: embedded_hal::pwm::SetDutyCycle + PwmInfo>{
    output: T,

}

impl <T: embedded_hal::pwm::SetDutyCycle + PwmInfo> Pwm <T>{
    fn new(pin_out: T) -> Self {
        Self{output: pin_out}
    }
}

impl <T: embedded_hal::pwm::SetDutyCycle + PwmInfo> EscApi for Pwm<T> {
    async fn set_power(&mut self, power_percent: u8) {
        self.output.set_duty_cycle(self.output.get_info().min_us + (power_percent as u16 * (self.output.get_info().max_us - self.output.get_info().min_us)) / 100 );
    }
}