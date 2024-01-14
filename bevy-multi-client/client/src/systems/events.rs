
use bevy::prelude::{info, EventReader, ResMut};

use naia_bevy_client::{
    default_channels::UnorderedReliableChannel,
    events::{ConnectEvent, DisconnectEvent, MessageEvents, RejectEvent},
    Client
};

use bevy_multi_client_server_a_protocol::messages::StringMessage;

use crate::{app::ClientName, resources::Global};

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

pub fn message_events<T: ClientName>(
    mut client: Client<T>,
    mut event_reader: EventReader<MessageEvents<T>>,
    mut global: ResMut<Global>,
) {
    for events in event_reader.read() {
        for message in events.read::<UnorderedReliableChannel, StringMessage>() {
            let message_contents = message.contents;
            info!("Client<{}> recv <- {}", T::name(), message_contents);

            let new_message_contents = format!("Client<{}> Message ({})", T::name(), global.message_count);
            info!("Client<{}> send -> {}", T::name(), new_message_contents);

            let string_message = StringMessage::new(new_message_contents);
            client.send_message::<UnorderedReliableChannel, StringMessage>(&string_message);

            global.message_count += 1;
        }
    }
}