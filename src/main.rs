use std::time::{Duration, Instant};

use async_std::channel::{Receiver, Sender, unbounded};
use async_std::pin::Pin;
use async_std::sync::Mutex;
use async_std::task;
use futures::future::join_all;
use log;

use crate::ac::AnimatedCorpse;
use crate::event::{ZoneEvent, ZoneEventType};
use crate::message::{Message, ZoneMessage};
use crate::tile::world::WorldTiles;
use crate::tile::zone::ZoneTiles;
use crate::zone::Zone;

mod ac;
mod behavior;
mod client;
mod event;
mod message;
mod model;
mod socket;
mod tile;
mod util;
mod world;
mod zone;

const TICK_EACH_MS: u64 = 1000;

async fn daemon() {
    // Prepare required variables
    let mut zones: Vec<Zone> = vec![];
    let client = client::Client::new("127.0.0.1", 5000);
    let (channel_sender, channel_receiver) = unbounded();

    // Connect to world socket
    let mut socket = socket::Channel::new("http://127.0.0.1:5000/world/events".to_string());
    socket.connect();

    // Grab world information
    // FIXME BS NOW: move zone creation somewhere else to readability
    let world_source = match client.get_world_source() {
        Ok(world_source) => world_source,
        Err(msg) => {
            panic!(msg)
        }
    };
    let legend = match util::extract_block_from_source("LEGEND", world_source.as_str()) {
        Ok(legend) => legend,
        Err(msg) => {
            panic!(msg)
        }
    };
    let world_raw = match util::extract_block_from_source("GEO", world_source.as_str()) {
        Ok(world_raw) => world_raw,
        Err(msg) => {
            panic!(msg)
        }
    };
    // Create zones and place animated corpses
    let mut found_animated_corpses = 0;
    for (world_row_i, row) in world_raw.lines().enumerate() {
        for (world_col_i, _) in row.chars().enumerate() {
            let zone_animated_corpses: Vec<Box<dyn AnimatedCorpse + Send + Sync>> = client
                .get_animated_corpses(world_row_i as u32, world_col_i as u32)
                .expect("Error during grab of animated corpses");
            log::info!(
                "Found {} animated corpses for zone {}.{}",
                zone_animated_corpses.len(),
                world_row_i,
                world_col_i
            );
            found_animated_corpses += zone_animated_corpses.len();

            // FIXME BS NOW: move zone creation somewhere else to readability
            let world_tiles = match WorldTiles::new(legend.as_str()) {
                Ok(world_tiles) => world_tiles,
                Err(msg) => {
                    panic!(msg)
                }
            };
            let world = match world::World::new(world_raw.as_str(), &world_tiles) {
                Ok(world) => world,
                Err(msg) => {
                    panic!(msg)
                }
            };
            let world_tile_type_id =
                world.rows[world_row_i as usize].cols[world_col_i as usize].clone();
            let server_tiles_data = match client.get_tiles_data() {
                Ok(server_tiles_data) => server_tiles_data,
                Err(msg) => {
                    panic!(msg)
                }
            };
            let zone_tiles = ZoneTiles::new(server_tiles_data);
            let zone_data = match client.get_zone_data(world_row_i as u32, world_col_i as u32) {
                Ok(zone_data) => zone_data,
                Err(msg) => {
                    panic!(msg)
                }
            };
            let zone_raw = zone_data["raw_source"].as_str().unwrap();
            let zone_raw = util::extract_block_from_source(util::BLOCK_GEO, zone_raw).unwrap();

            let zone_characters = client
                .get_zone_characters(world_row_i as u32, world_col_i as u32)
                .unwrap();
            let zone_builds = client
                .get_zone_builds(world_row_i as u32, world_col_i as u32)
                .unwrap();

            match Zone::new(
                world_row_i as u32,
                world_col_i as u32,
                zone_animated_corpses,
                zone_characters,
                zone_builds,
                &zone_raw,
                zone_tiles,
                world_tile_type_id,
            ) {
                Ok(zone) => zones.push(zone),
                Err(msg) => {
                    panic!(msg)
                }
            }
        }
    }
    log::info!(
        "Total of animated corpses found: {}",
        found_animated_corpses
    );

    let zones: Mutex<Vec<Zone>> = Mutex::new(zones);
    let mut futures: Vec<Pin<Box<dyn futures::Future<Output = ()> + std::marker::Send>>> = vec![];

    futures.push(Box::pin(event::on_events(&zones, &channel_sender, &socket)));
    futures.push(Box::pin(ac::animate(&zones, &channel_sender)));
    futures.push(Box::pin(message::on_messages(&zones, channel_receiver, &socket)));

    join_all(futures).await;
}

fn main() {
    env_logger::init();
    task::block_on(daemon());
}
