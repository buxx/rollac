use std::collections::HashMap;

use async_std::channel::Sender;
use async_std::sync::Mutex;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::{model, socket};
use crate::message::{Message, SendEventMessage, ZoneMessage};
use crate::zone::Zone;

pub const PLAYER_MOVE: &str = "PLAYER_MOVE";
pub const ANIMATED_CORPSE_MOVE: &str = "ANIMATED_CORPSE_MOVE";
pub const CLIENT_WANT_CLOSE: &str = "CLIENT_WANT_CLOSE";
pub const SERVER_PERMIT_CLOSE: &str = "SERVER_PERMIT_CLOSE";
pub const CHARACTER_ENTER_ZONE: &str = "CHARACTER_ENTER_ZONE";
pub const CHARACTER_EXIT_ZONE: &str = "CHARACTER_EXIT_ZONE";
pub const NEW_BUILD: &str = "NEW_BUILD";

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ZoneEventType {
    ClientWantClose,
    ServerPermitClose,
    PlayerMove {
        to_row_i: u32,
        to_col_i: u32,
        character_id: String,
    },
    AnimatedCorpseMove {
        to_row_i: u32,
        to_col_i: u32,
        animated_corpse_id: u32,
    },
    CharacterEnter {
        zone_row_i: u32,
        zone_col_i: u32,
        character_id: String,
    },
    CharacterExit {
        character_id: String,
    },
    NewBuild {
        build: model::Build,
    },
}

#[derive(Debug)]
pub struct ZoneEvent {
    pub event_type: ZoneEventType,
    pub event_type_name: String,
    pub world_row_i: u32,
    pub world_col_i: u32,
}

impl ZoneEvent {
    // TODO: by hand for now ... how to do automatic ?
    pub fn from_value(value: Value) -> Result<Self, String> {
        let type_ = value["type"].as_str().unwrap();
        let world_row_i = value["world_row_i"].as_i64().unwrap() as u32;
        let world_col_i = value["world_col_i"].as_i64().unwrap() as u32;
        let data = value.get("data").unwrap();

        match &type_ {
            &PLAYER_MOVE => Ok(ZoneEvent {
                world_row_i,
                world_col_i,
                event_type_name: String::from(PLAYER_MOVE),
                event_type: ZoneEventType::PlayerMove {
                    to_row_i: data["to_row_i"].as_i64().unwrap() as u32,
                    to_col_i: data["to_col_i"].as_i64().unwrap() as u32,
                    character_id: String::from(data["character_id"].as_str().unwrap()),
                },
            }),
            &ANIMATED_CORPSE_MOVE => Ok(ZoneEvent {
                world_row_i,
                world_col_i,
                event_type_name: String::from(ANIMATED_CORPSE_MOVE),
                event_type: ZoneEventType::AnimatedCorpseMove {
                    to_row_i: data["to_row_i"].as_i64().unwrap() as u32,
                    to_col_i: data["to_col_i"].as_i64().unwrap() as u32,
                    animated_corpse_id: data["animated_corpse_id"].as_i64().unwrap() as u32,
                },
            }),
            &CLIENT_WANT_CLOSE => Ok(ZoneEvent {
                world_row_i,
                world_col_i,
                event_type_name: String::from(CLIENT_WANT_CLOSE),
                event_type: ZoneEventType::ClientWantClose,
            }),
            &SERVER_PERMIT_CLOSE => Ok(ZoneEvent {
                world_row_i,
                world_col_i,
                event_type_name: String::from(SERVER_PERMIT_CLOSE),
                event_type: ZoneEventType::ServerPermitClose,
            }),
            &CHARACTER_ENTER_ZONE => Ok(ZoneEvent {
                world_row_i,
                world_col_i,
                event_type_name: String::from(CHARACTER_ENTER_ZONE),
                event_type: ZoneEventType::CharacterEnter {
                    zone_row_i: data["zone_row_i"].as_i64().unwrap() as u32,
                    zone_col_i: data["zone_col_i"].as_i64().unwrap() as u32,
                    character_id: String::from(data["character_id"].as_str().unwrap()),
                },
            }),
            &CHARACTER_EXIT_ZONE => Ok(ZoneEvent {
                world_row_i,
                world_col_i,
                event_type_name: String::from(CHARACTER_EXIT_ZONE),
                event_type: ZoneEventType::CharacterExit {
                    character_id: String::from(data["character_id"].as_str().unwrap()),
                },
            }),
            &NEW_BUILD => {
                let build_data = data["build"].as_object().unwrap();
                let mut traversable: HashMap<String, bool> = HashMap::new();
                traversable.insert(
                    "WALKING".to_string(),
                    build_data["traversable"]
                        .as_object()
                        .unwrap()
                        .get("WALKING")
                        .unwrap()
                        .as_bool()
                        .unwrap(),
                );

                Ok(ZoneEvent {
                    world_row_i,
                    world_col_i,
                    event_type_name: String::from(NEW_BUILD),
                    event_type: ZoneEventType::NewBuild {
                        build: model::Build {
                            id: build_data["id"].as_i64().unwrap() as u32,
                            build_id: build_data["build_id"].as_str().unwrap().to_string(),
                            row_i: build_data["row_i"].as_i64().unwrap() as u32,
                            col_i: build_data["col_i"].as_i64().unwrap() as u32,
                            traversable,
                        },
                    },
                })
            }
            _ => Err(format!("Unknown event {}", &type_)),
        }
    }

