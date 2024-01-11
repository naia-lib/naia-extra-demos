
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
            phantom_t: PhantomData,
        }
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
}