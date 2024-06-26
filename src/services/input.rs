use crate::services::esc::api::EscApi;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::{Receiver, Sender}};

pub enum InputControlState{
    ChangeAngle{roll: u8, pitch: u8},
    YayDegreePerSec(u8),
    Power(u8)
}

impl InputControlState{
    pub const CHANGE_ANGLE_VAL: u8 = 0;
    pub const YAY_DEGREE_PER_SEC_VAL: u8 = 1;
    pub const POWER_VAL: u8 = 2;

}

pub trait IInputControl{
    async fn get_control_command(&mut self) -> Option<InputControlState>;
}

#[embassy_executor::task]
pub async fn esc(mut power_input: Sender<'static, NoopRawMutex, u8, 1>, receiver: Receiver<'static, NoopRawMutex, u8, 1>) {
    loop {

    }
}
