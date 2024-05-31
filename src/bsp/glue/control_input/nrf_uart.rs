
use crate::services::input::InputControlState;
use embassy_nrf::{bind_interrupts, peripherals, uarte::{self, Instance}};

struct ControlInput<'d, T: Instance>{
    input: AbstractUart<'d, T>
}

struct AbstractUart<'d, T: Instance>{
    base: uarte::Uarte<'d, T>
}

fn qwe(){
    let p = embassy_nrf::init(Default::default());
    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD115200;
    //let mut uart: uarte::Uarte<peripherals::UARTE0> = uarte::Uarte::new(p.UARTE0, Irqs, p.P0_08, p.P0_06, config);
   // ControlInput{input: uart};
}