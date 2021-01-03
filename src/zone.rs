use async_std::task;
use std::time::Duration;

use crate::ac;

pub struct Zone {
    world_row_i: u16,
    world_col_i: u16,
    acs: Vec<Box<dyn ac::AnimatedCorpse + Send + Sync>>,
}

impl Zone {
    pub fn new(
        world_row_i: u16,
        world_col_i: u16,
        acs: Vec<Box<dyn ac::AnimatedCorpse + Send + Sync>>,
    ) -> Self {
        Zone {
            world_row_i,
            world_col_i,
            acs,
        }
    }

    pub fn get_acs(&self) -> &Vec<Box<dyn ac::AnimatedCorpse + Send + Sync>> {
        &self.acs
    }

    pub async fn listen_on_events(&self) {
        loop {
            task::sleep(Duration::from_secs(1)).await;
            println!(
                "Zone {}.{} receive event ...",
                self.world_row_i, self.world_col_i
            )
        }
    }
}
