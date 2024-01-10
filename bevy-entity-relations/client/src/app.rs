use bevy::{
    prelude::{
        App, ClearColor, Color, SystemSet,
    },
    DefaultPlugins,
    app::{Startup, Update},
};

use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin};

use bevy_entity_relations_shared::protocol;

use crate::systems::{events, init, input, sync};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct MainLoop;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct Tick;

pub fn run() {
    App::default()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        // Add Naia Client Plugin
        .add_plugins(ClientPlugin::new(ClientConfig::default(), protocol()))
        // Background Color
        .insert_resource(ClearColor(Color::BLACK))
        // Startup System
        .add_systems(Startup, init)
        // Receive Client Events
        .add_systems(Update, events::connect_events)
        .add_systems(Update, events::disconnect_events)
        .add_systems(Update, events::reject_events)
        .add_systems(Update, events::spawn_entity_events)
        .add_systems(Update, events::despawn_entity_events)
        .add_systems(Update, events::insert_component_events)
        .add_systems(Update, events::update_component_events)
        .add_systems(Update, events::remove_component_events)
        .add_systems(Update, events::message_events)
        // Tick Event
        .add_systems(Update, events::tick_events)
        // Realtime Gameplay Loop
        .add_systems(Update, input::key_input)
        .add_systems(Update, input::cursor_input)
        .add_systems(Update, sync::sync_clientside_sprites)
        .add_systems(Update, sync::sync_serverside_sprites)
        .add_systems(Update, sync::sync_cursor_sprite)
        .add_systems(Update, sync::sync_relation_lines)
        .add_systems(Update, sync::sync_baseline)
        // Run App
        .run();
}
