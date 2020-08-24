#[derive(Debug)]
pub enum ReturnCode {
   CONNECTION_ACCEPTED,
   UNKNOWN
}
impl ReturnCode {
    pub fn from_u8(value: u8) -> ReturnCode {
        match value {
            0 => ReturnCode::CONNECTION_ACCEPTED,
            _ => ReturnCode::UNKNOWN
        }
    }
}