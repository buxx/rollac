use crate::ac::{AnimatedCorpse, Type};
use crate::behavior::Behavior;
use crate::event::{ZoneEvent, ZoneEventType};
use crate::message::{Message, SendEventMessage};
use crate::util;
use crate::zone::Zone;
use rand::seq::SliceRandom;
use std::cmp::max;

pub struct Fear {}

impl Fear {
    pub fn from_animated_corpse(_animated_corpse: &Box<dyn AnimatedCorpse + Send + Sync>) -> Self {
        Self {}
    }
}

impl Behavior for Fear {
    fn animate_each(&self) -> Option<u8> {
        None
    }

    fn on_event(
        &self,
        animated_corpse: &Box<dyn AnimatedCorpse + Send + Sync>,
        event: &ZoneEvent,
        _zone: &Zone,
    ) -> Vec<Message> {
        let mut messages: Vec<Message> = vec![];

        match &event.event_type {
            ZoneEventType::PlayerMove {
                to_row_i,
                to_col_i,
                character_id: _,
            } => {
                if let Some(direction) = util::position_direction_from(
                    (animated_corpse.zone_row_i(), animated_corpse.zone_col_i()),
                    (*to_row_i, *to_col_i),
                ) {
                    println!("{:?}", direction);
                    let opposite_direction = util::opposite_direction(direction);
                    let opposite_modifier = util::direction_modifier(opposite_direction);
                    messages.push(Message::Event(SendEventMessage::RequireMove(
                        animated_corpse.base().clone(),
                        max(
                            0,
                            animated_corpse.zone_row_i() as i32 + opposite_modifier.0 as i32,
                        ) as u32,
                        max(
                            0,
                            animated_corpse.zone_col_i() as i32 + opposite_modifier.1 as i32,
                        ) as u32,
                    )));
                }
            }
            _ => {}
        }

        messages
    }

    fn on_animate(
        &self,
        _animated_corpse: &Box<dyn AnimatedCorpse + Send + Sync>,
        _zone: &Zone,
    ) -> Vec<Message> {
        vec![]
    }
}
