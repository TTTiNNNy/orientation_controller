use crate::services::input::InputControlState;


struct UartPacket{
    packet_type: u8,
    payload: InputControlState
}