    pub fn from_message(message: SendEventMessage, world_row_i: u32, world_col_i: u32) -> Self {
        match message {
            SendEventMessage::RequireAnimatedCorpseMove(
                animated_corpse_id,
                zone_row_i,
                zone_col_i,
            ) => Self {
                event_type_name: String::from(ANIMATED_CORPSE_MOVE),
                event_type: ZoneEventType::AnimatedCorpseMove {
                    to_row_i: zone_row_i,
                    to_col_i: zone_col_i,
                    animated_corpse_id,
                },
                world_row_i,
                world_col_i,
            },
        }
    }
}

impl Serialize for ZoneEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ZoneEvent", 2)?;
        state.serialize_field("type", &self.event_type_name)?;
        state.serialize_field("world_row_i", &self.world_row_i)?;
        state.serialize_field("world_col_i", &self.world_col_i)?;
        state.serialize_field("data", &self.event_type)?;
        state.end()
    }
}

pub async fn on_events(
    zones: &Mutex<Vec<Zone>>,
    channel_sender: &Sender<Message>,
    socket: &socket::Channel,
) {
    log::info!("Listening events");
    while let Ok(event) = socket.from_websocket_receiver.recv().await {
        log::debug!("Event received: {:?}", event);
        let mut messages: Vec<Message> = vec![];

        match &event.event_type {
            // Ignore internal mechanisms events
            ZoneEventType::ClientWantClose | ZoneEventType::ServerPermitClose => continue,
            // First convert some event to messages
            ZoneEventType::PlayerMove {
                to_row_i,
                to_col_i,
                character_id,
            } => {
                messages.push(Message::Zone(
                    ZoneMessage::UpdateCharacterPosition(
                        character_id.clone(),
                        *to_row_i,
                        *to_col_i,
                    ),
                    (event.world_row_i, event.world_col_i),
                ));
            }
            ZoneEventType::AnimatedCorpseMove {
                to_row_i,
                to_col_i,
                animated_corpse_id,
            } => messages.push(Message::Zone(
                ZoneMessage::UpdateAnimatedCorpsePosition(
                    *animated_corpse_id,
                    *to_row_i,
                    *to_col_i,
                ),
                (event.world_row_i, event.world_col_i),
            )),
            ZoneEventType::CharacterEnter {
                zone_row_i,
                zone_col_i,
                character_id,
            } => {
                messages.push(Message::Zone(
                    ZoneMessage::AddCharacter(character_id.clone(), *zone_row_i, *zone_col_i),
                    (event.world_row_i, event.world_col_i),
                ));
            }
            ZoneEventType::CharacterExit { character_id } => {
                messages.push(Message::Zone(
                    ZoneMessage::RemoveCharacter(character_id.clone()),
                    (event.world_row_i, event.world_col_i),
                ));
            }
            ZoneEventType::NewBuild { build } => {
                messages.push(Message::Zone(
                    ZoneMessage::AddBuild(build.clone()),
                    (event.world_row_i, event.world_col_i),
                ));
            }
        }

        for zone in zones.lock().await.iter_mut() {
            if event.world_row_i == zone.world_row_i && event.world_col_i == zone.world_col_i {
                messages.extend(zone.on_event(&event));
            }
        }

        for message in messages {
            if let Err(_) = channel_sender.send(message).await {
                panic!("Channel is closed !")
            }
        }
    }
}
