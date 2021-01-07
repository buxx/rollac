use crate::message::Message;

pub mod rabbit;

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Rabbit,
}

pub struct Base {
    type_: Type,
    world_row_i: u32,
    world_col_i: u32,
}

pub trait AnimatedCorpse {
    fn base(&self) -> &Base;
    fn type_(&self) -> Type {
        self.base().type_
    }
    fn get_world_row_i(&self) -> u32 {
        self.base().world_row_i
    }
    fn get_world_col_i(&self) -> u32 {
        self.base().world_col_i
    }
    fn apply_event(&mut self) -> Option<Vec<Message>>;
    fn animate(&mut self) -> Option<Vec<Message>>;
}
