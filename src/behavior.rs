use crate::ac::AnimatedCorpse;
use async_std::prelude::*;
use async_std::stream;
use async_std::stream::Interval;
use async_trait::async_trait;
use std::time::Duration;

#[async_trait]
pub trait Behavior: Send + Sync {
    fn get_execute_at_interval_duration(&self) -> Duration;
    fn execute_once(&self, ac: &Box<dyn AnimatedCorpse + Send + Sync>);
    async fn execute_at_interval(&self, ac: &Box<dyn AnimatedCorpse + Send + Sync>) {
        let duration = self.get_execute_at_interval_duration();
        let mut interval = stream::interval(duration);
        while let Some(_) = interval.next().await {
            self.execute_once(ac)
        }
    }
}
