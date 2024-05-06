use core::{borrow::Borrow, marker::ConstParamTy};
use embassy_nrf::gpio::{AnyPin, Output, Pin};

use crate::services::esc::api::EscApi;

#[derive(ConstParamTy, PartialEq, Eq)]
enum DShotTime{
    t150 = 1,
    t300,
    t600,
    t1200
}

struct DShot <const time: DShotTime>{
    output: Output<'static, AnyPin>
}

impl <const TIME: DShotTime> EscApi for DShot<TIME>{
    async fn set_power(&mut self, power_percent: u8){
        const MAX_GRAD_SIZE: u16 = 2000;
        const BIT_NS_BASE_TIME: u16 = 6670;
        
        self.output.set_high();

    }
}