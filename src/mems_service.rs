use core::future::poll_fn;
use core::task::Poll;

//use defmt::{info, unwrap};
use embassy_time::{Instant, Timer};

#[embassy_executor::task]
async fn service_mems() {
    //let mut _imu = mpu9250::Mpu9250::marg_default(spi, ncs, &mut delay)?;
    // to create sensor without mag support and default configuration:
    //let mut marg = mpu9250::Mpu9250::imu_default(spi, ncs, &mut delay)?;
    // to get all supported measurements:
    //let all = marg.all()?;
    //println!("{:?}", all);
    loop {
        //    info!("DING DONG");

        Timer::after_ticks(16000).await;
    }
}
