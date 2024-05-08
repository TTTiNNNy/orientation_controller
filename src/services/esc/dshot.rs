use core::{borrow::Borrow, marker::ConstParamTy};
use embassy_time::Timer;

use crate::services::esc::api::EscApi;

#[derive(ConstParamTy, PartialEq, Eq)]
enum Freq{
    d150 = 1,
    d300,
    d600,
    d1200
}

struct DShot <const time: Freq, T: embedded_hal::digital::OutputPin>{
    output: T
}

    fn crc(payload: &u16) -> u8 { 
       ((*payload ^ (*payload >> 4) ^ (*payload >> 8)) & 0x0F) as u8
    }

    impl <const TIME: Freq, T: embedded_hal::digital::OutputPin> DShot<TIME, T>{
        fn build_package(&self, payload: u16) -> u16 { payload << 5 + crc(&payload) }
    }

impl <const TIME: Freq, T:  embedded_hal::digital::OutputPin> EscApi for DShot<TIME, T>{
    async fn set_power(&mut self, power_percent: u8){
        const MAX_GRAD_SIZE: u16 = 2000;
        const BIT_NS_BASE_PERIOD_TIME: u16 = 6670;
        const BIT_NS_BASE_RAISE_TIME: u16 = 5000;

        let bit_period = BIT_NS_BASE_PERIOD_TIME / TIME as u16;
        let raise_period = BIT_NS_BASE_RAISE_TIME / TIME as u16;

        let msb = 1 << (u16::MAX - 1);
        
        let payload = power_percent as u16 * MAX_GRAD_SIZE / 100;
        let mut package = self.build_package(payload);

        while package != 0 {
            self.output.set_high();
            let sleep_period;
            if package & msb != 0{ 
                sleep_period = raise_period as u64;
                Timer::after_nanos(raise_period as u64).await;
            } else {
                sleep_period = raise_period as u64 / 2;
                Timer::after_nanos(raise_period as u64 / 2).await;
            }
            self.output.set_low();
            Timer::after_nanos(bit_period as u64 - sleep_period).await;

            package <<= 1;
        }


    }
}