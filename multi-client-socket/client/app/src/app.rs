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

use multi_client_socket_shared_a::{shared_config_a, PING_MSG_A, PONG_MSG_A};
use multi_client_socket_shared_b::{shared_config_b, PING_MSG_B, PONG_MSG_B};

pub struct App {
    packet_sender_a: Box<dyn PacketSender>,
    packet_receiver_a: Box<dyn PacketReceiver>,
    message_count_a: u8,
    timer_a: Timer,
    server_addr_str_a: Option<String>,

    packet_sender_b: Box<dyn PacketSender>,
    packet_receiver_b: Box<dyn PacketReceiver>,
    message_count_b: u8,
    timer_b: Timer,
    server_addr_str_b: Option<String>,
}

impl App {
    pub fn new() -> App {
        info!("Multi-Client Socket Client started");

        let (packet_sender_a, packet_receiver_a) =
            Socket::connect("http://127.0.0.1:14191", &shared_config_a());

        let (packet_sender_b, packet_receiver_b) =
            Socket::connect("http://127.0.0.1:14193", &shared_config_b());

        App {
            packet_sender_a,
            packet_receiver_a,
            message_count_a: 0,
            timer_a: Timer::new(Duration::from_secs(1)),
            server_addr_str_a: None,

            packet_sender_b,
            packet_receiver_b,
            message_count_b: 0,
            timer_b: Timer::new(Duration::from_secs(1)),
            server_addr_str_b: None,
        }
    }

    pub fn update(&mut self) {

        // Server A
        if self.server_addr_str_a.is_none() {
            if let ServerAddr::Found(addr) = self.packet_receiver_a.server_addr() {
                self.server_addr_str_a = Some(addr.to_string());
            }
        }

        match self.packet_receiver_a.receive() {
            Ok(Some(packet)) => {
                let message_from_server = String::from_utf8_lossy(packet);

                info!(
                    "Client recv <- {} (A): {}",
                    self.server_addr_str_a.as_ref().unwrap_or(&"".to_string()),
                    message_from_server
                );

                if message_from_server.eq(PONG_MSG_A) {
                    self.message_count_a += 1;

                    if self.message_count_a == 10 {
                        info!("Client finished sending messages to A");
                    }
                }
            }
            Ok(None) => {
                if self.message_count_a < 10 {
                    if self.timer_a.ringing() {
                        self.timer_a.reset();

                        let message_to_server: String = PING_MSG_A.to_string();

                        let server_addr = match self.packet_receiver_a.server_addr() {
                            ServerAddr::Found(addr) => addr.to_string(),
                            _ => "".to_string(),
                        };
                        info!("Client send -> {} (A): {}", server_addr, message_to_server);

                        match self.packet_sender_a.send(message_to_server.as_bytes()) {
                            Ok(()) => {}
                            Err(error) => {
                                info!("Client Send Error (to A): {}", error);
                            }
                        }
                    }
                }
            }
            Err(err) => {
                info!("Client Error (from A): {}", err);
            }
        }

        // Server B
        if self.server_addr_str_b.is_none() {
            if let ServerAddr::Found(addr) = self.packet_receiver_b.server_addr() {
                self.server_addr_str_b = Some(addr.to_string());
            }
        }

        match self.packet_receiver_b.receive() {
            Ok(Some(packet)) => {
                let message_from_server = String::from_utf8_lossy(packet);

                info!(
                    "Client recv <- {} (B): {}",
                    self.server_addr_str_b.as_ref().unwrap_or(&"".to_string()),
                    message_from_server
                );

                if message_from_server.eq(PONG_MSG_B) {
                    self.message_count_b += 1;

                    if self.message_count_b == 10 {
                        info!("Client finished sending messages to B");
                    }
                }
            }
            Ok(None) => {
                if self.message_count_b < 10 {
                    if self.timer_b.ringing() {
                        self.timer_b.reset();

                        let message_to_server: String = PING_MSG_B.to_string();

                        let server_addr = match self.packet_receiver_b.server_addr() {
                            ServerAddr::Found(addr) => addr.to_string(),
                            _ => "".to_string(),
                        };
                        info!("Client send -> {} (B): {}", server_addr, message_to_server);

                        match self.packet_sender_b.send(message_to_server.as_bytes()) {
                            Ok(()) => {}
                            Err(error) => {
                                info!("Client Send Error (to B): {}", error);
                            }
                        }
                    }
                }
            }
            Err(err) => {
                info!("Client Error (from B): {}", err);
            }
        }
    }
}
