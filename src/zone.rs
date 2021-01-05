use async_std::task;
use std::time::Duration;

use crate::ac;
use async_std::sync::Mutex;

pub struct Zone {
    world_row_i: u32,
    world_col_i: u32,
    acs: Vec<Mutex<Box<dyn ac::AnimatedCorpse + Send + Sync>>>,
}

impl Zone {
    pub fn new(
        world_row_i: u32,
        world_col_i: u32,
        acs: Vec<Mutex<Box<dyn ac::AnimatedCorpse + Send + Sync>>>,
    ) -> Self {
        Zone {
            world_row_i,
            world_col_i,
            acs,
        }
    }

    pub fn get_acs(&self) -> &Vec<Mutex<Box<dyn ac::AnimatedCorpse + Send + Sync>>> {
        &self.acs
    }

    pub async fn listen_on_events(&self) {
        loop {
            task::sleep(Duration::from_secs(2)).await;
            for ac in self.acs.iter() {
                ac.lock().await.apply_event()
            }
        }
    }
}
