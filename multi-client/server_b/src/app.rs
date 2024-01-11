use std::{thread::sleep, time::Duration};

use naia_server::{
    shared::default_channels::UnorderedReliableChannel, transport::webrtc, AuthEvent, ConnectEvent,
    DisconnectEvent, ErrorEvent, MessageEvent, Server as NaiaServer, ServerConfig,
    TickEvent,
};

use naia_demo_world::{Entity, World};

use multi_client_server_b_protocol::{protocol, Auth, StringMessage};

type Server = NaiaServer<Entity>;

pub struct App {
    server: Server,
    world: World,
    tick_count: u32,
}

impl App {
    pub fn new() -> Self {
        info!("Multi-Client Server B started");

        let world = World::default();

        let server_addresses = webrtc::ServerAddrs::new(
            "127.0.0.1:14193"
                .parse()
                .expect("could not parse Signaling address/port"),
            // IP Address to listen on for UDP WebRTC data channels
            "127.0.0.1:14194"
                .parse()
                .expect("could not parse WebRTC data address/port"),
            // The public WebRTC IP address to advertise
            "http://127.0.0.1:14194",
        );
        let protocol = protocol();
        let socket = webrtc::Socket::new(&server_addresses, &protocol.socket);
        let mut server = Server::new(ServerConfig::default(), protocol);
        server.listen(socket);

        App {
            server,
            world,
            tick_count: 0,
        }
    }

    pub fn update(&mut self) {
        let mut events = self.server.receive(self.world.proxy_mut());
        if events.is_empty() {
            // If we don't sleep here, app will loop at 100% CPU until a new message comes in
            sleep(Duration::from_millis(3));
            return;
        } else {
            for (user_key, auth) in events.read::<AuthEvent<Auth>>() {
                if auth.username == "charlie" && auth.password == "12345" {
                    // Accept incoming connection
                    self.server.accept_connection(&user_key);
                } else {
                    // Reject incoming connection
                    self.server.reject_connection(&user_key);
                }
            }
            for user_key in events.read::<ConnectEvent>() {
                info!(
                    "Server B connected to: {}",
                    self.server.user(&user_key).address()
                );
            }
            for (_user_key, user) in events.read::<DisconnectEvent>() {
                info!("Server B disconnected from: {:?}", user.address);
            }
            for (user_key, message) in
                events.read::<MessageEvent<UnorderedReliableChannel, StringMessage>>()
            {
                let message_contents = &(*message.contents);
                info!(
                    "Server B recv from ({}) <- {}",
                    self.server.user(&user_key).address(),
                    message_contents
                );
            }
            for _ in events.read::<TickEvent>() {
                // All game logic should happen here, on a tick event

                // Message sending
                for user_key in self.server.user_keys() {
                    let new_message_contents = format!("Server B Message ({})", self.tick_count);
                    info!(
                        "Server B send to   ({}) -> {}",
                        self.server.user(&user_key).address(),
                        new_message_contents
                    );

                    let new_message = StringMessage::new(new_message_contents);
                    self.server
                        .send_message::<UnorderedReliableChannel, _>(&user_key, &new_message);
                }

                // VERY IMPORTANT! Calling this actually sends all update data
                // packets to all Clients that require it. If you don't call this
                // method, the Server will never communicate with it's connected Clients
                self.server.send_all_updates(self.world.proxy());

                self.tick_count = self.tick_count.wrapping_add(1);
            }
            for error in events.read::<ErrorEvent>() {
                info!("Server B Error: {}", error);
            }
        }
    }
}
