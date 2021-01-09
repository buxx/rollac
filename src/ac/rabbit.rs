use crate::ac::{AnimatedCorpse, Base, Type};
use crate::event::ZoneEvent;
use crate::message::Message;

pub struct Rabbit {
    base: Base,
    counter: i64,
}

impl Rabbit {
    pub fn new(world_row_i: u32, world_col_i: u32) -> Self {
        Rabbit {
            base: Base {
                type_: Type::Rabbit,
                world_row_i,
                world_col_i,
            },
            counter: 0,
        }
    }
}

impl AnimatedCorpse for Rabbit {
    fn base(&self) -> &Base {
        &self.base
    }

    fn apply_event(&mut self, event: &ZoneEvent) -> Option<Vec<Message>> {
        self.counter += 1;
        None
    }

    fn animate(&mut self) -> Option<Vec<Message>> {
        self.counter += 1;
        Some(vec![Message::RequireMove(1, 2)])
    }
}
