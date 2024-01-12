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
    client_runner_b: Option<ClientRunner<StringMessageB>>,
    client_runner_c: Option<ClientRunner<StringMessageB>>,
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
        let client_runner_b = init_runner_for_server_b();

        App {
            world: World::default(),
            client_runner_a,
            client_runner_b: Some(client_runner_b),
            client_runner_c: None,
        }
    }

    pub fn update(&mut self) {
        self.client_runner_a.update(&mut self.world);

        if let Some(client_runner_b) = &mut self.client_runner_b {
            client_runner_b.update(&mut self.world);
            if client_runner_b.message_count() > 10 {
                info!("-----   Closing Client B.   -----");
                self.client_runner_b = None;
                info!("-----   Starting Client C.  -----");
                self.client_runner_c = Some(init_runner_for_server_c());
            }
        }

        if let Some(client_runner_c) = &mut self.client_runner_c {
            client_runner_c.update(&mut self.world);
            if client_runner_c.message_count() > 10 {
                info!("-----   Closing Client C.   -----");
                self.client_runner_c = None;
                info!("-----   Starting Client B.  -----");
                self.client_runner_b = Some(init_runner_for_server_b());
            }
        }
    }
}

fn init_runner_for_server_b() -> ClientRunner::<StringMessageB> {
    let protocol = protocol_b();
    let socket_config = protocol.socket.clone();
    let socket = webrtc::Socket::new("http://127.0.0.1:14193", &socket_config);
    let auth = AuthB::new("charlie", "12345");

    ClientRunner::<StringMessageB>::new("B".to_string(), socket, auth, protocol)
}

fn init_runner_for_server_c() -> ClientRunner::<StringMessageB> {
    let protocol = protocol_b();
    let socket_config = protocol.socket.clone();
    let socket = webrtc::Socket::new("http://127.0.0.1:14195", &socket_config);
    let auth = AuthB::new("charlie", "12345");

    ClientRunner::<StringMessageB>::new("C".to_string(), socket, auth, protocol)
}
