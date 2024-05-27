use core::{borrow::Borrow, marker::ConstParamTy};
use embassy_nrf::pwm::{Instance, Prescaler, SimplePwm};
use crate::services::esc::pwm::{ModeInfo, PwmInfo};

pub struct Pwm<'a, T: Instance> {
    pub channel: usize,
    pub pwm: SimplePwm<'a, T>,
    pub info: ModeInfo,
}

impl<T: Instance> PwmInfo for Pwm<'_, T> {
    fn get_info(&self) -> ModeInfo {
        self.info.clone()
    }
}

impl<T: embassy_nrf::pwm::Instance> embedded_hal::pwm::ErrorType for Pwm<'_, T> {
    type Error = core::convert::Infallible;
}

impl<T: Instance> embedded_hal::pwm::SetDutyCycle for Pwm<'_, T> {
    fn max_duty_cycle(&self) -> u16 {
        self.pwm.max_duty()
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.pwm.set_duty(self.channel, duty);
        Result::Ok(())
    }
}
