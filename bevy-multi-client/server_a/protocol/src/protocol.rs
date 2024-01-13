use std::time::Duration;

use naia_bevy_shared::{LinkConditionerConfig, Protocol};

use crate::messages::MessagesPlugin;

// Protocol Build
pub fn protocol() -> Protocol {
    Protocol::builder()
        // Config
        .tick_interval(Duration::from_millis(800))
        .link_condition(LinkConditionerConfig::average_condition())
        // Channels
        .add_default_channels()
        // Messages
        .add_plugin(MessagesPlugin)
        // Build Protocol
        .build()
}
