use std::{thread::sleep, time::Duration};

use naia_server_socket::{PacketReceiver, PacketSender, ServerAddrs, Socket};

use multi_client_socket_shared_a::{shared_config_a, PING_MSG_A, PONG_MSG_A};

pub struct App {
    packet_sender: Box<dyn PacketSender>,
    packet_receiver: Box<dyn PacketReceiver>,
}

impl App {
    pub fn new() -> Self {
        info!("Multi-Client Socket Server A started");

        let server_address = ServerAddrs::new(
            "127.0.0.1:14191"
                .parse()
                .expect("could not parse Session address/port"),
            // IP Address to listen on for UDP WebRTC data channels
            "127.0.0.1:14192"
                .parse()
                .expect("could not parse WebRTC data address/port"),
            // The public WebRTC IP address to advertise
            "http://127.0.0.1:14192",
        );
        let shared_config = shared_config_a();

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
                info!("Server A recv <- {}: {}", address, message_from_client);

                if message_from_client.eq(PING_MSG_A) {
                    let message_to_client: String = PONG_MSG_A.to_string();
                    info!("Server A send -> {}: {}", address, message_to_client);
                    match self
                        .packet_sender
                        .send(&address, message_to_client.as_bytes())
                    {
                        Ok(()) => {}
                        Err(error) => {
                            info!("Server Send Error {}", error);
                        }
                    }
                }
            }
            Ok(None) => {
                // If we don't sleep here, app will loop at 100% CPU until a new message comes in
                sleep(Duration::from_millis(1));
            }
            Err(error) => {
                info!("Server A Error: {}", error);
            }
        }
    }
}
