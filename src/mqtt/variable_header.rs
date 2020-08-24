use super::connect_flags::ConnectFlags;

#[derive(Debug)]
pub struct VariableHeader {
    protocol_name: String,
    protocol_level: String,
    connect_flags: ConnectFlags,
    keep_alive: u8,
    pub length: usize
}
impl VariableHeader {
    pub fn new(buffer: &[u8], start: usize) -> VariableHeader {
        let mut index = start;
        println!("Variable header {:?}", &buffer[index..]);
        index = index + 1;
        let protocol_len = buffer[index] as usize;
        index = index + 1;
        let protocol_name = match String::from_utf8(buffer[index..index + protocol_len].to_vec()) {
            Ok(name) => name,
            Err(_) => "".to_string()
        };
        index = index + protocol_len;
        let protocol_level = match buffer[index] {
            3 => "3.1".to_string(),
            4 => "3.3.1".to_string(),
            _ => "Unknown".to_string()
        };
        index = index + 1;
        VariableHeader {
            protocol_name: protocol_name,
            protocol_level: protocol_level,
            connect_flags: ConnectFlags::new(buffer[index]),
            keep_alive: buffer[index + 2],
            length: index + 1
        }
    }
}