use embassy_nrf::pwm::{Instance, Prescaler, SimplePwm};
use core::{borrow::Borrow, marker::ConstParamTy};

struct Pwm<'a, T: Instance>{
    channel: usize,
    pwm: SimplePwm<'a, T>,
    info: ModeInfo
}
enum Mode{
    Pwm,
    OnShot125,
    OnShot42,
    Multishot
}

#[derive(ConstParamTy, PartialEq, Eq, Clone)]
pub struct ModeInfo{
    pub min_us: u16,
    pub max_us: u16,
    pub freq_hz: u16
}

impl ModeInfo {
    pub fn new(mode: Mode) -> Self {
        match mode {
            Mode::Pwm => ModeInfo{freq_hz: 490, min_us: 1000, max_us: 2000},
            Mode::OnShot125 => ModeInfo{freq_hz: 3900, min_us: 125, max_us: 250},
            Mode::OnShot42 => ModeInfo{freq_hz: 11900, min_us: 42, max_us: 84},
            Mode::Multishot => ModeInfo{freq_hz: 31900, min_us: 5, max_us: 25},
        }
    }
}

pub trait PwmInfo{
    fn get_info(&self) -> ModeInfo;
}

impl <T: Instance>PwmInfo for Pwm<'_, T>{
    fn get_info(&self) -> ModeInfo {
        self.info.clone()
    }
}


impl <T: embassy_nrf::pwm::Instance> embedded_hal::pwm::ErrorType for Pwm<'_, T> {
    type Error = core::convert::Infallible;
}

impl <T: Instance>embedded_hal::pwm::SetDutyCycle for Pwm<'_, T>{
    fn max_duty_cycle(&self) -> u16 {
        self.pwm.max_duty()
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.pwm.set_duty(self.channel, duty);
        Result::Ok(())
    }
}
