use crate::ac::{AnimatedCorpse, AnimatedCorpseBase};
use crate::behavior::move_::Move;
use crate::behavior::{move_, Behavior};
use crate::event::{ZoneEvent, ZoneEventType};
use crate::message::{Message, SendEventMessage, ZoneMessage};
use crate::zone::Zone;
use async_std::stream::Extend;
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

    fn on_event(&self, _event: &ZoneEvent, _zone: &Zone) -> Vec<Message> {
        vec![]
    }

    fn on_message(&mut self, _message: ZoneMessage) {
        //
    }

    fn animate(&self, zone: &Zone, tick_count: u64) -> Vec<Message> {
        vec![]
    }
}
