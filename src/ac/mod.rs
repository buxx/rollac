use crate::message::Message;

pub mod rabbit;

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Rabbit,
}

pub trait AnimatedCorpse {
    fn get_type(&self) -> Type;
    fn get_world_row_i(&self) -> u32;
    fn get_world_col_i(&self) -> u32;
    fn apply_event(&mut self) -> Option<Vec<Message>>;
    fn animate(&mut self) -> Option<Vec<Message>>;
}
