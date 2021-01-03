use crate::behavior;

pub mod rabbit;

#[derive(Debug, Clone)]
pub enum Type {
    Rabbit,
}

pub trait AnimatedCorpse {
    fn get_type(&self) -> Type;
    fn get_behaviors(&self) -> &Vec<Box<dyn behavior::Behavior>>;
    fn get_world_row_i(&self) -> u16;
    fn get_world_col_i(&self) -> u16;
}
