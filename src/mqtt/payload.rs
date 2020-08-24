
#[derive(Debug)]
pub struct Payload {
    client_id: String,
    will_topic: Option<String>,
    will_message: Option<String>,
    username: Option<String>,
    password: Option<String>
}
impl Payload {
    pub fn new(buffer: &[u8], start: usize) -> Payload {
        let mut index = start;
        //println!("Payload {:?}", &buffer[index..]);
        index = index + 1;
        let client_id_len = buffer[index] as usize;
        index = index + 1;
        let client_id = match String::from_utf8(buffer[index..index+client_id_len].to_vec()) {
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