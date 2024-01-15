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
        // Component
        .add_component::<MyComponent>()
        // Build Protocol
        .build()
}

// Component
use bevy_ecs::prelude::Component;

use naia_bevy_shared::{Property, Replicate};

#[derive(Component, Replicate)]
pub struct MyComponent {
    pub x: Property<i16>,
}

impl MyComponent {
    pub fn new(x: i16) -> Self {
        Self::new_complete(x)
    }
}
