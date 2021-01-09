use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use serde_derive::{Deserialize as DeserializeDerive, Serialize as SerializeDerive};
use serde_json::Value;
use std::collections::HashMap;

pub const PLAYER_MOVE: &str = "PLAYER_MOVE";
pub const ANIMATED_CORPSE_MOVE: &str = "ANIMATED_CORPSE_MOVE";
pub const CLIENT_WANT_CLOSE: &str = "CLIENT_WANT_CLOSE";
pub const SERVER_PERMIT_CLOSE: &str = "SERVER_PERMIT_CLOSE";

#[derive(SerializeDerive, DeserializeDerive, Debug)]
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
}

#[derive(Debug)]
pub struct ZoneEvent {
    pub event_type: ZoneEventType,
    pub event_type_name: String,
}

impl ZoneEvent {
    // TODO: by hand for now ... how to do automatic ?
    pub fn from_value(value: Value) -> Result<Self, String> {
        let type_ = value["type"].as_str().unwrap();
        let data = value.get("data").unwrap();

        match &type_ {
            &PLAYER_MOVE => Ok(ZoneEvent {
                event_type_name: String::from(PLAYER_MOVE),
                event_type: ZoneEventType::PlayerMove {
                    to_row_i: data["to_row_i"].as_i64().unwrap() as u32,
                    to_col_i: data["to_col_i"].as_i64().unwrap() as u32,
                    character_id: String::from(data["character_id"].as_str().unwrap()),
                },
            }),
            &ANIMATED_CORPSE_MOVE => Ok(ZoneEvent {
                event_type_name: String::from(ANIMATED_CORPSE_MOVE),
                event_type: ZoneEventType::AnimatedCorpseMove {
                    to_row_i: data["to_row_i"].as_i64().unwrap() as u32,
                    to_col_i: data["to_col_i"].as_i64().unwrap() as u32,
                    animated_corpse_id: data["animated_corpse_id"].as_i64().unwrap() as u32,
                },
            }),
            &CLIENT_WANT_CLOSE => Ok(ZoneEvent {
                event_type_name: String::from(CLIENT_WANT_CLOSE),
                event_type: ZoneEventType::ClientWantClose,
            }),
            &SERVER_PERMIT_CLOSE => Ok(ZoneEvent {
                event_type_name: String::from(SERVER_PERMIT_CLOSE),
                event_type: ZoneEventType::ServerPermitClose,
            }),
            _ => Err(format!("Unknown event {}", &type_)),
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
        state.serialize_field("data", &self.event_type)?;
        state.end()
    }
}
