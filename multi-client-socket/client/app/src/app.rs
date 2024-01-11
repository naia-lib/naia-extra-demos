use std::time::Duration;

cfg_if! {
    if #[cfg(feature = "mquad")] {
        use miniquad::info;
    } else {
        use log::info;
    }
}

use naia_client_socket::{PacketReceiver, PacketSender, ServerAddr, Socket};

use naia_shared::Timer;

use multi_client_socket_server_a_protocol::{shared_config_a, PING_MSG_A, PONG_MSG_A};
use multi_client_socket_server_b_protocol::{shared_config_b, PING_MSG_B, PONG_MSG_B};

pub struct Client {
    letter: String,
    ping_msg: String,
    pong_msg: String,
    packet_sender: Box<dyn PacketSender>,
    packet_receiver: Box<dyn PacketReceiver>,
    message_count: u8,
    timer: Timer,
    server_addr_str: Option<String>,
}

impl Client {
    pub fn update(&mut self) -> bool {

        if self.server_addr_str.is_none() {
            if let ServerAddr::Found(addr) = self.packet_receiver.server_addr() {
                self.server_addr_str = Some(addr.to_string());
            }
        }

        match self.packet_receiver.receive() {
            Ok(Some(packet)) => {
                let message_from_server = String::from_utf8_lossy(packet);

                info!(
                    "Client recv <- {} ({}): {}",
                    self.server_addr_str.as_ref().unwrap_or(&"".to_string()),
                    self.letter,
                    message_from_server
                );

                if message_from_server.eq(self.pong_msg.as_str()) {
                    self.message_count += 1;

                    if self.message_count == 3 {
                        info!("Client finished sending messages to {}", self.letter);
                        return true;
                    }
                }
            }
            Ok(None) => {
                if self.message_count < 3 {
                    if self.timer.ringing() {
                        self.timer.reset();

                        let message_to_server: String = self.ping_msg.clone();

                        let server_addr = match self.packet_receiver.server_addr() {
                            ServerAddr::Found(addr) => addr.to_string(),
                            _ => "".to_string(),
                        };
                        info!("Client send -> {} ({}): {}", server_addr, self.letter, message_to_server);

                        match self.packet_sender.send(message_to_server.as_bytes()) {
                            Ok(()) => {}
                            Err(error) => {
                                info!("Client Send Error (to {}): {}", self.letter, error);
                            }
                        }
                    }
                } else {
                    return true;
                }
            }
            Err(err) => {
                info!("Client Error (from {}): {}", self.letter, err);
            }
        }

        return false;
    }
}

pub struct App {
    client_a: Option<Client>,
    client_b: Option<Client>,
}

impl App {
    pub fn new() -> App {
        info!("Multi-Client Socket Client started");

        let (packet_sender_a, packet_receiver_a) =
            Socket::connect("http://127.0.0.1:14191", &shared_config_a());

        let client_a = Client {
            letter: "A".to_string(),
            ping_msg: PING_MSG_A.to_string(),
            pong_msg: PONG_MSG_A.to_string(),
            packet_sender: packet_sender_a,
            packet_receiver: packet_receiver_a,
            message_count: 0,
            timer: Timer::new(Duration::from_secs(1)),
            server_addr_str: None,
        };

        let (packet_sender_b, packet_receiver_b) =
            Socket::connect("http://127.0.0.1:14193", &shared_config_b());

        let client_b = Client {
            letter: "B".to_string(),
            ping_msg: PING_MSG_B.to_string(),
            pong_msg: PONG_MSG_B.to_string(),
            packet_sender: packet_sender_b,
            packet_receiver: packet_receiver_b,
            message_count: 0,
            timer: Timer::new(Duration::from_secs(1)),
            server_addr_str: None,
        };

        App {
            client_a: Some(client_a),
            client_b: Some(client_b),
        }
    }

    pub fn update(&mut self) {
        if let Some(client_a) = &mut self.client_a {
            if client_a.update() {
                self.client_a = None;
                info!("Closed Client A");
            }
        }
        if let Some(client_b) = &mut self.client_b {
            if client_b.update() {
                self.client_b = None;
                info!("Closed Client B");
            }
        }
    }
}
