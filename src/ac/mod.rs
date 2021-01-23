use std::time::{Duration, Instant};

use async_std::channel::Sender;
use async_std::sync::Mutex;
use async_std::task::sleep;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::ac::hare::Hare;
use crate::event::ZoneEvent;
use crate::message::{Message, ZoneMessage};
use crate::TICK_EACH_MS;
use crate::zone::Zone;

pub mod hare;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Type {
    HARE,
}

pub fn animated_corpse_from_value(
    value: Value,
) -> Result<Box<dyn AnimatedCorpse + Send + Sync>, String> {
    let base: AnimatedCorpseBase = serde_json::from_value(value.clone()).unwrap();
    let type_ = value["type_"].as_str().unwrap();
    match type_ {
        "HARE" => Ok(Box::new(Hare::new(base))),
        _ => Err(format!("Unknown type {}", type_)),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct AnimatedCorpseBase {
    pub id: u32,
    pub type_: Type,
    pub world_row_i: u32,
    pub world_col_i: u32,
    pub zone_row_i: u32,
    pub zone_col_i: u32,
}

pub trait AnimatedCorpse {
    fn base(&self) -> &AnimatedCorpseBase;
    fn base_mut(&mut self) -> &mut AnimatedCorpseBase;
    fn type_(&self) -> Type {
        self.base().type_
    }
    fn id(&self) -> u32 {
        self.base().id
    }
    fn world_row_i(&self) -> u32 {
        self.base().world_row_i
    }
    fn set_world_row_i(&mut self, world_row_i: u32) {
        self.base_mut().world_row_i = world_row_i
    }
    fn world_col_i(&self) -> u32 {
        self.base().world_col_i
    }
    fn set_world_col_i(&mut self, world_col_i: u32) {
        self.base_mut().world_col_i = world_col_i
    }
    fn zone_row_i(&self) -> u32 {
        self.base().zone_row_i
    }
    fn set_zone_row_i(&mut self, zone_row_i: u32) {
        self.base_mut().zone_row_i = zone_row_i
    }
    fn zone_col_i(&self) -> u32 {
        self.base().zone_col_i
    }
    fn set_zone_col_i(&mut self, zone_col_i: u32) {
        self.base_mut().zone_col_i = zone_col_i
    }
    fn on_event(&self, event: &ZoneEvent, zone: &Zone) -> Vec<Message>;
    fn on_message(&mut self, message: ZoneMessage);
    fn animate(&self, zone: &Zone, tick_count: u64) -> Vec<Message>;
}

pub async fn animate(zones: &Mutex<Vec<Zone>>, channel_sender: &Sender<Message>) {
    let mut tick_count: u64 = 0;
    let mut last_tick = Instant::now();
    log::info!("Begin animation loop");
    loop {
        let now = Instant::now();
        let last_tick_duration = now - last_tick;
        let sleep_for = TICK_EACH_MS - last_tick_duration.as_millis() as u64;
        log::debug!("Last tick duration is {} ms, sleep for {} ms", last_tick_duration.as_millis(), sleep_for);
        sleep(Duration::from_millis(sleep_for)).await;
        last_tick = Instant::now();
        let mut messages: Vec<Message> = vec![];

        {
            for zone in zones.lock().await.iter_mut() {
                messages.extend(zone.animate(tick_count))
            }
        };

        for message in messages {
            if let Err(_) = channel_sender.send(message).await {
                panic!("Channel is closed !")
            };
        }

        tick_count += 1;
    }
}
