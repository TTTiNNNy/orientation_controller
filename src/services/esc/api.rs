
pub trait EscApi{
    async fn set_power(&mut self, power_percent: u8);
    fn get_crc();
}