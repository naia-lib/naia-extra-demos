use std::{thread::sleep, time::Duration};

use naia_server_socket::{PacketReceiver, PacketSender, ServerAddrs, Socket};

use multi_client_socket_shared_b::{shared_config_b, PING_MSG_B, PONG_MSG_B};

pub struct App {
    packet_sender: Box<dyn PacketSender>,
    packet_receiver: Box<dyn PacketReceiver>,
}

impl App {
    pub fn new() -> Self {
        info!("Multi-Client Socket Server B started");

        let server_address = ServerAddrs::new(
            "127.0.0.1:14193"
                .parse()
                .expect("could not parse Session address/port"),
            // IP Address to listen on for UDP WebRTC data channels
            "127.0.0.1:14194"
                .parse()
                .expect("could not parse WebRTC data address/port"),
            // The public WebRTC IP address to advertise
            "http://127.0.0.1:14194",
        );
        let shared_config = shared_config_b();

        let (packet_sender, packet_receiver) = Socket::listen(&server_address, &shared_config);

        App {
            packet_sender,
            packet_receiver,
        }
    }

    pub fn update(&mut self) {
        match self.packet_receiver.receive() {
            Ok(Some((address, payload))) => {
                let message_from_client = String::from_utf8_lossy(payload);
                info!("Server B recv <- {}: {}", address, message_from_client);

                if message_from_client.eq(PING_MSG_B) {
                    let message_to_client: String = PONG_MSG_B.to_string();
                    info!("Server B send -> {}: {}", address, message_to_client);
                    match self
                        .packet_sender
                        .send(&address, message_to_client.as_bytes())
                    {
                        Ok(()) => {}
                        Err(error) => {
                            info!("Server B Send Error {}", error);
                        }
                    }
                }
            }
            Ok(None) => {
                // If we don't sleep here, app will loop at 100% CPU until a new message comes in
                sleep(Duration::from_millis(1));
            }
            Err(error) => {
                info!("Server B Error: {}", error);
            }
        }
    }
}
