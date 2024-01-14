
use bevy::prelude::{info, EventReader, Local};

use naia_bevy_client::{
    default_channels::UnorderedReliableChannel,
    events::{ConnectEvent, DisconnectEvent, MessageEvents, RejectEvent},
    Client
};

use crate::app::{ClientName, IsStringMessage};

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
    mut message_count: Local<u32>,
) {
    for events in event_reader.read() {
        for message in events.read::<UnorderedReliableChannel, M>() {
            let message_contents = message.contents();
            info!("Client<{}> recv <- {}", T::name(), message_contents);

            let new_message_contents = format!("Client<{}> Message ({})", T::name(), *message_count);
            info!("Client<{}> send -> {}", T::name(), new_message_contents);

            let string_message = M::new(new_message_contents);
            client.send_message::<UnorderedReliableChannel, M>(&string_message);

            *message_count += 1;
        }
    }
}