
cfg_if! {
    if #[cfg(feature = "mquad")] {
        use miniquad::info;
    } else {
        use log::info;
    }
}

use std::marker::PhantomData;

use naia_client::{
    shared::{default_channels::UnorderedReliableChannel, Message, Protocol},
    Client as NaiaClient, ClientConfig, ConnectEvent,
    DisconnectEvent, ErrorEvent, MessageEvent,
    transport::webrtc::Socket,
};

use multi_client_server_b_protocol::{protocol as protocol_b, Auth as AuthB};

use naia_demo_world::{Entity, World};

type Client = NaiaClient<Entity>;

pub trait IsStringMessage: Message {
    fn new(contents: String) -> Self;
    fn contents(&self) -> &String;
}

pub struct ClientRunner<T: IsStringMessage> {
    letter: String,
    client: Client,
    message_count: u32,
    disconnected_count: u32,
    phantom_t: PhantomData<T>,
}

impl<T: IsStringMessage> ClientRunner<T> {
    pub fn new<M: Message>(letter: String, socket: Socket, auth: M, protocol: Protocol) -> Self {

        let mut client = Client::new(ClientConfig::default(), protocol);

        client.auth(auth);
        client.connect(socket);

        Self {
            letter,
            client,
            message_count: 0,
            disconnected_count: 0,
            phantom_t: PhantomData,
        }
    }

    pub fn letter(&self) -> String {
        self.letter.clone()
    }

    pub fn message_count(&self) -> u32 {
        self.message_count
    }

    pub fn update(&mut self, world: &mut World) {
        if self.client.is_disconnected() {
            return;
        }

        let mut events = self.client.receive(world.proxy_mut());

        for server_address in events.read::<ConnectEvent>() {
            info!("Client {} connected to: {}", self.letter, server_address);
        }
        for server_address in events.read::<DisconnectEvent>() {
            info!("Client {} disconnected from: {}", self.letter, server_address);
        }
        for message in events.read::<MessageEvent<UnorderedReliableChannel, T>>() {
            let message_contents = message.contents();
            info!("Client {} recv <- {}", self.letter, message_contents);

            let new_message_contents = format!("Client {} Message ({})", self.letter, self.message_count);
            info!("Client {} send -> {}", self.letter, new_message_contents);

            let string_message = T::new(new_message_contents);
            self.client.send_message::<UnorderedReliableChannel, T>(&string_message);

            self.message_count += 1;
        }
        for error in events.read::<ErrorEvent>() {
            info!("Client {} Error: {}", self.letter, error);
            return;
        }
    }

    pub fn is_connected(&self) -> bool {
        self.client.is_connected()
    }

    pub fn is_disconnected(&self) -> bool {
        self.client.is_disconnected()
    }

    pub fn disconnect(&mut self) {
        self.client.disconnect();
    }

    pub fn disconnect_count(&self) -> u32 {
        self.disconnected_count
    }

    pub fn increment_disconnect_count(&mut self) {
        self.disconnected_count += 1;
    }

    pub fn connect_to_server_b(&mut self) {
        self.letter = "B".to_string();
        self.message_count = 0;
        self.disconnected_count = 0;

        let protocol = protocol_b();
        let socket_config = protocol.socket.clone();
        let socket = Socket::new("http://127.0.0.1:14193", &socket_config);
        let auth = AuthB::new("charlie", "12345");

        self.client.auth(auth);
        self.client.connect(socket);
    }

    pub fn connect_to_server_c(&mut self) {
        self.letter = "C".to_string();
        self.message_count = 0;
        self.disconnected_count = 0;

        let protocol = protocol_b();
        let socket_config = protocol.socket.clone();
        let socket = Socket::new("http://127.0.0.1:14195", &socket_config);
        let auth = AuthB::new("charlie", "12345");

        self.client.auth(auth);
        self.client.connect(socket);
    }
}