use core::future::poll_fn;
use core::task::Poll;

use crate::bsp::glue;
use defmt::trace;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::Sender};
use embassy_time::{Instant, Timer};
use libm::sqrtf;
use nalgebra::Vector3;
use crate::utils::comp_fltr::compliment_filter;

fn axis_degree_diff(
    prev_angles: &mut Vector3<f32>,
    mut accel: Vector3<f32>,
    gyro: Vector3<f32>,
    magn: Vector3<f32>,
    dt_s: f32,
) {
    let accel_axis_len =
        sqrtf((accel[0] * accel[0]) + (accel[1] * accel[1]) + (accel[2] * accel[2]));

    accel.iter_mut().for_each(|axi| {
        *axi /= accel_axis_len;
    });

    trace!("accel_axis_len: {}", accel_axis_len);

    let accel_angles = [
        libm::acosf(accel[0].into()).to_degrees(),
        libm::acosf(accel[1].into()).to_degrees(),
        libm::acosf(accel[2].into()).to_degrees(),
    ];

    let gyro_angles = [
        prev_angles[0] + gyro[0].to_degrees() * dt_s,
        prev_angles[1] + gyro[1].to_degrees() * dt_s,
        prev_angles[2] + gyro[2].to_degrees() * dt_s,
    ];

    gyro_angles
        .iter()
        .enumerate()
        .zip(accel_angles.iter())
        .for_each(|((i, &acc), &gyro)| {
            prev_angles[i] = compliment_filter(1.0 - (0.04 - accel_axis_len / 100.0), (gyro, acc))
        });
}

#[embassy_executor::task]
pub async fn orient_calc(
    mut mems: impl glue::mems::Mems + 'static,
    sender: Sender<'static, NoopRawMutex, Vector3<f32>, 1>,
) {
    // #[embassy_executor::task]
    // async fn orient_calc(p: impl) {

    let mut angles = Vector3::new(0.0, 0.0, 0.0);

    let mut now = Instant::now();

    loop {
        let mut mems_data = mems.read_axis_9().await;

        match mems_data {
            Ok(mut mems_data) => {
                let dt: f32 = f32::from(Instant::elapsed(&now).as_millis() as u16) * 0.001;
                now = Instant::now();
                mems_data.acc.iter_mut().for_each(|el| {
                    *el = el.to_degrees();
                });

                axis_degree_diff(&mut angles, mems_data.acc, mems_data.gyr, mems_data.mag, dt);

                trace!(
                    "accel x: {},\taccel y: {},\taccel z: {}",
                    mems_data.acc[0],
                    mems_data.acc[1],
                    mems_data.acc[2]
                );
                trace!(
                    "gyro x: {},\tgyro y: {},\tgyro z: {}",
                    mems_data.gyr[0],
                    mems_data.gyr[1],
                    mems_data.gyr[2]
                );
                trace!("x: {},\ty: {},\tz: {}", angles[0], angles[1], angles[2]);
                sender.send(angles.clone()).await;

                Timer::after_millis(1).await;
            }

            Err(err) => {}
        }
    }
}
