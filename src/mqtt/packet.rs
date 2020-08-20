use super::fixed_header::FixedHeader;
use super::variable_header::VariableHeader;
use super::payload::Payload;

#[derive(Debug)]
pub struct MqttPacket {
    fixed_header: FixedHeader,
    variable_header: VariableHeader,
    payload: Payload
}
impl MqttPacket {
    pub fn new(buffer: Vec<u8>) -> MqttPacket {
        MqttPacket {
            fixed_header: FixedHeader::new(&buffer[0..2]),
            variable_header: VariableHeader::new(&buffer[2..12]),
            payload: Payload::new(&buffer[12..])
        }
    }
}
