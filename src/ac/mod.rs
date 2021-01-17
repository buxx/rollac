use crate::ac::hare::Hare;
use crate::behavior::Behavior;
use crate::event::{ZoneEvent, ZoneEventType};
use crate::message::{AnimatedCorpseMessage, Message};
use crate::zone::Zone;
use async_std::stream::Extend;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

pub mod hare;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Type {
    HARE,
}

pub fn animated_corpse_from_value(
    value: Value,
) -> Result<Box<dyn AnimatedCorpse + Send + Sync>, String> {
    let base: AnimatedCorpseBase = serde_json::from_value(value.clone()).unwrap();
    let type_ = value["type_"].as_str().unwrap();
    match type_ {
        "HARE" => Ok(Box::new(Hare::new(base))),
        _ => Err(format!("Unknown type {}", type_)),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct AnimatedCorpseBase {
    pub id: u32,
    pub type_: Type,
    pub world_row_i: u32,
    pub world_col_i: u32,
    pub zone_row_i: u32,
    pub zone_col_i: u32,
}

pub trait AnimatedCorpse {
    fn base(&self) -> &AnimatedCorpseBase;
    fn base_mut(&mut self) -> &mut AnimatedCorpseBase;
    fn type_(&self) -> Type {
        self.base().type_
    }
    fn id(&self) -> u32 {
        self.base().id
    }
    fn world_row_i(&self) -> u32 {
        self.base().world_row_i
    }
    fn set_world_row_i(&mut self, world_row_i: u32) {
        self.base_mut().world_row_i = world_row_i
    }
    fn world_col_i(&self) -> u32 {
        self.base().world_col_i
    }
    fn set_world_col_i(&mut self, world_col_i: u32) {
        self.base_mut().world_col_i = world_col_i
    }
    fn zone_row_i(&self) -> u32 {
        self.base().zone_row_i
    }
    fn set_zone_row_i(&mut self, zone_row_i: u32) {
        self.base_mut().zone_row_i = zone_row_i
    }
    fn zone_col_i(&self) -> u32 {
        self.base().zone_col_i
    }
    fn set_zone_col_i(&mut self, zone_col_i: u32) {
        self.base_mut().zone_col_i = zone_col_i
    }
    fn on_event(&self, event: &ZoneEvent, zone: &Zone) -> Vec<Message> {
        let mut messages: Vec<Message> = vec![];

        match event.event_type {
            ZoneEventType::AnimatedCorpseMove {
                to_row_i,
                to_col_i,
                animated_corpse_id,
            } => {
                if animated_corpse_id == self.base().id {
                    messages.push(Message::AnimatedCorpse(
                        AnimatedCorpseMessage::UpdateZonePosition(
                            self.base().clone(),
                            to_row_i,
                            to_col_i,
                        ),
                    ))
                }
            }
            _ => {}
        }

        for message in self._on_event(event, zone) {
            messages.push(message);
        }

        messages
    }
    fn _on_event(&self, event: &ZoneEvent, zone: &Zone) -> Vec<Message>;
    fn on_message(&mut self, message: AnimatedCorpseMessage);
    fn animate(&self, zone: &Zone, tick_count: u64) -> Vec<Message>;
}
