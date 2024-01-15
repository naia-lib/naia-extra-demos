
use bevy_ecs::prelude::Commands;
use bevy_log::info;

use naia_bevy_server::{transport::webrtc, Server, CommandsExt};

use bevy_multi_client_server_a_protocol::MyComponent;

use crate::{LETTER, resources::Global};

pub fn init(mut server: Server, mut commands: Commands) {
    info!("Bevy Multi-Client Demo Server {} is running", LETTER);

    // Naia Server initialization
    let server_addresses = webrtc::ServerAddrs::new(
        "127.0.0.1:14191"
            .parse()
            .expect("could not parse Signaling address/port"),
        // IP Address to listen on for UDP WebRTC data channels
        "127.0.0.1:14192"
            .parse()
            .expect("could not parse WebRTC data address/port"),
        // The public WebRTC IP address to advertise
        "http://127.0.0.1:14192",
    );
    let socket = webrtc::Socket::new(&server_addresses, server.socket_config());
    server.listen(socket);

    // Make room
    let main_room_key = server.make_room().key();

    // Init Global Resource
    let global = Global {
        main_room_key,
    };

    // Insert Global Resource
    commands.insert_resource(global);

    // Make component
    let entity = commands
        .spawn_empty()
        .enable_replication(&mut server)
        .insert(MyComponent::new(0))
        .id();

    // Add to room
    server
        .room_mut(&main_room_key)
        .add_entity(&entity);
}
