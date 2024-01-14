use std::default::Default;

use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Global {
    pub message_count_a: u32,
    pub message_count_b: u32,
    pub message_count_c: u32,
}

impl Default for Global {
    fn default() -> Self {
        Self {
            message_count_a: 0,
            message_count_b: 0,
            message_count_c: 0,
        }
    }
}
