use crate::ac::{AnimatedCorpse, Base, Type};
use crate::event::{ZoneEvent, ZoneEventType};
use crate::message::{AnimatedCorpseInfo, Message};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Hare {
    base: Base,
}

impl Hare {
    pub fn new(base: Base) -> Self {
        Hare { base }
    }
}

impl AnimatedCorpse for Hare {
    fn base(&self) -> &Base {
        &self.base
    }

    fn apply_event(&mut self, event: &ZoneEvent) -> Option<Vec<Message>> {
        match event.event_type {
            ZoneEventType::AnimatedCorpseMove {
                to_row_i,
                to_col_i,
                animated_corpse_id,
            } => {
                if animated_corpse_id != self.base.id {
                    return None; // We don't care this event for now if it is not for itself
                }
                self.base.zone_row_i = to_row_i;
                self.base.zone_col_i = to_col_i;
            }
            _ => {}
        }

        None
    }

    fn animate(&mut self) -> Option<Vec<Message>> {
        println!("animate");
        let new_zone_row_i = self.base.zone_row_i + 1;
        Some(vec![Message::RequireMove(
            (self.id(), self.world_row_i(), self.world_col_i()),
            new_zone_row_i,
            self.base.zone_col_i,
        )])
    }
}
