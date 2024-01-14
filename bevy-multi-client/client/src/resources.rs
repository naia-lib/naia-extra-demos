use std::default::Default;

use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Global {
    pub message_count: u32
}

impl Default for Global {
    fn default() -> Self {
        Self {
            message_count: 0,
        }
    }
}
