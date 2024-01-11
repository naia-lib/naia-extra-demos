cfg_if! {
    if #[cfg(feature = "mquad")] {
        use miniquad::info;
    } else {
        use log::info;
    }
}

use naia_client::{
    shared::{default_channels::UnorderedReliableChannel, SocketConfig},
    transport::webrtc,
    Client as NaiaClient, ClientConfig, ClientTickEvent, ConnectEvent, DespawnEntityEvent,
    DisconnectEvent, ErrorEvent, MessageEvent, RejectEvent, RemoveComponentEvent, SpawnEntityEvent,
    UpdateComponentEvent,
};

use naia_demo_world::{Entity, World};

use multi_client_server_a_protocol::{protocol, Auth, Character, StringMessage};

type Client = NaiaClient<Entity>;

pub struct App {
    client: Client,
    world: World,
    message_count: u32,
    socket_config: SocketConfig,
}

impl App {
    pub fn default() -> Self {
        info!("Multi-Client Demo Client started");

        let protocol = protocol();
        let socket_config = protocol.socket.clone();
        let socket = webrtc::Socket::new("http://127.0.0.1:14191", &socket_config);
        let mut client = Client::new(ClientConfig::default(), protocol);

        let auth = Auth::new("charlie", "12345");
        client.auth(auth);

        client.connect(socket);

        App {
            client,
            world: World::default(),
            message_count: 0,
            socket_config,
        }
    }

    pub fn update(&mut self) {
        if self.client.is_disconnected() {
            return;
        }

        let mut events = self.client.receive(self.world.proxy_mut());

        for server_address in events.read::<ConnectEvent>() {
            info!("Client connected to: {}", server_address);
        }
        for server_address in events.read::<DisconnectEvent>() {
            info!("Client disconnected from: {}", server_address);
        }
        for message in events.read::<MessageEvent<UnorderedReliableChannel, StringMessage>>() {
            let message_contents = &(*message.contents);
            info!("Client recv <- {}", message_contents);

            self.message_count += 1;
        }
        for error in events.read::<ErrorEvent>() {
            info!("Client Error: {}", error);
            return;
        }
    }
}
