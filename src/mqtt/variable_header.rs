use super::connect_flags::ConnectFlags;
use super::return_code::ReturnCode;

#[derive(Debug)]
pub struct VariableHeader {
    protocol_name: Option<String>,
    protocol_level: Option<String>,
    connect_flags: Option<ConnectFlags>,
    keep_alive: Option<u8>,
    return_code: Option<ReturnCode>,
    pub length: usize,
}
impl VariableHeader {
    pub fn new(buffer: &[u8], start: usize) -> VariableHeader {
        let mut index = start;
        //println!("Variable header {:?}", &buffer[index..]);
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
            protocol_name: Some(protocol_name),
            protocol_level: Some(protocol_level),
            connect_flags: Some(ConnectFlags::new(buffer[index])),
            keep_alive: Some(buffer[index + 2]),
            return_code: None,
            length: index + 1
        }
    }

    pub fn new_connack_header(return_code: ReturnCode) -> VariableHeader {
        VariableHeader {
            return_code: Some(return_code),
            length: 3,
            protocol_name:None,
            protocol_level: None,
            connect_flags: None,
            keep_alive: None
        }
    }
}