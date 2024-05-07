use core::{borrow::Borrow, marker::ConstParamTy};

use super::api::EscApi;

#[derive(ConstParamTy, PartialEq, Eq)]
enum Mode{
    Pwm,
    OnShot125,
    OnShot42,
    Multishot
}

#[derive(ConstParamTy, PartialEq, Eq)]
struct ModeInfo{
    min_us: u16,
    max_us: u16,
    freq_hz: u16
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

struct Pwm <T: embedded_hal::pwm::SetDutyCycle>{
    info: ModeInfo,
    output: T
}

impl <T: embedded_hal::pwm::SetDutyCycle> Pwm <T>{
    fn new(mode_info: ModeInfo, pin_out: T) -> Self {
        Self{info: mode_info, output: pin_out}
    }
}

impl <T: embedded_hal::pwm::SetDutyCycle> EscApi for Pwm<T> {
    async fn set_power(&mut self, power_percent: u8) {
        self.output.set_duty_cycle(self.info.min_us + (power_percent as u16 * (self.info.max_us - self.info.min_us)) / 100 );
    }
}
