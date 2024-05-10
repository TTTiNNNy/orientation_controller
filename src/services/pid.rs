use core::marker::ConstParamTy;

use crate::services::esc::api::EscApi;
use embassy_sync::{
    blocking_mutex::raw::NoopRawMutex,
    channel::{Receiver, Sender},
};
use nalgebra::Vector3;
use pid::Pid;

#[derive(ConstParamTy, PartialEq, Eq)]

enum QuadroRotors {
    Xpyp,
    Xpyn,
    Xnyn,
    Xnyp,
}

enum CopterType {
    Three,
    Quadro(QuadroRotors),
    Octa,
    Hex,
}

async fn quadro_pid_calc(
    pid: Pid<f32>,
    motor_type: QuadroRotors,
    mut esc: impl EscApi + 'static,
    control_degree_receiver: Receiver<'static, NoopRawMutex, Vector3<f32>, 1>,
    orientation_receiver: Receiver<'static, NoopRawMutex, Vector3<f32>, 1>,
    xxyy: Sender<'static, NoopRawMutex, u8, 1>,
) {
    loop {
        0

    }
}

#[embassy_executor::task]
pub async fn pid(
    deviceType: CopterType,
    mut esc: impl EscApi + 'static,
    control_degree_receiver: Receiver<'static, NoopRawMutex, Vector3<f32>, 1>,
    orientation_receiver: Receiver<'static, NoopRawMutex, Vector3<f32>, 1>,
    xxyy: Sender<'static, NoopRawMutex, u8, 1>,
) {
    let mut pid: Pid<f32> = Pid::new(0.0, 3600.0);

    match deviceType {
    CopterType::Quadro(motor_type) => quadro_pid_calc(pid, motor_type, esc, control_degree_receiver, orientation_receiver, xxyy).await,
    _ => todo!()
    }
        
    }


