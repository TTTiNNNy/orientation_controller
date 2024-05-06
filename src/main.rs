#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use cortex_m::asm;
use cortex_m_rt::entry;
use embassy_nrf::twim::{self, Twim};
use libm::{fabs, fabsf, pow, sqrt, sqrtf};
use orientation_controller::{bsp, services};
use bsp::glue;

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

    unwrap!(spawner.spawn(services::mems::orient_calc(imu)));
    loop {
        Timer::after_millis(100).await;
    }
}
