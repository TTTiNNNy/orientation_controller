use core::marker::ConstParamTy;

use crate::{services::esc::api::EscApi, utils::comp_fltr::compliment_filter};
use embassy_sync::{
    blocking_mutex::raw::NoopRawMutex,
    channel::{Receiver, Sender},
};
use embassy_time::Timer;
use nalgebra::Vector3;
use pid::Pid;
use crate::utils::comp_fltr;

// #[derive(ConstParamTy, PartialEq, Eq)]
// enum QuadroRotors<T: EscApi> {
//     Xpyp (T),
//     Xpyn,
//     Xnyn,
//     Xnyp,
// }

enum CopterType{
    Three,
    Quadro,
    Octa,
    Hex,
}




#[embassy_executor::task]
pub async fn pid(
    
    deviceType: CopterType,
    mut esc: impl EscApi + 'static,
    control_angles_receiver: Receiver<'static, NoopRawMutex, Vector3<f32>, 1>,
    orientation_angles_receiver: Receiver<'static, NoopRawMutex, Vector3<f32>, 1>,
    power_receiver: Receiver<'static, NoopRawMutex, u8, 1>,

    xxyy: impl EscApi + Clone + 'static,
) {
    match deviceType {
    CopterType::Quadro => {
        let mut pid_x: Pid<f32> = Pid::new(0.0, 100.0);
        pid_x.p(10.0, 50.0);
        pid_x.i(5.0, 35.0);
        pid_x.d(1.0, 10.0);

        let mut pid_y: Pid<f32> = Pid::new(0.0, 100.0);
        pid_y.p(10.0, 50.0);
        pid_x.i(5.0, 35.0);
        pid_x.d(1.0, 10.0);

        let mut xpyp = xxyy.clone();
        let mut xpyn = xxyy.clone();
        let mut xnyn = xxyy.clone();
        let mut xnyp = xxyy.clone();

        let mut control_angles = control_angles_receiver.receive().await;
        let mut orientation_angles = orientation_angles_receiver.receive().await;
        let mut power = power_receiver.receive().await;
        loop {

            let x_out = pid_x.next_control_output(orientation_angles.x).output;
            let y_out = pid_y.next_control_output(orientation_angles.y).output;

            let diff = control_angles - orientation_angles;
            let ratio_xy = diff[0] / diff[1];

            let xy_out = compliment_filter(ratio_xy, (x_out, y_out));
            
            xpyp.set_power(power + compliment_filter(ratio_xy, (x_out, y_out)) as u8);
            xpyn.set_power(power + compliment_filter(ratio_xy, (x_out, - y_out)) as u8);
            xnyn.set_power(power + compliment_filter(ratio_xy, (- x_out, - y_out)) as u8);
            xnyp.set_power(power + compliment_filter(ratio_xy, (- x_out, y_out)) as u8);

            orientation_angles_receiver.receive().await;

        }
    },
    _ => todo!()
    };
        
    }


