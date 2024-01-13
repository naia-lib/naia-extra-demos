
use bevy::prelude::{info, EventReader, ResMut};

use naia_bevy_client::{
    default_channels::UnorderedReliableChannel,
    events::{ConnectEvent, DisconnectEvent, MessageEvents, RejectEvent},
    Client
};

use bevy_multi_client_server_a_protocol::messages::StringMessage;

use crate::resources::{Global, LETTER_A};

pub fn connect_events(
    client: Client,
    mut event_reader: EventReader<ConnectEvent>,
) {
    for _ in event_reader.read() {
        let Ok(server_address) = client.server_address() else {
            panic!("Shouldn't happen");
        };
        info!("Client connected to: {}", server_address);
    }
}

pub fn reject_events(mut event_reader: EventReader<RejectEvent>) {
    for _ in event_reader.read() {
        info!("Client rejected from connecting to Server");
    }
}

pub fn disconnect_events(mut event_reader: EventReader<DisconnectEvent>) {
    for _ in event_reader.read() {
        info!("Client disconnected from Server");
    }
}

pub fn message_events(
    mut client: Client,
    mut event_reader: EventReader<MessageEvents>,
    mut global: ResMut<Global>,
) {
    for events in event_reader.read() {
        for message in events.read::<UnorderedReliableChannel, StringMessage>() {
            let message_contents = message.contents;
            info!("Client {} recv <- {}", LETTER_A, message_contents);

            let new_message_contents = format!("Client {} Message ({})", LETTER_A, global.message_count);
            info!("Client {} send -> {}", LETTER_A, new_message_contents);

            let string_message = StringMessage::new(new_message_contents);
            client.send_message::<UnorderedReliableChannel, StringMessage>(&string_message);

            global.message_count += 1;
        }
    }
}