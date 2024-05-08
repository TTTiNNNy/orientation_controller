
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::Receiver};
use crate::services::esc::api::EscApi;

#[embassy_executor::task]
pub async fn orient_calc(mut esc: impl EscApi+ 'static, receiver: Receiver<'static, NoopRawMutex, u8, 1>) {
    loop {
        esc.set_power(receiver.receive().await);        
    }
}