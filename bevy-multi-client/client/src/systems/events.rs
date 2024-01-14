
use bevy::prelude::{info, EventReader, ResMut};

use naia_bevy_client::{
    default_channels::UnorderedReliableChannel,
    events::{ConnectEvent, DisconnectEvent, MessageEvents, RejectEvent},
    Client
};
use naia_bevy_client::transport::webrtc::Socket;

use bevy_multi_client_server_b_protocol::messages::Auth as AuthB;

use crate::{app::{Alt, Alt2, ClientName, IsStringMessage}, resources::Global};

pub fn connect_events<T: ClientName>(
    client: Client<T>,
    mut event_reader: EventReader<ConnectEvent<T>>,
) {
    for _ in event_reader.read() {
        let Ok(server_address) = client.server_address() else {
            panic!("Shouldn't happen");
        };
        info!("Client<{}> connected to: {}", T::name(), server_address);
    }
}

pub fn reject_events<T: ClientName>(mut event_reader: EventReader<RejectEvent<T>>) {
    for _ in event_reader.read() {
        info!("Client<{}> rejected from connecting to Server", T::name());
    }
}

pub fn disconnect_events<T: ClientName>(mut event_reader: EventReader<DisconnectEvent<T>>) {
    for _ in event_reader.read() {
        info!("Client<{}> disconnected from Server", T::name());
    }
}

pub fn message_events<T: ClientName, M: IsStringMessage>(
    mut client: Client<T>,
    mut event_reader: EventReader<MessageEvents<T>>,
    mut global: ResMut<Global>,
) {
    for events in event_reader.read() {
        for message in events.read::<UnorderedReliableChannel, M>() {
            let message_contents = message.contents();
            info!("Client<{}> recv <- {}", T::name(), message_contents);

            let new_message_contents = format!("Client<{}> Message ({})", T::name(), T::get_msg_count(&global));
            info!("Client<{}> send -> {}", T::name(), new_message_contents);

            let string_message = M::new(new_message_contents);
            client.send_message::<UnorderedReliableChannel, M>(&string_message);

            T::inc_msg_count(&mut global);
        }
    }
}

pub fn toggle_between_alt_clients(
    mut global: ResMut<Global>,
    mut client_b: Client<Alt>,
    mut client_c: Client<Alt2>,
) {
    match (client_b.connection_status().is_connected(), client_c.connection_status().is_connected()) {
        (true, true) => {
            //info!("All clients connected!");
            let b_count = global.message_count_b;
            let c_count = global.message_count_c;

            let b_is_oldest = b_count > c_count;
            if b_is_oldest {
                if c_count > 5 {
                    info!("-----   Closing Client {}.  -----", Alt::name());

                    client_b.disconnect();
                    global.message_count_b = 0;
                }
            } else {
                if b_count > 5 {
                    info!("-----   Closing Client {}.  -----", Alt2::name());

                    client_c.disconnect();
                    global.message_count_c = 0;
                }
            }
        }

        (true, false) => {
            //info!("Client B connected, C disconnected!");
            if global.message_count_b > 15 {
                if client_c.connection_status().is_disconnected() {

                    info!("-----   Starting Client {}.  -----", Alt2::name());

                    global.message_count_c = 0;

                    let socket = Socket::new("http://127.0.0.1:14195", client_c.socket_config());
                    let auth = AuthB::new("charlie", "12345");

                    client_c.auth(auth);
                    client_c.connect(socket);
                }
            }
        }
        (false, true) => {
            //info!("Client C connected, B disconnected!");
            if global.message_count_c > 15 {
                if client_b.connection_status().is_disconnected() {

                    info!("-----   Starting Client {}.  -----", Alt::name());

                    global.message_count_b = 0;

                    let socket = Socket::new("http://127.0.0.1:14193", client_b.socket_config());
                    let auth = AuthB::new("charlie", "12345");

                    client_b.auth(auth);
                    client_b.connect(socket);
                }
            }
        }
        (false, false) => {
            // info!("All clients disconnected!");
        }
    }
}