use core::{borrow::Borrow, marker::ConstParamTy};
use embassy_nrf::pwm::{Instance, Prescaler, SimplePwm};
use crate::services::esc::{api::EscApi, pwm::{ModeInfo}};

pub struct Pwm<'a, T: Instance> {
    pub pwm: SimplePwm<'a, T>,
    pub info: ModeInfo,
}

impl <T: Instance>EscApi for Pwm<'_, T>{
    async fn set_power(&mut self, channel: u8, power_percent: u8) {
        let k = (self.pwm.max_duty() / self.info.freq_hz) as u32;
        let max_duty = (self.info.freq_hz as u32 * self.info.max_us as u32 * k / 1_000_000) as u16;
        let min_duty =  (self.info.freq_hz as u32 * self.info.min_us as u32 * k / 1_000_000) as u16;
        let duty: u16 = max_duty - min_duty;
        self.pwm.set_duty(channel.into(), duty * power_percent as u16 / 100);
    }
}
