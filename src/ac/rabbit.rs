use crate::ac::{AnimatedCorpse, Type};
use std::time::Duration;
use async_std::sync::Mutex;
use crate::behavior::Behavior;

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

struct Move {}
impl Behavior for Move {
    fn get_interval(&self) -> Duration {
        Duration::from_secs(2)
    }

    fn execute_once(&self) {
        println!("move")
    }
}

struct Eat {}
impl Behavior for Eat {
    fn get_interval(&self) -> Duration {
        Duration::from_secs(5)
    }

    fn execute_once(&self) {
        println!("eat")
    }
}

impl AnimatedCorpse for Rabbit {
    fn get_type(&self) -> Type {
        self.type_.clone()
    }

    fn get_world_row_i(&self) -> u32 {
        self.world_row_i
    }

    fn get_world_col_i(&self) -> u32 {
        self.world_col_i
    }

    fn apply_event(&mut self) {
        self.counter += 1;
        println!("apply_event: {}", self.counter);
    }

    fn execute_once(&mut self) {
        self.counter += 1;
        println!("execute_once: {}", self.counter);
    }

    fn get_behaviors(&self) -> Vec<Box<dyn Behavior>> {
        vec![
            Box::new(Move {}),
            Box::new(Eat {}),
        ]
    }
}
