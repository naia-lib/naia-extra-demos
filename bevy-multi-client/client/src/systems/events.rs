
use bevy::prelude::{info, EventReader, Local};

use naia_bevy_client::{
    default_channels::UnorderedReliableChannel,
    events::{ConnectEvent, DisconnectEvent, MessageEvents, RejectEvent},
    Client
};

use bevy_multi_client_server_a_protocol::messages::{StringMessage as StringMessageA};
use bevy_multi_client_server_b_protocol::messages::{StringMessage as StringMessageB};

use crate::app::{ClientName, Alt, Main};

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

pub fn message_events_main(
    mut client: Client<Main>,
    mut event_reader: EventReader<MessageEvents<Main>>,
    mut message_count: Local<u32>,
) {
    for events in event_reader.read() {
        for message in events.read::<UnorderedReliableChannel, StringMessageA>() {
            let message_contents = message.contents;
            info!("Client<{}> recv <- {}", Main::name(), message_contents);

            let new_message_contents = format!("Client<{}> Message ({})", Main::name(), *message_count);
            info!("Client<{}> send -> {}", Main::name(), new_message_contents);

            let string_message = StringMessageA::new(new_message_contents);
            client.send_message::<UnorderedReliableChannel, StringMessageA>(&string_message);

            *message_count += 1;
        }
    }
}

pub fn message_events_alt(
    mut client: Client<Alt>,
    mut event_reader: EventReader<MessageEvents<Alt>>,
    mut message_count: Local<u32>,
) {
    for events in event_reader.read() {
        for message in events.read::<UnorderedReliableChannel, StringMessageB>() {
            let message_contents = message.contents;
            info!("Client<{}> recv <- {}", Alt::name(), message_contents);

            let new_message_contents = format!("Client<{}> Message ({})", Alt::name(), *message_count);
            info!("Client<{}> send -> {}", Alt::name(), new_message_contents);

            let string_message = StringMessageB::new(new_message_contents);
            client.send_message::<UnorderedReliableChannel, StringMessageB>(&string_message);

            *message_count += 1;
        }
    }
}