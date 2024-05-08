use crate::services::esc::api::EscApi;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::Receiver};
use nalgebra::Vector3;
use pid::Pid;

#[embassy_executor::task]
pub async fn pid(
    mut esc: impl EscApi + 'static,
    control_degree_receiver: Receiver<'static, NoopRawMutex, Vector3<f32>, 1>,
    xpyp: Sender<'static, NoopRawMutex, LedState, 1>,
    xpyn: Sender<'static, NoopRawMutex, LedState, 1>,
    xnyn: Sender<'static, NoopRawMutex, LedState, 1>,
    xnyp: Sender<'static, NoopRawMutex, LedState, 1>,
) {
    let mut pid: Pid<f32> = Pid::new(0.0, 3600.0);

    
    loop {}
}
