
use bevy_log::info;

use naia_bevy_server::{transport::webrtc, Server};

use crate::{SERVER_LETTER, SIGNAL_ADDR, WEBRTC_ADDR, WEBRTC_URL};

pub fn init(mut server: Server) {
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
}
