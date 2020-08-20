
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