mod buffer_util;
mod msg_type;
mod fixed_header;
mod variable_header;
mod connect_flags;
mod payload;
mod packet;
mod return_code;

use packet::MqttPacket;
use msg_type::MessageType;
use fixed_header::FixedHeader;
use variable_header::VariableHeader;
use return_code::ReturnCode;

pub fn process_packet(buffer: &[u8]) -> Option<MqttPacket> {
    let packet = MqttPacket::new(buffer);
    println!("{:?}", packet);
    match packet.fixed_header.message_type {
        MessageType::CONNECT => Some(build_connack_packet()),
        _ => None
    }
}

fn build_connack_packet() -> MqttPacket {
    MqttPacket {
        fixed_header: FixedHeader {
            message_type: MessageType::CONNACK,
            reserved: 0,
            length: 2
        },
        variable_header: VariableHeader::new_connack_header(ReturnCode::CONNECTION_ACCEPTED),
        payload: None
    }
}