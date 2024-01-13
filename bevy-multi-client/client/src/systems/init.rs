use bevy::prelude::{
    info, Camera2dBundle, Commands,
};

use naia_bevy_client::{transport::webrtc, Client};

use bevy_multi_client_server_a_protocol::messages::Auth;

use crate::resources::Global;

pub fn init(
    mut commands: Commands,
    mut client: Client,
) {
    info!("Bevy Multi Client Demo Client started");

    client.auth(Auth::new("charlie", "12345"));
    let socket = webrtc::Socket::new("http://127.0.0.1:14191", client.socket_config());
    client.connect(socket);

    // Setup Camera
    commands.spawn(Camera2dBundle::default());

    // Setup Global Resource
    let global = Global::default();

    // Insert Global Resource
    commands.insert_resource(global);
}
