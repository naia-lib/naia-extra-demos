
#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(not(feature = "alternate"))]
    {
        pub const SERVER_LETTER: &str = "B";
        pub const SIGNAL_ADDR: &str = "127.0.0.1:14193";
        pub const WEBRTC_ADDR: &str = "127.0.0.1:14194";
        pub const WEBRTC_URL: &str = "http://127.0.0.1:14194";
    }
    else
    {
        pub const SERVER_LETTER: &str = "C";
        pub const SIGNAL_ADDR: &str = "127.0.0.1:14195";
        pub const WEBRTC_ADDR: &str = "127.0.0.1:14196";
        pub const WEBRTC_URL: &str = "http://127.0.0.1:14196";
    }
}

use std::time::Duration;

use bevy_app::{App, ScheduleRunnerPlugin, Startup, Update};
use bevy_core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_log::{info, LogPlugin};

use naia_bevy_server::{Plugin as ServerPlugin, ReceiveEvents, ServerConfig};

mod systems;
use systems::{events, init};

use bevy_multi_client_server_b_protocol::protocol;

fn main() {
    info!("Bevy Multi-Client Demo Server {} Demo starting up", SERVER_LETTER);

    let mut server_config = ServerConfig::default();
    server_config.connection.disconnection_timeout_duration = Duration::from_secs(10);

    // Build App
    App::default()
        // Plugins
        .add_plugins(TaskPoolPlugin::default())
        .add_plugins(TypeRegistrationPlugin::default())
        .add_plugins(FrameCountPlugin::default())
        // this is needed to avoid running the server at uncapped FPS
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(3)))
        .add_plugins(LogPlugin::default())
        .add_plugins(ServerPlugin::new(server_config, protocol()))
        // Startup System
        .add_systems(Startup, init)
        // Receive Server Events
        .add_systems(
            Update,
            (
                events::auth_events,
                events::connect_events,
                events::disconnect_events,
                events::error_events,
                events::tick_events,
                events::message_events,
            )
                .chain()
                .in_set(ReceiveEvents),
        )
        // Run App
        .run();
}
