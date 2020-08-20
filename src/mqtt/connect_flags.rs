use super::buffer_util;

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