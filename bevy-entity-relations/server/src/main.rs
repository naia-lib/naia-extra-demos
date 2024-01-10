use std::time::Duration;

use bevy_app::{App, ScheduleRunnerPlugin, Startup, Update};
use bevy_core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
use bevy_log::{info, LogPlugin};

use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig};

use bevy_entity_relations_shared::protocol;

mod resources;
mod systems;

use systems::{events, init};

fn main() {
    info!("Naia Bevy Server Demo starting up");

    // Build App
    App::default()
        // Plugins
        .add_plugins(TaskPoolPlugin::default())
        .add_plugins(TypeRegistrationPlugin::default())
        .add_plugins(FrameCountPlugin::default())
        // this is needed to avoid running the server at uncapped FPS
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(3)))
        .add_plugins(LogPlugin::default())
        .add_plugins(ServerPlugin::new(ServerConfig::default(), protocol()))
        // Startup System
        .add_systems(Startup, init)
        // Receive Server Events
        .add_systems(Update, events::auth_events)
        .add_systems(Update, events::connect_events)
        .add_systems(Update, events::disconnect_events)
        .add_systems(Update, events::error_events)
        .add_systems(Update, events::tick_events)
        .add_systems(Update, events::spawn_entity_events)
        .add_systems(Update, events::spawn_entity_events)
        .add_systems(Update, events::despawn_entity_events)
        .add_systems(Update, events::publish_entity_events)
        .add_systems(Update, events::insert_component_events)
        .add_systems(Update, events::remove_component_events)
        // Run App
        .run();
}
