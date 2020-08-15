
pub fn bool_from_char(char_bit_option: Option<char>) -> bool {
    let user_name_flag = match char_bit_option {
        Some(v) => v == '1',
        None => false
    };
    user_name_flag
}

pub fn u8_from_string(value_str: &str) -> u8 {
    println!("{}", value_str);
    let value = match value_str.parse::<u8>() {
        Ok(v) => v,
        Err(_e) => 0
    };
    value
}