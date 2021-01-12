use async_std::channel::{unbounded, Receiver, Sender};
use async_std::pin::Pin;
use async_std::sync::Mutex;
use async_std::task;
use futures::future::join_all;
use std::time::Duration;

use crate::ac::AnimatedCorpse;
use crate::client::ClientError;
use crate::message::Message;
use crate::tile::world::WorldTiles;
use crate::tile::zone::ZoneTiles;
use crate::world::World;
use crate::zone::Zone;
use serde_json::Value;
use std::ops::Deref;

mod ac;
mod client;
mod event;
mod message;
mod socket;
mod tile;
mod util;
mod world;
mod zone;

async fn on_events(
    zones: &Mutex<Vec<Zone>>,
    channel_sender: &Sender<Message>,
    socket: &socket::Channel,
) {
    while let Ok(event) = socket.from_websocket_receiver.recv().await {
        let mut messages: Vec<Message> = vec![];

        {
            for zone in zones.lock().await.iter_mut() {
                messages.extend(zone.react(&event));
            }
        }

        for message in messages {
            if let Err(_) = channel_sender.send(message).await {
                panic!("Channel is closed !")
            }
        }
    }
}

async fn animate(zones: &Mutex<Vec<Zone>>, channel_sender: &Sender<Message>) {
    loop {
        task::sleep(Duration::from_secs(1)).await; // TODO calculate to have 1 fps
        let mut messages: Vec<Message> = vec![];

        {
            for zone in zones.lock().await.iter_mut() {
                messages.extend(zone.animate())
            }
        };

        for message in messages {
            if let Err(_) = channel_sender.send(message).await {
                panic!("Channel is closed !")
            };
        }
    }
}

async fn on_messages(channel_receiver: Receiver<Message>, socket: &socket::Channel) {
    while let Ok(message) = channel_receiver.recv().await {
        match message {
            Message::RequireMove(animated_corpse_info, zone_row_i, zone_col_i) => {
                let (animated_corpse_id, world_row_i, world_col_i) = animated_corpse_info;
                socket
                    .send(event::ZoneEvent {
                        event_type_name: String::from(event::ANIMATED_CORPSE_MOVE),
                        event_type: event::ZoneEventType::AnimatedCorpseMove {
                            to_row_i: zone_row_i,
                            to_col_i: zone_col_i,
                            animated_corpse_id,
                        },
                        world_row_i,
                        world_col_i,
                    })
                    .await;
            }
        }
    }

    // TODO: manage daemon close
    panic!("Channel is closed !");
}

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
        for (world_col_i, tile_type_as_char) in row.chars().enumerate() {
            let zone_animated_corpses: Vec<Box<dyn AnimatedCorpse + Send + Sync>> = client
                .get_animated_corpses(world_row_i as u32, world_col_i as u32)
                .expect("Error during grab of animated corpses");
            println!(
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

            match Zone::new(
                world_row_i as u32,
                world_col_i as u32,
                zone_animated_corpses,
                &zone_raw,
                &zone_tiles,
                world_tile_type_id,
            ) {
                Ok(zone) => zones.push(zone),
                Err(msg) => {
                    panic!(msg)
                }
            }
        }
    }
    println!(
        "Total of animated corpses found: {}",
        found_animated_corpses
    );

    let zones: Mutex<Vec<Zone>> = Mutex::new(zones);
    let mut futures: Vec<Pin<Box<dyn futures::Future<Output = ()> + std::marker::Send>>> = vec![];

    futures.push(Box::pin(on_events(&zones, &channel_sender, &socket)));
    futures.push(Box::pin(animate(&zones, &channel_sender)));
    futures.push(Box::pin(on_messages(channel_receiver, &socket)));

    join_all(futures).await;
}

fn main() {
    task::block_on(daemon())
}
