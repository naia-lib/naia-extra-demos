cfg_if! {
    if #[cfg(feature = "mquad")] {
        use miniquad::info;
    } else {
        use log::info;
    }
}

use naia_client::transport::webrtc;

use naia_demo_world::World;

use multi_client_server_a_protocol::{protocol as protocol_a, Auth as AuthA, StringMessage as StringMessageA};
use multi_client_server_b_protocol::{protocol as protocol_b, Auth as AuthB, StringMessage as StringMessageB};

use crate::client_runner::{IsStringMessage, ClientRunner};

impl IsStringMessage for StringMessageA {
    fn new(contents: String) -> Self {
        Self::new(contents)
    }
    fn contents(&self) -> &String {
        &self.contents
    }
}

impl IsStringMessage for StringMessageB {
    fn new(contents: String) -> Self {
        Self::new(contents)
    }
    fn contents(&self) -> &String {
        &self.contents
    }
}

pub struct App {
    world: World,

    client_runner_a: ClientRunner<StringMessageA>,
    client_runner_b: ClientRunner<StringMessageB>,
}

impl App {
    pub fn default() -> Self {
        info!("Multi-Client Demo Client started");

        // Client Runner A
        let client_runner_a = {
            let protocol = protocol_a();
            let socket_config = protocol.socket.clone();
            let socket = webrtc::Socket::new("http://127.0.0.1:14191", &socket_config);

            let auth = AuthA::new("charlie", "12345");

            ClientRunner::<StringMessageA>::new("A".to_string(), socket, auth, protocol)
        };

        // Client Runner B
        let client_runner_b = {
            let protocol = protocol_b();
            let socket_config = protocol.socket.clone();
            let socket = webrtc::Socket::new("http://127.0.0.1:14193", &socket_config);
            let auth = AuthB::new("charlie", "12345");

            ClientRunner::<StringMessageB>::new("B".to_string(), socket, auth, protocol)
        };

        App {
            world: World::default(),
            client_runner_a,
            client_runner_b,
        }
    }

    pub fn update(&mut self) {
        self.client_runner_a.update(&mut self.world);

        self.client_runner_b.update(&mut self.world);
        if self.client_runner_b.message_count() > 3 {
            if self.client_runner_b.is_connected() {
                let letter = self.client_runner_b.letter();
                info!("-----   Closing Client {}.   -----", letter);
                self.client_runner_b.disconnect();
            } else {
                if self.client_runner_b.is_disconnected() {
                    if self.client_runner_b.disconnect_count() > 1000 {
                        let letter = self.client_runner_b.letter();
                        match letter.as_str() {
                            "B" => {
                                info!("-----   Starting Client C.  -----");
                                self.client_runner_b.connect_to_server_c();
                            }
                            "C" => {
                                info!("-----   Starting Client B.  -----");
                                self.client_runner_b.connect_to_server_b();
                            }
                            _ => {
                                panic!("Unknown letter: {}", letter);
                            }
                        }
                    } else {
                        self.client_runner_b.increment_disconnect_count();
                        // if self.client_runner_b.disconnect_count() % 100 == 0 {
                        //     info!(".");
                        // }
                    }
                } else {
                    info!(".not disconnected yet.");
                }
            }
        }
    }
}
