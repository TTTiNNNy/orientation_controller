pub trait EscApi {
    async fn set_power(&mut self, channel: u8, power_percent: u8);
}
