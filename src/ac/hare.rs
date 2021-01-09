use crate::ac::{AnimatedCorpse, AnimatedCorpseBase};
use crate::event::{ZoneEvent, ZoneEventType};
use crate::message::{AnimatedCorpseMessage, Message, SendEventMessage};
use crate::zone::Zone;
use rand::seq::SliceRandom;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Hare {
    base: AnimatedCorpseBase,
}

impl Hare {
    pub fn new(base: AnimatedCorpseBase) -> Self {
        Hare { base }
    }
}

impl AnimatedCorpse for Hare {
    fn base(&self) -> &AnimatedCorpseBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut AnimatedCorpseBase {
        &mut self.base
    }

    fn on_event(&self, event: &ZoneEvent, _zone: &Zone) -> Option<Vec<Message>> {
        let mut messages: Vec<Message> = vec![];

        match event.event_type {
            ZoneEventType::AnimatedCorpseMove {
                to_row_i,
                to_col_i,
                animated_corpse_id,
            } => {
                if animated_corpse_id != self.base.id {
                    // TODO Moving in a pack
                    return None;
                } else {
                    messages.push(Message::AnimatedCorpse(
                        AnimatedCorpseMessage::UpdateZonePosition(
                            self.base,
                            to_row_i,
                            to_col_i,
                        ),
                    ))
                }
            }
            _ => {}
        }

        Some(messages)
    }

    fn on_message(&mut self, _message: AnimatedCorpseMessage) {
        //
    }

    fn animate(&self, zone: &Zone) -> Option<Vec<Message>> {
        if let Some(((move_to_row_i, move_to_col_i), _weight)) = zone
            .get_successors(self.zone_row_i(), self.zone_col_i())
            .choose(&mut rand::thread_rng())
        {
            return Some(vec![Message::Event(SendEventMessage::RequireMove(
                self.base,
                *move_to_row_i,
                *move_to_col_i,
            ))]);
        }

        None
    }
}
