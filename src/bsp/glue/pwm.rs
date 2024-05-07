use embassy_nrf::pwm::{Instance, Prescaler, SimplePwm};

struct Pwm<'a, T: Instance>{
    pwm: SimplePwm<'a, T>
}

impl <T: embassy_nrf::pwm::Instance> embedded_hal::pwm::ErrorType for Pwm<'_, T> {
    type Error = core::convert::Infallible;
}

impl <T: Instance>embedded_hal::pwm::SetDutyCycle for Pwm<'_, T>{
    fn max_duty_cycle(&self) -> u16 {
        todo!()
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        todo!()
    }
}