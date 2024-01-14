use bevy::{
    prelude::{
        App, ClearColor, Color, IntoSystemConfigs, Startup, Update,
    },
    DefaultPlugins,
};

use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, ReceiveEvents, Message};

use bevy_multi_client_server_a_protocol::{protocol as protocolA, messages::StringMessage as StringMessageA};
use bevy_multi_client_server_b_protocol::{protocol as protocolB, messages::StringMessage as StringMessageB};

use crate::resources::Global;
use crate::systems::{events, init};

// ClientName
pub trait ClientName: Send + Sync + 'static {
    fn name() -> &'static str;
    fn get_msg_count(global: &Global) -> u32;
    fn inc_msg_count(global: &mut Global);
    fn reset_msg_count(global: &mut Global);
}

pub struct Main;
pub struct Alt;
pub struct Alt2;

impl ClientName for Main {
    fn name() -> &'static str {
        "A"
    }

    fn get_msg_count(global: &Global) -> u32 {
        global.message_count_a
    }

    fn inc_msg_count(global: &mut Global) {
        global.message_count_a += 1;
    }

    fn reset_msg_count(global: &mut Global) {
        global.message_count_a = 0;
    }
}

impl ClientName for Alt {
    fn name() -> &'static str {
        "B"
    }

    fn get_msg_count(global: &Global) -> u32 {
        global.message_count_b
    }

    fn inc_msg_count(global: &mut Global) {
        global.message_count_b += 1;
    }

    fn reset_msg_count(global: &mut Global) {
        global.message_count_b = 0;
    }
}

impl ClientName for Alt2 {
    fn name() -> &'static str {
        "C"
    }

    fn get_msg_count(global: &Global) -> u32 {
        global.message_count_c
    }

    fn inc_msg_count(global: &mut Global) {
        global.message_count_c += 1;
    }

    fn reset_msg_count(global: &mut Global) {
        global.message_count_c = 0;
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
        .add_plugins(ClientPlugin::<Alt2>::new(ClientConfig::default(), protocolB()))
        // Background Color
        .insert_resource(ClearColor(Color::BLACK))
        // Resource
        .init_resource::<Global>()
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

                events::connect_events::<Alt2>,
                events::disconnect_events::<Alt2>,
                events::reject_events::<Alt2>,
                events::message_events::<Alt2, StringMessageB>,

                events::toggle_between_alt_clients,
            )
                .chain()
                .in_set(ReceiveEvents),
        )
        // Run App
        .run();
}
