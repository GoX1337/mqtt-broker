mod buffer_util;
mod msg_type;
use msg_type::MessageType;

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

#[derive(Debug)]
pub struct ConnectFlags {
    user_name_flag: bool,
    password_flag: bool,
    will_retain: bool,
    will_qos: u8,
    will_flag: bool,
    clean_session: bool,
    reserved: bool
}
impl ConnectFlags {
    pub fn new(buffer: u8) -> ConnectFlags {
        let mut flags = format!("{:08b}", buffer).chars();
        let char1 = flags.nth(0);

        ConnectFlags {
            user_name_flag: false,
            password_flag: false,
            will_retain: false,
            will_qos: 0,
            will_flag: false,
            clean_session: false,
            reserved: false
        }
    }
}

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
            keep_alive: 0
        }
    }
}

#[derive(Debug)]
pub struct Payload {
}
impl Payload {
    pub fn new() -> Payload {
        Payload {
        }
    }
}

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
            payload: Payload::new()
        }
    }
}
