use naia_bevy_shared::Message;

#[derive(Message)]
pub struct StringMessage {
    pub contents: String,
}

impl StringMessage {
    pub fn new(contents: String) -> Self {
        Self { contents }
    }
}
