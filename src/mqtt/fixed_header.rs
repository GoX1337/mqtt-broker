use super::msg_type::MessageType;

#[derive(Debug)]
pub struct FixedHeader {
    message_type: MessageType,
    reserved: u8,
    length: u8,
}
impl FixedHeader {
    pub fn new(buffer: &[u8]) -> FixedHeader {
        let msg_type_str = &format!("{:08b}", buffer[0])[0..4];
        let msg_type = match msg_type_str.parse::<u8>() {
            Ok(msg_type) => MessageType::from_u8(msg_type),
            Err(_e) => MessageType::UNKNOWN
        };
        let reserved_str = &format!("{:08b}", buffer[0])[4..8];
        let reserved = match reserved_str.parse::<u8>() {
            Ok(res) => res,
            Err(_e) => 0
        };
        FixedHeader {
            message_type: msg_type,
            reserved: reserved,
            length: buffer[1]
        }
    }
}
