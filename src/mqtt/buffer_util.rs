
pub fn bool_from_char(char_bit_option: Option<char>) -> bool {
    let user_name_flag = match char_bit_option {
        Some(v) => v == '1',
        None => false
    };
    user_name_flag
}