cfg_if! {
    if #[cfg(feature = "mquad")] {
        use miniquad::info;
    } else {
        use log::info;
    }
}

use naia_client::{
    shared::default_channels::UnorderedReliableChannel,
    transport::webrtc,
    Client as NaiaClient, ClientConfig, ConnectEvent,
    DisconnectEvent, ErrorEvent, MessageEvent,
};

use naia_demo_world::{Entity, World};

use multi_client_server_a_protocol::{protocol as protocol_a, Auth as AuthA, StringMessage as StringMessageA};
use multi_client_server_b_protocol::{protocol as protocol_b, Auth as AuthB, StringMessage as StringMessageB};

type Client = NaiaClient<Entity>;

pub struct App {
    world: World,

    client_a: Client,
    message_count_a: u32,

    client_b: Client,
    message_count_b: u32,
}

impl App {
    pub fn default() -> Self {
        info!("Multi-Client Demo Client started");

        // Client A
        let client_a = {
            let protocol = protocol_a();
            let socket_config = protocol.socket.clone();
            let socket = webrtc::Socket::new("http://127.0.0.1:14191", &socket_config);
            let mut client = Client::new(ClientConfig::default(), protocol);

            let auth = AuthA::new("charlie", "12345");
            client.auth(auth);

            client.connect(socket);
            client
        };

        // Client B
        let client_b = {
            let protocol = protocol_b();
            let socket_config = protocol.socket.clone();
            let socket = webrtc::Socket::new("http://127.0.0.1:14193", &socket_config);
            let mut client = Client::new(ClientConfig::default(), protocol);

            let auth = AuthB::new("charlie", "12345");
            client.auth(auth);

            client.connect(socket);
            client
        };

        App {
            world: World::default(),
            client_a,
            message_count_a: 0,
            client_b,
            message_count_b: 0,
        }
    }

    pub fn update(&mut self) {
        self.update_a();
        self.update_b();
    }

    fn update_a(&mut self) {
        if self.client_a.is_disconnected() {
            return;
        }

        let mut events = self.client_a.receive(self.world.proxy_mut());

        for server_address in events.read::<ConnectEvent>() {
            info!("Client A connected to: {}", server_address);
        }
        for server_address in events.read::<DisconnectEvent>() {
            info!("Client A disconnected from: {}", server_address);
        }
        for message in events.read::<MessageEvent<UnorderedReliableChannel, StringMessageA>>() {
            let message_contents = &(*message.contents);
            info!("Client A recv <- {}", message_contents);

            let new_message_contents = format!("Client A Message ({})", self.message_count_a);
            info!("Client A send -> {}", new_message_contents);

            let string_message = StringMessageA::new(new_message_contents);
            self.client_a.send_message::<UnorderedReliableChannel, StringMessageA>(&string_message);

            self.message_count_a += 1;
        }
        for error in events.read::<ErrorEvent>() {
            info!("Client A Error: {}", error);
            return;
        }
    }

    fn update_b(&mut self) {
        if self.client_b.is_disconnected() {
            return;
        }

        let mut events = self.client_b.receive(self.world.proxy_mut());

        for server_address in events.read::<ConnectEvent>() {
            info!("Client B connected to: {}", server_address);
        }
        for server_address in events.read::<DisconnectEvent>() {
            info!("Client B disconnected from: {}", server_address);
        }
        for message in events.read::<MessageEvent<UnorderedReliableChannel, StringMessageB>>() {
            let message_contents = &(*message.contents);
            info!("Client B recv <- {}", message_contents);

            let new_message_contents = format!("Client B Message ({})", self.message_count_b);
            info!("Client B send -> {}", new_message_contents);

            let string_message = StringMessageB::new(new_message_contents);
            self.client_b.send_message::<UnorderedReliableChannel, StringMessageB>(&string_message);

            self.message_count_b += 1;
        }
        for error in events.read::<ErrorEvent>() {
            info!("Client B Error: {}", error);
            return;
        }
    }
}
