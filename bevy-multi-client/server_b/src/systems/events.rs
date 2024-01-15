use bevy_ecs::{
    event::EventReader, system::{Local, Query},
};
use bevy_log::info;

use naia_bevy_server::{
    default_channels::UnorderedReliableChannel,
    events::{
        AuthEvents, ConnectEvent, DisconnectEvent, ErrorEvent,
        TickEvent, MessageEvents
    },
    Server,
};

use bevy_multi_client_server_b_protocol::{messages::{Auth, StringMessage}, MyComponent};

use crate::SERVER_LETTER;

pub fn auth_events(mut server: Server, mut event_reader: EventReader<AuthEvents>) {
    for events in event_reader.read() {
        for (user_key, auth) in events.read::<Auth>() {
            if auth.username == "charlie" && auth.password == "12345" {
                // Accept incoming connection
                server.accept_connection(&user_key);
            } else {
                // Reject incoming connection
                server.reject_connection(&user_key);
            }
        }
    }
}

pub fn connect_events(
    mut server: Server,
    mut event_reader: EventReader<ConnectEvent>,
) {
    for ConnectEvent(user_key) in event_reader.read() {
        let address = server
            .user_mut(user_key)
            // Get User's address for logging
            .address();

        info!("Server {} connected to: {}", SERVER_LETTER, address);
    }
}

pub fn disconnect_events(
    mut event_reader: EventReader<DisconnectEvent>,
) {
    for DisconnectEvent(_user_key, user) in event_reader.read() {
        info!("Server {} disconnected from: {:?}", SERVER_LETTER, user.address);
    }
}

pub fn error_events(mut event_reader: EventReader<ErrorEvent>) {
    for ErrorEvent(error) in event_reader.read() {
        info!("Server {} Error: {:?}", SERVER_LETTER, error);
    }
}

pub fn tick_events(
    mut server: Server,
    mut tick_reader: EventReader<TickEvent>,
    mut tick_count: Local<u32>,
    mut component_q: Query<&mut MyComponent>,
) {
    let mut has_ticked = false;

    for TickEvent(_server_tick) in tick_reader.read() {
        has_ticked = true;
    }

    if has_ticked {
        // Send a message to all connected clients

        for user_key in server.user_keys() {
            let new_message_contents = format!("Server {} Message({})", SERVER_LETTER, *tick_count);

            info!(
                "Server {} send to ({}) -> {}",
                SERVER_LETTER,
                server.user(&user_key).address(),
                new_message_contents
            );

            let new_message = StringMessage::new(new_message_contents);
            server.send_message::<UnorderedReliableChannel, StringMessage>(&user_key, &new_message);
        }

        *tick_count += 1;

        // Update scopes of entities
        for (_, user_key, entity) in server.scope_checks() {

            if let Ok(mut component) = component_q.get_mut(entity) {
                *component.x += 1;

                if *component.x > 10 {
                    *component.x = 0;
                }

                info!("Server {} updated MyComponent to x: {}", SERVER_LETTER, *component.x);

                if *component.x > 3 && *component.x < 7 {
                    server.user_scope(&user_key).include(&entity);
                } else {
                    server.user_scope(&user_key).exclude(&entity);
                }
            }
        }
    }
}

pub fn message_events(
    mut event_reader: EventReader<MessageEvents>,
) {
    for events in event_reader.read() {
        for (_user_key, message) in events.read::<UnorderedReliableChannel, StringMessage>() {
            let message_contents = message.contents;
            info!("Server {} recv <- {}", SERVER_LETTER, message_contents);
        }
    }
}