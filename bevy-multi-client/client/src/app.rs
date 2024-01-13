use bevy::{
    prelude::{
        App, ClearColor, Color, IntoSystemConfigs, Startup, Update,
    },
    DefaultPlugins,
};

use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, ReceiveEvents};

use bevy_multi_client_server_a_protocol::protocol;

use crate::systems::{events, init};

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
        .add_systems(
            Update,
            (
                events::connect_events,
                events::disconnect_events,
                events::reject_events,
                events::message_events,
            )
                .chain()
                .in_set(ReceiveEvents),
        )
        // Run App
        .run();
}
