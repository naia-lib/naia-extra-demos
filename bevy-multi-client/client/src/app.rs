use bevy::{
    prelude::{
        App, ClearColor, Color, IntoSystemConfigs, Startup, Update,
    },
    DefaultPlugins,
};

use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, ReceiveEvents};

use bevy_multi_client_server_a_protocol::{protocol as protocolA};
use bevy_multi_client_server_b_protocol::{protocol as protocolB};

use crate::systems::{events, init};

// Marker for the main client
pub struct Main;
pub struct Alt;

pub trait ClientName: Send + Sync + 'static {
    fn name() -> &'static str;
}

impl ClientName for Main {
    fn name() -> &'static str {
        "Main"
    }
}

impl ClientName for Alt {
    fn name() -> &'static str {
        "Alt"
    }
}

pub fn run() {
    App::default()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        // Add Client Plugins
        .add_plugins(ClientPlugin::<Main>::new(ClientConfig::default(), protocolA()))
        .add_plugins(ClientPlugin::<Alt>::new(ClientConfig::default(), protocolB()))
        // Background Color
        .insert_resource(ClearColor(Color::BLACK))
        // Startup System
        .add_systems(Startup, init)
        // Receive Client Events
        .add_systems(
            Update,
            (
                events::connect_events::<Main>,
                events::disconnect_events::<Main>,
                events::reject_events::<Main>,
                events::message_events_main,

                events::connect_events::<Alt>,
                events::disconnect_events::<Alt>,
                events::reject_events::<Alt>,
                events::message_events_alt,
            )
                .chain()
                .in_set(ReceiveEvents),
        )
        // Run App
        .run();
}
