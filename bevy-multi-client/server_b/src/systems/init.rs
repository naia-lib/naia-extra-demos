
use bevy_ecs::system::Commands;
use bevy_log::info;

use naia_bevy_server::{transport::webrtc, Server, CommandsExt};

use bevy_multi_client_server_b_protocol::MyComponent;

use crate::{resources::Global, SERVER_LETTER, SIGNAL_ADDR, WEBRTC_ADDR, WEBRTC_URL};

pub fn init(mut server: Server, mut commands: Commands) {
    info!("Bevy Multi-Client Demo Server {} is running", SERVER_LETTER);

    // Server initialization
    let server_addresses = webrtc::ServerAddrs::new(
        SIGNAL_ADDR
            .parse()
            .expect("could not parse Signaling address/port"),
        // IP Address to listen on for UDP WebRTC data channels
        WEBRTC_ADDR
            .parse()
            .expect("could not parse WebRTC data address/port"),
        // The public WebRTC IP address to advertise
        WEBRTC_URL,
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
        .insert(MyComponent::new(SERVER_LETTER, 0))
        .id();

    // Add to room
    server
        .room_mut(&main_room_key)
        .add_entity(&entity);
}
