use crate::ac::hare::Hare;
use crate::event::ZoneEvent;
use crate::message::Message;
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
    let base: Base = serde_json::from_value(value.clone()).unwrap();
    let type_ = value["type_"].as_str().unwrap();
    match type_ {
        "HARE" => Ok(Box::new(Hare::new(base))),
        _ => Err(format!("Unknown type {}", type_)),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Base {
    id: u32,
    type_: Type,
    world_row_i: u32,
    world_col_i: u32,
    zone_row_i: u32,
    zone_col_i: u32,
}

pub trait AnimatedCorpse {
    fn base(&self) -> &Base;
    fn type_(&self) -> Type {
        self.base().type_
    }
    fn id(&self) -> u32 {
        self.base().id
    }
    fn world_row_i(&self) -> u32 {
        self.base().world_row_i
    }
    fn world_col_i(&self) -> u32 {
        self.base().world_col_i
    }
    fn zone_row_i(&self) -> u32 {
        self.base().zone_row_i
    }
    fn zone_col_i(&self) -> u32 {
        self.base().zone_col_i
    }
    fn apply_event(&mut self, event: &ZoneEvent) -> Option<Vec<Message>>;
    fn animate(&mut self) -> Option<Vec<Message>>;
}
