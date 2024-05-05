use embedded_hal_async::delay::DelayNs;
use icm20948_async::{BusTransfer, Init, MagEnabled};
use nalgebra::Vector3;

pub struct MemsData {
    pub acc: Vector3<f32>,
    pub gyr: Vector3<f32>,
    pub mag: Vector3<f32>,
    pub tmp: f32,
}

pub trait Mems {
    type ERR;
    async fn read_axis_9(&mut self) -> Result<MemsData, Self::ERR>;
}

impl<BUS: BusTransfer<E>, DELAY: DelayNs, E> Mems
    for icm20948_async::Icm20948<BUS, MagEnabled, Init, DELAY, E>
{
    type ERR = E;
    async fn read_axis_9(&mut self) -> Result<MemsData, E> {
        let res = self.read_9dof().await;
        match res {
            Ok(vals) => Result::Ok(MemsData {
                acc: vals.acc,
                gyr: vals.gyr,
                mag: vals.mag,
                tmp: vals.tmp,
            }),
            Err(err) => core::result::Result::Err(err),
        }
    }
}
