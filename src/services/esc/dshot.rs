use core::{borrow::Borrow, marker::ConstParamTy};
use embassy_nrf::gpio::{AnyPin, Output, Pin};
use embassy_time::Timer;

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

    fn crc(payload: &u16) -> u8 { 
       ((*payload ^ (*payload >> 4) ^ (*payload >> 8)) & 0x0F) as u8
    }

    impl <const TIME: DShotTime> DShot<TIME>{
        fn build_package(&self, payload: u16) -> u16 { payload << 5 + crc(&payload) }
    }


impl <const TIME: DShotTime> EscApi for DShot<TIME>{
    async fn set_power(&mut self, power_percent: u8){
        const MAX_GRAD_SIZE: u16 = 2000;
        const BIT_NS_BASE_PERIOD_TIME: u16 = 6670;
        const BIT_NS_BASE_RAISE_TIME: u16 = 5000;

        let BIT_PERIOD = BIT_NS_BASE_PERIOD_TIME / TIME as u16;
        let RAISE_PERIOD = BIT_NS_BASE_PERIOD_TIME / TIME as u16;

        let msb = 1 << (u16::MAX - 1);
        
        let payload = power_percent as u16 * MAX_GRAD_SIZE / 100;
        let mut package = self.build_package(payload);

        while package != 0 {
            self.output.set_high();
            let sleep_period;
            if package & msb != 0{ 
                sleep_period = RAISE_PERIOD as u64;
                Timer::after_nanos(RAISE_PERIOD as u64).await;
            } else {
                sleep_period = RAISE_PERIOD as u64 / 2;
                Timer::after_nanos(RAISE_PERIOD as u64 / 2).await;
            }
            self.output.set_low();
            Timer::after_nanos(BIT_PERIOD as u64 - sleep_period).await;

            package <<= 1;
        }


    }
}