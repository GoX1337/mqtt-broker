#[derive(Debug)]
pub enum MessageType {
    CONNECT,
    CONNACK,
    PUBLISH,
    PUBACK,
    PUBREC,
    PUBREL,
    PUBCOMP,
    SUBSCRIBE,
    SUBACK,
    UNSUBSCRIBE,
    UNSUBACK,
    PINGREQ,
    PINGRESP,
    DISCONNECT,
    RESERVED,
    UNKNOWN
}
impl MessageType {
    pub fn from_u8(value: u8) -> MessageType {
        match value {
            0 => MessageType::RESERVED,
            1 => MessageType::CONNECT,
            2 => MessageType::CONNACK,
            3 => MessageType::PUBLISH,
            4 => MessageType::PUBACK,
            5 => MessageType::PUBREC,
            6 => MessageType::PUBREL,
            7 => MessageType::PUBCOMP,
            8 => MessageType::SUBSCRIBE,
            9 => MessageType::SUBACK,
            10 => MessageType::UNSUBSCRIBE,
            11 => MessageType::UNSUBACK,
            12 => MessageType::PINGREQ,
            13 => MessageType::PINGRESP,
            14 => MessageType::DISCONNECT,
            15 => MessageType::RESERVED,
            _ => MessageType::UNKNOWN
        }
    }
}