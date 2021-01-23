use crate::ac::{AnimatedCorpse, Type};
use crate::behavior::Behavior;
use crate::event::{ZoneEvent, ZoneEventType};
use crate::message::{Message, SendEventMessage};
use crate::util;
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
        Some(1)
    }

    fn on_event(
        &self,
        animated_corpse: &Box<dyn AnimatedCorpse + Send + Sync>,
        event: &ZoneEvent,
        _zone: &Zone,
    ) -> Vec<Message> {
        match &event.event_type {
            ZoneEventType::AnimatedCorpseMove {
                to_row_i: _,
                to_col_i: _,
                animated_corpse_id,
            } => {
                if animated_corpse_id != &animated_corpse.base().id && self.move_in_pack {
                    // TODO Moving in a pack
                };
            }
            _ => {}
        }

        vec![]
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
            let mut near_character = false;
            for character in &zone.characters {
                // FIXME: impelment retry (5 times for example)
                if util::is_near(
                    (character.zone_row_i, character.zone_col_i),
                    (*move_to_row_i, *move_to_col_i),
                    2,
                ) {
                    near_character = true;
                    break;
                }
            }
            if !near_character {
                messages.push(Message::Event(
                    SendEventMessage::RequireAnimatedCorpseMove(
                        animated_corpse.id(),
                        *move_to_row_i,
                        *move_to_col_i,
                    ),
                    (animated_corpse.world_row_i(), animated_corpse.world_col_i()),
                ));
            }
        }

        messages
    }
}
