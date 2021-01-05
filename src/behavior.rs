use crate::ac::AnimatedCorpse;
use async_std::prelude::*;
use async_std::stream;
use async_std::stream::Interval;
use async_trait::async_trait;
use std::time::Duration;
use async_std::sync::Mutex;

#[async_trait]
pub trait Behavior: Send + Sync {
    fn get_interval(&self) -> Duration;
    fn execute_once(&self);
}
