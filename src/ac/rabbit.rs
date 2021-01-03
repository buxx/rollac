use crate::ac::{AnimatedCorpse, Type};
use crate::behavior::Behavior;
use std::time::Duration;

struct Move {}
impl Behavior for Move {
    fn get_execute_at_interval_duration(&self) -> Duration {
        Duration::from_secs(2)
    }

    fn execute_once(&self, ac: &Box<dyn AnimatedCorpse + Send + Sync>) {
        println!("move")
    }
}

struct Eat {}
impl Behavior for Eat {
    fn get_execute_at_interval_duration(&self) -> Duration {
        Duration::from_secs(5)
    }

    fn execute_once(&self, ac: &Box<dyn AnimatedCorpse + Send + Sync>) {
        println!("eat")
    }
}

pub struct Rabbit {
    type_: Type,
    world_row_i: u16,
    world_col_i: u16,
    behaviors: Vec<Box<dyn Behavior>>,
}

impl Rabbit {
    pub fn new(world_row_i: u16, world_col_i: u16) -> Self {
        Rabbit {
            type_: Type::Rabbit,
            world_row_i,
            world_col_i,
            behaviors: vec![Box::new(Move {}), Box::new(Eat {})],
        }
    }
}

impl AnimatedCorpse for Rabbit {
    fn get_type(&self) -> Type {
        self.type_.clone()
    }

    fn get_behaviors(&self) -> &Vec<Box<dyn Behavior>> {
        &self.behaviors
    }

    fn get_world_row_i(&self) -> u16 {
        self.world_row_i
    }

    fn get_world_col_i(&self) -> u16 {
        self.world_col_i
    }
}
