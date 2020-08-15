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
        let flags_str = format!("{:08b}", buffer);
        let mut flags = flags_str.chars();
        let user_name_flag = buffer_util::bool_from_char(flags.next());
        let password_flag = buffer_util::bool_from_char(flags.next());
        let will_retain = buffer_util::bool_from_char(flags.next());
        let will_qos = buffer_util::u8_from_string("");
        flags.next();
        flags.next();
        let will_flag = buffer_util::bool_from_char(flags.next());
        let clean_session = buffer_util::bool_from_char(flags.next());
        let reserved = buffer_util::bool_from_char(flags.next());
        ConnectFlags {
            user_name_flag: user_name_flag,
            password_flag: password_flag,
            will_retain: will_retain,
            will_qos: will_qos,
            will_flag: will_flag,
            clean_session: clean_session,
            reserved: reserved
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
            keep_alive: buffer[9]
        }
    }
}

#[derive(Debug)]
pub struct Payload {
    client_id: String,
    will_topic: Option<String>,
    will_message: Option<String>,
    username: Option<String>,
    password: Option<String>
}
impl Payload {
    pub fn new(buffer: &[u8]) -> Payload {
        let client_id_len = buffer[1] as usize;
        let client_id = match String::from_utf8(buffer[2..2+client_id_len].to_vec()) {
            Ok(id) => id,
            Err(_) => "".to_string()
        };
        Payload {
            client_id: client_id,
            will_topic: None,
            will_message: None,
            username: None,
            password: None
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
            payload: Payload::new(&buffer[12..])
        }
    }
}
