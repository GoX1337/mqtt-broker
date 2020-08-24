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
    pub fn new(buffer: &[u8]) -> MqttPacket {
        println!("MqttPacket buffer: {:?}, len:{}", buffer, buffer.len());
        let fixed_header = FixedHeader::new(&buffer);
        let variable_header = VariableHeader::new(&buffer, fixed_header.length);
        let payload = Payload::new(&buffer, fixed_header.length + variable_header.length);
        MqttPacket {
            fixed_header: fixed_header,
            variable_header: variable_header,
            payload: payload
        }
    }
}
