use async_std::prelude::*;
use async_std::stream;
use async_std::stream::Interval;
use async_trait::async_trait;
use std::time::Duration;
use async_std::sync::Mutex;

pub mod rabbit;

#[derive(Debug, Clone)]
pub enum Type {
    Rabbit,
}

pub fn get_interval(type_: Type) -> Duration {
    match type_ {
        Type::Rabbit => {Duration::from_secs(1)}
    }
}

pub trait AnimatedCorpse {
    fn get_type(&self) -> Type;
    fn get_world_row_i(&self) -> u32;
    fn get_world_col_i(&self) -> u32;
    fn apply_event(&mut self);
    fn execute_once(&mut self);
}
