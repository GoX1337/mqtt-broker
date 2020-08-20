use super::connect_flags::ConnectFlags;

#[derive(Debug)]
pub struct VariableHeader {
    protocol_name: String,
    protocol_level: String,
    connect_flags: ConnectFlags,
    keep_alive: u8
}
impl VariableHeader {
    pub fn new(buffer: &[u8]) -> VariableHeader {
        let protocol_name = match String::from_utf8(buffer[2..6].to_vec()) {
            Ok(name) => name,
            Err(_) => "".to_string()
        };
        let protocol_level = match buffer[6] {
            4 => "3.3.1".to_string(),
            _ => "Unknown".to_string()
        };
        VariableHeader {
            protocol_name: protocol_name,
            protocol_level: protocol_level,
            connect_flags: ConnectFlags::new(buffer[7]),
            keep_alive: buffer[9]
        }
    }
}