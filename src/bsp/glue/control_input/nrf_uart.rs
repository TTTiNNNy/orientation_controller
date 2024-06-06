
use crate::services::input::{IInputControl, InputControlState};
use defmt::warn;
use embassy_nrf::{bind_interrupts, peripherals, uarte::{self, Instance}};

impl <'d, T: Instance> IInputControl for uarte::Uarte<'d, T>{
    async fn get_control_command(&mut self) -> Option<InputControlState> {
        let mut addr = [0];
        self.read(&mut addr).await;
        match addr[0] {
            InputControlState::POWER_VAL => { let mut data: [u8; 1] = [0]; self.read(&mut data).await; Some(InputControlState::Power(data[0])) }
            InputControlState::YAY_DEGREE_PER_SEC_VAL => { let mut data: [u8; 1] = [0]; self.read(&mut data).await; Some(InputControlState::YayDegreePerSec(data[0])) }
            InputControlState::CHANGE_ANGLE_VAL => { let mut data: [u8; 2] = [0, 0]; self.read(&mut data).await; Some(InputControlState::ChangeAngle{roll: data[0], pitch: data[1]}) }

            _ => { warn!("incorrect uart input data"); None }
        }
    }
}
