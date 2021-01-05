use crate::ac::{AnimatedCorpse, Type};
use crate::message::Message;

pub struct Rabbit {
    type_: Type,
    world_row_i: u32,
    world_col_i: u32,
    counter: i64,
}

impl Rabbit {
    pub fn new(world_row_i: u32, world_col_i: u32) -> Self {
        Rabbit {
            type_: Type::Rabbit,
            world_row_i,
            world_col_i,
            counter: 0,
        }
    }
}

impl AnimatedCorpse for Rabbit {
    fn get_type(&self) -> Type {
        self.type_
    }

    fn get_world_row_i(&self) -> u32 {
        self.world_row_i
    }

    fn get_world_col_i(&self) -> u32 {
        self.world_col_i
    }

    fn apply_event(&mut self) -> Option<Vec<Message>> {
        self.counter += 1;
        Some(vec![Message::HelloWorldAnimatedCorpse])
    }

    fn animate(&mut self) -> Option<Vec<Message>> {
        self.counter += 1;
        println!("animate ({})", self.counter);
        Some(vec![Message::HelloWorldAnimatedCorpse])
    }
}
