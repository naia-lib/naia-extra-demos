use std::default::Default;

use bevy::sprite::Anchor;
use bevy::{
    prelude::{
        info, BuildChildren, Color as BevyColor, Commands, DespawnRecursiveExt, Entity,
        EventReader, Query, Res, ResMut, Sprite, SpriteBundle, Transform, TransformBundle, Vec2,
        VisibilityBundle,
    },
    sprite::MaterialMesh2dBundle,
};

use naia_bevy_client::{events::{
    ClientTickEvent, ConnectEvent, DespawnEntityEvent, DisconnectEvent, InsertComponentEvents,
    MessageEvents, RejectEvent, RemoveComponentEvents, SpawnEntityEvent, UpdateComponentEvents,
}, sequence_greater_than, Client, CommandsExt, Random, Replicate, Tick, ReplicationConfig};

use bevy_entity_relations_shared::{
    behavior as shared_behavior,
    channels::{EntityAssignmentChannel, PlayerCommandChannel},
    components::{Color, ColorValue, Position, Relation, Shape, ShapeValue},
    messages::{EntityAssignment, KeyCommand},
};

use crate::{
    components::{Confirmed, Interp, Line, LocalCursor, Predicted},
    resources::{Global, OwnedEntity},
};

const SQUARE_SIZE: f32 = 32.0;

pub fn connect_events(
    mut commands: Commands,
    mut client: Client,
    mut global: ResMut<Global>,
    mut event_reader: EventReader<ConnectEvent>,
) {
    for _ in event_reader.iter() {
        let Ok(server_address) = client.server_address() else {
            panic!("Shouldn't happen");
        };
        info!("Client connected to: {}", server_address);

        // Create entity for Client-authoritative Cursor

        // Spawn Cursor Entity
        let cursor_entity = commands
            // Spawn new Square Entity
            .spawn_empty()
            // MUST call this to begin replication
            .enable_replication(&mut client)
            // make this Entity public, which means that it will be replicated to all other clients
            .configure_replication(ReplicationConfig::Public)
            // Insert Position component
            .insert(Position::new(
                16 * ((Random::gen_range_u32(0, 40) as i16) - 20),
                16 * ((Random::gen_range_u32(0, 30) as i16) - 15),
            ))
            // Insert Shape component
            .insert(Shape::new(ShapeValue::Circle))
            // Insert Cursor marker component
            .insert(LocalCursor)
            // return Entity id
            .id();

        global.cursor_entity = Some(cursor_entity);
    }
}

pub fn reject_events(mut event_reader: EventReader<RejectEvent>) {
    for _ in event_reader.iter() {
        info!("Client rejected from connecting to Server");
    }
}

pub fn disconnect_events(mut event_reader: EventReader<DisconnectEvent>) {
    for _ in event_reader.iter() {
        info!("Client disconnected from Server");
    }
}

pub fn message_events(
    mut commands: Commands,
    client: Client,
    mut global: ResMut<Global>,
    mut event_reader: EventReader<MessageEvents>,
    position_query: Query<&Position>,
    color_query: Query<&Color>,
) {
    for events in event_reader.iter() {
        for message in events.read::<EntityAssignmentChannel, EntityAssignment>() {
            let assign = message.assign;

            let entity = message.entity.get(&client).unwrap();
            if assign {
                info!("gave ownership of entity");

                // Here we create a local copy of the Player entity, to use for client-side prediction
                if let Ok(position) = position_query.get(entity) {
                    let prediction_entity = commands
                        .entity(entity)
                        .local_duplicate() // copies all Replicate components as well
                        .insert(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                                color: BevyColor::WHITE,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..Default::default()
                        })
                        // insert interpolation component
                        .insert(Interp::new(*position.x, *position.y))
                        // mark as predicted
                        .insert(Predicted)
                        .id();

                    global.owned_entity = Some(OwnedEntity::new(entity, prediction_entity));
                }
                // Now that we know the Color of the player, we assign it to our Cursor
                if let Ok(color) = color_query.get(entity) {
                    if let Some(cursor_entity) = global.cursor_entity {
                        // Add Color to cursor entity
                        commands.entity(cursor_entity).insert(color.clone());

                        // Insert SpriteBundle locally only
                        let color_handle = {
                            match *color.value {
                                ColorValue::Red => &global.red,
                                ColorValue::Blue => &global.blue,
                                ColorValue::Yellow => &global.yellow,
                                ColorValue::Green => &global.green,
                                ColorValue::White => &global.white,
                                ColorValue::Purple => &global.purple,
                                ColorValue::Orange => &global.orange,
                                ColorValue::Aqua => &global.aqua,
                            }
                        };
                        commands.entity(cursor_entity).insert(MaterialMesh2dBundle {
                            mesh: global.circle.clone().into(),
                            material: color_handle.clone(),
                            transform: Transform::from_xyz(0.0, 0.0, 0.0),
                            ..Default::default()
                        });
                        info!("assigned color to cursor");
                    }
                }
            } else {
                let mut disowned: bool = false;
                if let Some(owned_entity) = &global.owned_entity {
                    if owned_entity.confirmed == entity {
                        commands.entity(owned_entity.predicted).despawn();
                        disowned = true;
                    }
                }
                if disowned {
                    info!("removed ownership of entity");
                    global.owned_entity = None;
                }
            }
        }
    }
}

