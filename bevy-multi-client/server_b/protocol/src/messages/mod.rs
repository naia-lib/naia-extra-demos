mod auth;
mod string_message;

pub use auth::Auth;
pub use string_message::StringMessage;

use naia_bevy_shared::{Protocol, ProtocolPlugin};

// Plugin
pub struct MessagesPlugin;

impl ProtocolPlugin for MessagesPlugin {
    fn build(&self, protocol: &mut Protocol) {
        protocol
            .add_message::<Auth>()
            .add_message::<StringMessage>();
    }
}
