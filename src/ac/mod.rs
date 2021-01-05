use async_std::prelude::*;
use async_std::stream;
use async_std::stream::Interval;
use async_trait::async_trait;
use std::time::Duration;
use async_std::sync::Mutex;
use async_std::pin::Pin;
use async_std::prelude::*;

use crate::behavior::Behavior;

pub mod rabbit;

#[derive(Debug, Clone)]
pub enum Type {
    Rabbit,
}


// async fn execute_at_interval(ac: &Mutex<Box<dyn Behavior + Send + Sync>>, duration: Duration) {
// async fn execute_at_interval(ac: &Mutex<Box<dyn AnimatedCorpse + Send + Sync>>, behavior: &Box<dyn Behavior>, duration: Duration) {
//     let mut interval = stream::interval(duration);
//     while let Some(_) = interval.next().await {
//         behavior.execute_once(ac);
//     }
// }

pub trait AnimatedCorpse {
    fn get_type(&self) -> Type;
    fn get_world_row_i(&self) -> u32;
    fn get_world_col_i(&self) -> u32;
    fn apply_event(&mut self);
    fn execute_once(&mut self);
    fn get_behaviors(&self) -> Vec<Box<dyn Behavior>>;
    fn get_futures(&self) -> Vec<Pin<Box<dyn futures::Future<Output = ()> + std::marker::Send>>> {
        let mut futures: Vec<Pin<Box<dyn futures::Future<Output = ()> + std::marker::Send>>> = vec![];
        for behavior in self.get_behaviors() {
            let duration = behavior.get_interval();
            let future = async move {
                {
                    let mut interval = stream::interval(duration);
                    while let Some(_) = interval.next().await {
                        behavior.execute_once();
                    }
                }
            };
            futures.push(Box::pin(future));
        }
        futures
    }
}