pub fn spawn_entity_events(mut event_reader: EventReader<SpawnEntityEvent>) {
    for SpawnEntityEvent(_entity) in event_reader.iter() {
        info!("spawned entity");
    }
}

pub fn despawn_entity_events(mut event_reader: EventReader<DespawnEntityEvent>) {
    for DespawnEntityEvent(_entity) in event_reader.iter() {
        info!("despawned entity");
    }
}

pub fn insert_component_events(
    mut commands: Commands,
    mut event_reader: EventReader<InsertComponentEvents>,
    client: Client,
    global: Res<Global>,
    sprite_query: Query<(&Shape, &Color)>,
    position_query: Query<&Position>,
    relation_query: Query<&Relation>,
) {
    for events in event_reader.iter() {
        for entity in events.read::<Color>() {
            // When we receive a replicated Color component for a given Entity,
            // use that value to also insert a local-only SpriteBundle component into this entity
            info!("add Color Component to entity");

            if let Ok((shape, color)) = sprite_query.get(entity) {
                match *shape.value {
                    // Square
                    ShapeValue::Square => {
                        let color = {
                            match *color.value {
                                ColorValue::Red => BevyColor::RED,
                                ColorValue::Blue => BevyColor::BLUE,
                                ColorValue::Yellow => BevyColor::YELLOW,
                                ColorValue::Green => BevyColor::GREEN,
                                ColorValue::White => BevyColor::WHITE,
                                ColorValue::Purple => BevyColor::PURPLE,
                                ColorValue::Orange => BevyColor::ORANGE,
                                ColorValue::Aqua => BevyColor::AQUAMARINE,
                            }
                        };

                        commands
                            .entity(entity)
                            .insert(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                                    color,
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                                ..Default::default()
                            })
                            // mark as confirmed
                            .insert(Confirmed);
                    }
                    // Circle
                    ShapeValue::Circle => {
                        let handle = {
                            match *color.value {
                                ColorValue::Red => &global.red,
                                ColorValue::Blue => &global.blue,
                                ColorValue::Yellow => &global.yellow,
                                ColorValue::Green => &global.green,
                                ColorValue::White => &global.white,
                                ColorValue::Purple => &global.purple,
                                ColorValue::Orange => &global.orange,
                                ColorValue::Aqua => &global.aqua,
                            }
                        };
                        commands
                            .entity(entity)
                            .insert(MaterialMesh2dBundle {
                                mesh: global.circle.clone().into(),
                                material: handle.clone(),
                                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                                ..Default::default()
                            })
                            // mark as confirmed
                            .insert(Confirmed);
                    }
                    // Circle
                    ShapeValue::BigCircle => {
                        let handle = {
                            match *color.value {
                                ColorValue::Red => &global.red,
                                ColorValue::Blue => &global.blue,
                                ColorValue::Yellow => &global.yellow,
                                ColorValue::Green => &global.green,
                                ColorValue::White => &global.white,
                                ColorValue::Purple => &global.purple,
                                ColorValue::Orange => &global.orange,
                                ColorValue::Aqua => &global.aqua,
                            }
                        };
                        commands
                            .entity(entity)
                            .insert(MaterialMesh2dBundle {
                                mesh: global.big_circle.clone().into(),
                                material: handle.clone(),
                                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                                ..Default::default()
                            })
                            // mark as confirmed
                            .insert(Confirmed);
                    }
                }
            }
        }
        for entity in events.read::<Position>() {
            info!("add Position Component to entity");
            if let Ok(position) = position_query.get(entity) {
                // initialize interpolation
                commands
                    .entity(entity)
                    .insert(Interp::new(*position.x, *position.y));
            }
        }
        for square_entity in events.read::<Relation>() {
            info!("add Relation Component to entity");
            if let Ok(relation) = relation_query.get(square_entity) {
                let baseline_entity = relation.entity.get(&client);
                info!("spawn connecting line");
                // initialize connecting line
                commands
                    .spawn(TransformBundle::default())
                    .insert(VisibilityBundle::default())
                    .insert(Line::new(square_entity, baseline_entity))
                    .with_children(|parent| {
                        parent.spawn(SpriteBundle {
                            sprite: Sprite {
                                color: BevyColor::GRAY,
                                custom_size: Some(Vec2::new(1.0, 1.0)),
                                anchor: Anchor::CenterLeft,
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    });
            }
        }
    }
}

pub fn update_component_events(
    client: Client,
    mut global: ResMut<Global>,
    mut event_reader: EventReader<UpdateComponentEvents>,
    mut position_query: Query<&mut Position>,
    relation_query: Query<&Relation>,
    mut line_query: Query<(Entity, &mut Line)>,
) {
    // When we receive a new Position update for the Player's Entity,
    // we must ensure the Client-side Prediction also remains in-sync
    // So we roll the Prediction back to the authoritative Server state
    // and then execute all Player Commands since that tick, using the CommandHistory helper struct
    if let Some(owned_entity) = &global.owned_entity {
        let mut latest_tick: Option<Tick> = None;
        let server_entity = owned_entity.confirmed;
        let client_entity = owned_entity.predicted;

        for events in event_reader.iter() {
            // Update square position
            for (server_tick, updated_entity) in events.read::<Position>() {
                // If entity is owned
                if updated_entity == server_entity {
                    if let Some(last_tick) = &mut latest_tick {
                        if sequence_greater_than(server_tick, *last_tick) {
                            *last_tick = server_tick;
                        }
                    } else {
                        latest_tick = Some(server_tick);
                    }
                }
            }
            // Update Relation line
            for (_server_tick, updated_entity) in events.read::<Relation>() {
                if let Ok(relation) = relation_query.get(updated_entity) {
                    let baseline_entity = relation.entity.get(&client);
                    for (_line_entity, mut line) in line_query.iter_mut() {
                        if line.start_entity == updated_entity {
                            line.end_entity = baseline_entity;
                            break;
                        }
                    }
                }
            }
        }

        if let Some(server_tick) = latest_tick {
            if let Ok([server_position, mut client_position]) =
                position_query.get_many_mut([server_entity, client_entity])
            {
                // Set to authoritative state
                client_position.mirror(&*server_position);

                // Replay all stored commands

                // TODO: why is it necessary to subtract 1 Tick here?
                // it's not like this in the Macroquad demo
                let modified_server_tick = server_tick.wrapping_sub(1);

                let replay_commands = global.command_history.replays(&modified_server_tick);
                for (_command_tick, command) in replay_commands {
                    shared_behavior::process_command(&command, &mut client_position);
                }
            }
        }
    }
}

pub fn remove_component_events(
    mut commands: Commands,
    mut event_reader: EventReader<RemoveComponentEvents>,
    line_query: Query<(Entity, &Line)>,
) {
    for events in event_reader.iter() {
        for (_entity, _component) in events.read::<Position>() {
            info!("removed Position component from entity");
        }
        for (_entity, _component) in events.read::<Color>() {
            info!("removed Color component from entity");
        }
        for (square_entity, _relation) in events.read::<Relation>() {
            info!("removed Relation component from entity");
            for (line_entity, line) in line_query.iter() {
                if line.start_entity == square_entity {
                    info!("despawned connecting line");
                    commands.entity(line_entity).despawn_recursive();
                    break;
                }
            }
        }
    }
}

pub fn tick_events(
    mut client: Client,
    mut global: ResMut<Global>,
    mut tick_reader: EventReader<ClientTickEvent>,
    mut position_query: Query<&mut Position>,
) {
    let Some(predicted_entity) = global
        .owned_entity
        .as_ref()
        .map(|owned_entity| owned_entity.predicted) else {
        // No owned Entity
        return;
    };

    let Some(command) = global.queued_command.take() else {
        return;
    };

    for ClientTickEvent(client_tick) in tick_reader.iter() {
        if !global.command_history.can_insert(client_tick) {
            // History is full
            continue;
        }

        // Record command
        global.command_history.insert(*client_tick, command.clone());

        // Send command
        client.send_tick_buffer_message::<PlayerCommandChannel, KeyCommand>(client_tick, &command);

        if let Ok(mut position) = position_query.get_mut(predicted_entity) {
            // Apply command
            shared_behavior::process_command(&command, &mut position);
        }
    }
}
