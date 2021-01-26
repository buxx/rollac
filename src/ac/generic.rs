use crate::ac::{AnimatedCorpse, AnimatedCorpseBase};
use crate::event::ZoneEvent;
use crate::message::{Message, ZoneMessage};
use crate::zone::Zone;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Generic {
    base: AnimatedCorpseBase,
}

impl Generic {
    pub fn new(base: AnimatedCorpseBase) -> Self {
        Self { base }
    }
}

impl AnimatedCorpse for Generic {
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

    fn animate(&self, _zone: &Zone, _tick_count: u64) -> Vec<Message> {
        vec![]
    }
}
