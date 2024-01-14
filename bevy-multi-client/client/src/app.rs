use bevy::{
    prelude::{
        App, ClearColor, Color, IntoSystemConfigs, Startup, Update,
    },
    DefaultPlugins,
};

use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, ReceiveEvents, Message};

use bevy_multi_client_server_a_protocol::{protocol as protocolA, messages::StringMessage as StringMessageA};
use bevy_multi_client_server_b_protocol::{protocol as protocolB, messages::StringMessage as StringMessageB};

use crate::systems::{events, init};

// ClientName
pub trait ClientName: Send + Sync + 'static {
    fn name() -> &'static str;
}

pub struct Main;
pub struct Alt;

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

// IsStringMessage
pub trait IsStringMessage: Message {
    fn new(contents: String) -> Self;
    fn contents(&self) -> &String;
}

impl IsStringMessage for StringMessageA {
    fn new(contents: String) -> Self {
        Self::new(contents)
    }
    fn contents(&self) -> &String {
        &self.contents
    }
}

impl IsStringMessage for StringMessageB {
    fn new(contents: String) -> Self {
        Self::new(contents)
    }
    fn contents(&self) -> &String {
        &self.contents
    }
}

// App
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
                events::message_events::<Main, StringMessageA>,

                events::connect_events::<Alt>,
                events::disconnect_events::<Alt>,
                events::reject_events::<Alt>,
                events::message_events::<Alt, StringMessageB>,
            )
                .chain()
                .in_set(ReceiveEvents),
        )
        // Run App
        .run();
}
