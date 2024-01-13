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
    client_runner_c: ClientRunner<StringMessageB>,
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

        // Client Runner C
        let client_runner_c = {
            let protocol = protocol_b();
            let mut client_runner = ClientRunner::<StringMessageB>::new("C".to_string(), protocol);
            // don't connect this one just yet

            client_runner
        };

        App {
            world: World::default(),
            client_runner_a,
            client_runner_b,
            client_runner_c,
        }
    }

    pub fn update(&mut self) {
        self.client_runner_a.update(&mut self.world);
        self.client_runner_b.update(&mut self.world);
        self.client_runner_c.update(&mut self.world);

        match (self.client_runner_b.is_connected(), self.client_runner_c.is_connected()) {
            (true, true) => {
                //info!("All clients connected!");
                let b_count = self.client_runner_b.message_count();
                let c_count = self.client_runner_c.message_count();

                let b_is_oldest = b_count > c_count;
                if b_is_oldest {
                    if c_count > 5 {
                        self.client_runner_b.disconnect();
                    }
                } else {
                    if b_count > 5 {
                        self.client_runner_c.disconnect();
                    }
                }
            }

            (true, false) => {
                //info!("Client B connected, C disconnected!");
                if self.client_runner_b.message_count() > 15 {
                    if self.client_runner_c.is_disconnected() {
                        self.client_runner_c.connect_to_server_c();
                    }
                }
            }
            (false, true) => {
                //info!("Client C connected, B disconnected!");
                if self.client_runner_c.message_count() > 15 {
                    if self.client_runner_b.is_disconnected() {
                        self.client_runner_b.connect_to_server_b();
                    }
                }
            }
            (false, false) => {
                // info!("All clients disconnected!");
            }
        }

        if self.client_runner_b.is_connected() {

        } else {
            if self.client_runner_c.is_connected() {
                if self.client_runner_b.is_disconnected() {
                    let runner_c_count = self.client_runner_c.message_count();
                    if runner_c_count > 15 {
                        self.client_runner_b.connect_to_server_b();
                    }
                }
            }
        }
    }
}
