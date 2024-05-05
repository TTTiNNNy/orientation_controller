#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use cortex_m::asm;
use cortex_m_rt::entry;
use embassy_nrf::twim::{self, Twim};
use libm::{fabs, fabsf, pow, sqrt, sqrtf};
use orientation_controller::bsp::glue;

use core::borrow::Borrow;
use core::future::poll_fn;
use core::task::Poll;
use core::time;

use defmt::{dbg, debug, info, trace, unwrap, warn};
use embassy_executor::Spawner;
use embassy_time::{Instant, Timer};

use embassy_nrf::gpio::{Level, Output, OutputDrive};

use embassy_nrf::{bind_interrupts, peripherals, spim, Peripheral};
use nalgebra::{self, UnitVector3, Vector3};

use icm20948_async;

bind_interrupts!(struct Irqs {
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;
});

fn compliment_filter(c: f32, pair_values: (f32, f32)) -> f32 {
    (pair_values.0 * c) + (pair_values.1 * (1.0 - c))
}

// fn axis_diff(accel: Vector3<f32>, gyro: Vector3<f32>, magn: Vector3<f32>) -> Vector3<f32>{

//     Vector3{..Default::default()}
// }

#[embassy_executor::task]
async fn orient_calc(mut mems: impl glue::mems::Mems + 'static) {
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

                let accel_axis_len = sqrtf(
                    (mems_data.acc[0] * mems_data.acc[0])
                        + (mems_data.acc[1] * mems_data.acc[1])
                        + (mems_data.acc[2] * mems_data.acc[2]),
                );

                mems_data.acc.iter_mut().for_each(|axi| {
                    *axi /= accel_axis_len;
                });

                let accel_angles = [
                    libm::acosf(mems_data.acc[0].into()).to_degrees(),
                    libm::acosf(mems_data.acc[1].into()).to_degrees(),
                    libm::acosf(mems_data.acc[2].into()).to_degrees(),
                ];
                let gyro_angles = [
                    angles[0] + mems_data.gyr[0].to_degrees() * dt,
                    angles[1] + mems_data.gyr[1].to_degrees() * dt,
                    angles[2] + mems_data.gyr[2].to_degrees() * dt,
                ];

                gyro_angles
                    .iter()
                    .enumerate()
                    .zip(accel_angles.iter())
                    .for_each(|((i, &acc), &gyro)| {
                        angles[i] = compliment_filter(
                            0.96 + (fabsf(accel_axis_len - 1.0) * 0.04),
                            (gyro, acc),
                        )
                    });

                trace!("accel_axis_len: {}", accel_axis_len);

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

                Timer::after_millis(1).await;
            }

            Err(err) => {}
        }
    }
}

#[embassy_executor::main]
async fn init(spawner: Spawner) {
    Timer::after_ticks(100).await;

    let p: embassy_nrf::Peripherals = embassy_nrf::init(Default::default());
    let config = twim::Config::default();

    let mut twi: Twim<'_, peripherals::TWISPI0> =
        Twim::new(p.TWISPI0, Irqs, p.P0_26, p.P0_31, config);
    let mut buf = [0u8; 2];
    unwrap!(twi.blocking_write_read(0b1101000, &mut [117], &mut buf));

    info!("Read: {=[u8]:x}", buf);

    let imu = icm20948_async::Icm20948::new_i2c(twi, embassy_time::Delay)
        .gyr_unit(icm20948_async::GyrUnit::Dps)
        .gyr_dlp(icm20948_async::GyrDlp::Hz361)
        .acc_range(icm20948_async::AccRange::Gs4)
        .set_address(0x69)
        .initialize_9dof()
        .await
        .unwrap();

    unwrap!(spawner.spawn(orient_calc(imu)));
    loop {
        Timer::after_millis(100).await;
    }
}
