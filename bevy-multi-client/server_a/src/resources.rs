
use bevy_ecs::prelude::Resource;

use naia_bevy_server::RoomKey;

#[derive(Resource)]
pub struct Global {
    pub main_room_key: RoomKey,
}