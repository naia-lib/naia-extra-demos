use bevy::prelude::{
    info, Camera2dBundle, Commands,
};

use naia_bevy_client::{transport::webrtc, Client};

use bevy_multi_client_server_a_protocol::messages::{Auth as AuthA};
use bevy_multi_client_server_b_protocol::messages::{Auth as AuthB};

use crate::{app::{Main, Alt}, resources::Global};

pub fn init(
    mut commands: Commands,
    mut client_main: Client<Main>,
    mut client_alte: Client<Alt>,
) {
    info!("Bevy Multi Client Demo Client started");

    // Setup Main Client
    client_main.auth(AuthA::new("charlie", "12345"));
    let socket = webrtc::Socket::new("http://127.0.0.1:14191", client_main.socket_config());
    client_main.connect(socket);

    // Setup Alt Client
    client_alte.auth(AuthB::new("charlie", "12345"));
    let socket = webrtc::Socket::new("http://127.0.0.1:14193", client_alte.socket_config());
    client_alte.connect(socket);

    // Setup Camera
    commands.spawn(Camera2dBundle::default());

    // Setup Global Resource
    let global = Global::default();

    // Insert Global Resource
    commands.insert_resource(global);
}
