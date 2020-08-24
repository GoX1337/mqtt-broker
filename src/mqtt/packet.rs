use super::fixed_header::FixedHeader;
use super::variable_header::VariableHeader;
use super::payload::Payload;
use super::msg_type::MessageType;

#[derive(Debug)]
pub struct MqttPacket {
    pub fixed_header: FixedHeader,
    pub variable_header: VariableHeader,
    pub payload: Option<Payload>
}
impl MqttPacket {
    pub fn new(buffer: &[u8]) -> MqttPacket {
        //println!("MqttPacket buffer: {:?}, len:{}", buffer, buffer.len());
        let fixed_header = FixedHeader::new(&buffer);
        let variable_header = VariableHeader::new(&buffer, fixed_header.length);
        let payload = Payload::new(&buffer, fixed_header.length + variable_header.length);
        MqttPacket {
            fixed_header: fixed_header,
            variable_header: variable_header,
            payload: Some(payload)
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match self.fixed_header.message_type {
            MessageType::CONNACK => {
                Vec::new()
            }
            _ =>  Vec::new()
        }
    }
}