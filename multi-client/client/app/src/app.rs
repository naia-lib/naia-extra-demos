cfg_if! {
    if #[cfg(feature = "mquad")] {
        use miniquad::info;
    } else {
        use log::info;
    }
}

use naia_demo_world::World;

use multi_client_server_a_protocol::{protocol as protocol_a, StringMessage as StringMessageA};
use multi_client_server_b_protocol::{protocol as protocol_b, StringMessage as StringMessageB};

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
            let mut client_runner = ClientRunner::<StringMessageA>::new("A".to_string(), protocol);
            client_runner.connect_to_server_a();

            client_runner
        };

        // Client Runner B
        let client_runner_b = {
            let protocol = protocol_b();
            let mut client_runner = ClientRunner::<StringMessageB>::new("B".to_string(), protocol);
            client_runner.connect_to_server_b();

            client_runner
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
                    info!(".not disconnected yet.");
                }
            }
        }
    }
}
