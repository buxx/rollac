use crate::ac::{AnimatedCorpse, Type};
use crate::behavior::Behavior;
use crate::event::{ZoneEvent, ZoneEventType};
use crate::message::{Message, SendEventMessage};
use crate::zone::Zone;
use rand::seq::SliceRandom;

pub struct Move {
    pub move_in_pack: bool,
}

impl Move {
    pub fn from_animated_corpse(animated_corpse: &Box<dyn AnimatedCorpse + Send + Sync>) -> Self {
        let move_in_pack = match animated_corpse.type_() {
            Type::HARE => true,
        };
        Self { move_in_pack }
    }
}

impl Behavior for Move {
    fn animate_each(&self) -> Option<u8> {
        Some(5)
    }

    fn on_event(
        &self,
        animated_corpse: &Box<dyn AnimatedCorpse + Send + Sync>,
        event: &ZoneEvent,
        _zone: &Zone,
    ) -> Vec<Message> {
        let mut messages: Vec<Message> = vec![];

        match &event.event_type {
            ZoneEventType::AnimatedCorpseMove {
                to_row_i,
                to_col_i,
                animated_corpse_id,
            } => {
                if animated_corpse_id != &animated_corpse.base().id && self.move_in_pack {
                    // TODO Moving in a pack
                };
            }
            _ => {}
        }

        messages
    }

    fn on_animate(
        &self,
        animated_corpse: &Box<dyn AnimatedCorpse + Send + Sync>,
        zone: &Zone,
    ) -> Vec<Message> {
        let mut messages: Vec<Message> = vec![];

        if let Some(((move_to_row_i, move_to_col_i), _weight)) = zone
            .get_successors(animated_corpse.zone_row_i(), animated_corpse.zone_col_i())
            .choose(&mut rand::thread_rng())
        {
            messages.push(Message::Event(SendEventMessage::RequireMove(
                animated_corpse.base().clone(),
                *move_to_row_i,
                *move_to_col_i,
            )));
        }

        messages
    }
}
