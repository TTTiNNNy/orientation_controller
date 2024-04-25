#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use cortex_m::asm;
use cortex_m_rt::entry;
use embassy_nrf::twim::{self, Twim};
use libm::{fabs, fabsf, pow, sqrt, sqrtf};

use core::borrow::Borrow;
use core::future::poll_fn;
use core::task::Poll;
use core::time;

use defmt::{dbg, debug, info, trace, unwrap, warn};
use embassy_executor::Spawner;
use embassy_time::{Instant, Timer};

use embassy_nrf::gpio::{Level, Output, OutputDrive};

use embassy_nrf::{bind_interrupts, peripherals, spim};
use nalgebra::{self, UnitVector3, Vector3};

//use mpu6050_dmp::{quaternion::Quaternion, yaw_pitch_roll::YawPitchRoll};
use mpu6050;

fn angle_compiment(accel: nalgebra::Vector3<f32>, gyro: nalgebra::Vector3<f32>) {}

bind_interrupts!(struct Irqs {
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;
});

fn compliment_filter(c: f32, pair_values: (f32, f32)) -> f32 {
    (pair_values.0 * c) + (pair_values.1 * (1.0 - c))
}

#[embassy_executor::task]
async fn orient_calc(mut mems: mpu6050::Mpu6050<Twim<'static, peripherals::TWISPI0>>) {
    mems.init(&mut embassy_time::Delay {}).unwrap();

    let mut angles = Vector3::new(0.0, 0.0, 0.0);

    let mut now = Instant::now();

    loop {

        let gyro = mems.get_gyro().unwrap();


        let mut acc = mems.get_acc().unwrap();
        let dt: f32 = f32::from(Instant::elapsed(&now).as_millis() as u16) * 0.001;
        now = Instant::now();
        //acc.iter_mut().for_each(|el|{*el = el.to_degrees();});

        let accel_axis_len =
        sqrtf((acc[0] * acc[0]) + (acc[1] * acc[1]) + (acc[2] * acc[2]));

        acc.iter_mut().for_each(|axi|{ *axi /=  accel_axis_len; });
        
        let accel_angles = [
            libm::acosf(acc[0].into()).to_degrees(),
            libm::acosf(acc[1].into()).to_degrees(),
            libm::acosf(acc[2].into()).to_degrees(),
        ];
        let gyro_angles = [
            angles[0] + gyro[0].to_degrees() * dt,
            angles[1] + gyro[1].to_degrees() * dt,
            angles[2] + gyro[2].to_degrees() * dt,
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

        //warn!("{}", accel_axis_norm);


        info!("accel x: {},\taccel y: {},\taccel z: {}", acc[0], acc[1], acc[2]);
        info!("gyro x: {},\tgyro y: {},\tgyro z: {}", gyro[0], gyro[1], gyro[2]);
        debug!("x: {},\ty: {},\tz: {}", angles[0], angles[1], angles[2]);


        Timer::after_millis(1).await;

    }
}
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    //Timer::after_ticks(100).await;

    let p: embassy_nrf::Peripherals = embassy_nrf::init(Default::default());
    //info!("running!");

    //let mut config = spim::Config::default();
    //config.frequency = spim::Frequency::M1;

    //let mut spim = spim::Spim::new(p.TWISPI0, Irqs, p.P0_31, p.P0_29, p.P0_26, config);
    //let mut ncs = Output::new(p.P0_04, Level::High, OutputDrive::Standard);

    //let mut rx = [0; 2];

    // read ESTAT
    // cortex_m::asm::delay(5000);
    // ncs.set_low();
    // cortex_m::asm::delay(5000);
    // let tx = [117 + (1 << 7), 0];
    // unwrap!(spim.transfer(&mut rx, &tx).await);
    // cortex_m::asm::delay(5000);
    // ncs.set_high();
    // info!("estat: {=[?]}", rx);
    let config = twim::Config::default();

    let mut twi = Twim::new(p.TWISPI0, Irqs, p.P0_26, p.P0_31, config);
    let ad0 = Output::new(p.P0_29, Level::Low, OutputDrive::Standard);

    ad0.borrow();

    let mut buf = [0u8; 2];
    unwrap!(twi.blocking_write_read(0b1101000, &mut [117], &mut buf));

    info!("Read: {=[u8]:x}", buf);

    //let mut sensor =
    //mpu6050_dmp::sensor::Mpu6050::new(twi, mpu6050_dmp::address::Address::default()).unwrap();
    let mut d = &mut embassy_time::Delay {};
    //sensor.initialize_dmp(&  mut d).unwrap();
    let mut mpu = mpu6050::Mpu6050::new(twi);
    //let mut _imu = mpu9250::Mpu9250::marg_default(spim, ncs, &  mut embassy_time::Delay{}).unwrap();
    //info!("imu init compiete!");

    unwrap!(spawner.spawn(orient_calc(mpu)));

    //asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {
        Timer::after_millis(100).await;

        // let mut acc= mpu.get_gyro().unwrap();
        // acc.iter_mut().for_each(|el|{*el = el.to_degrees();});
        // let acc = mpu.get_acc().unwrap();
        // info!("raw x: {}, y: {}, z: {}", acc[0], acc[1], acc[2] );

        // let accel_axis_norm = sqrt(pow(acc[0].into(), 2.0) + pow(acc[1].into(), 2.0) + pow(acc[2].into(), 2.0));

        // info!("x: {}, y: {}, z: {}", libm::acos(acc[0].into()).to_degrees(), libm::acos(acc[1].into()).to_degrees(), libm::acos(acc[2].into()).to_degrees() );

        // let acc = mpu.get_acc_angles().unwrap();
        // info!("roll: {}, pitch: {}", acc[0], acc[1] );

        //acc;
        //info!("imu init compiete!, {:?}", acc);

        // your code goes here
    }
}
