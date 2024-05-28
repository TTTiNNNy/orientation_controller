use crate::services::esc::api::EscApi;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::Receiver};

#[embassy_executor::task]
pub async fn esc(mut esc: impl EscApi + 'static, receiver: Receiver<'static, NoopRawMutex, u8, 1>) {
    loop {
        //esc.set_power(receiver.receive().await);
    }
}
