use std::default::Default;

use bevy::prelude::Resource;

pub const LETTER_A: &str = "A";

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
