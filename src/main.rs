use async_std::channel::{unbounded, Receiver, Sender};
use async_std::pin::Pin;
use async_std::sync::Mutex;
use async_std::task;
use futures::future::join_all;
use std::time::{Duration, Instant};

use crate::ac::AnimatedCorpse;
use crate::event::{ZoneEvent, ZoneEventType};
use crate::message::Message;
use crate::tile::world::WorldTiles;
use crate::tile::zone::ZoneTiles;
use crate::zone::Zone;

mod ac;
mod behavior;
mod client;
mod event;
mod message;
mod socket;
mod tile;
mod util;
mod world;
mod zone;

const TICK_EACH_MS: u64 = 1000;

async fn on_events(
    zones: &Mutex<Vec<Zone>>,
    channel_sender: &Sender<Message>,
    socket: &socket::Channel,
) {
    while let Ok(event) = socket.from_websocket_receiver.recv().await {
        let mut messages: Vec<Message> = vec![];

        match event.event_type {
            // Ignore internal mechanisms events
            ZoneEventType::ClientWantClose | ZoneEventType::ServerPermitClose => continue,
            // Event to give to animated corpses
            ZoneEventType::PlayerMove { .. } | ZoneEventType::AnimatedCorpseMove { .. } => {
                for zone in zones.lock().await.iter_mut() {
                    if event.world_row_i == zone.world_row_i
                        && event.world_col_i == zone.world_col_i
                    {
                        messages.extend(zone.on_event(&event));
                    }
                }
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
    let mut tick_count: u64 = 0;
    let mut last_tick = Instant::now();
    loop {
        let now = Instant::now();
        let last_tick_duration = now - last_tick;
        task::sleep(Duration::from_millis(
            TICK_EACH_MS - last_tick_duration.as_millis() as u64,
        ))
        .await;
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

async fn on_messages(
    zones: &Mutex<Vec<Zone>>,
    channel_receiver: Receiver<Message>,
    socket: &socket::Channel,
) {
    while let Ok(message) = channel_receiver.recv().await {
        match message {
            Message::Event(send_event_message) => {
                socket
                    .send(ZoneEvent::from_message(send_event_message))
                    .await
            }
            Message::AnimatedCorpse(animated_corpse_message) => {
                for zone in zones.lock().await.iter_mut() {
                    zone.on_message(animated_corpse_message)
                }
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
        for (world_col_i, _) in row.chars().enumerate() {
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
    println!(
        "Total of animated corpses found: {}",
        found_animated_corpses
    );

    let zones: Mutex<Vec<Zone>> = Mutex::new(zones);
    let mut futures: Vec<Pin<Box<dyn futures::Future<Output = ()> + std::marker::Send>>> = vec![];

    futures.push(Box::pin(on_events(&zones, &channel_sender, &socket)));
    futures.push(Box::pin(animate(&zones, &channel_sender)));
    futures.push(Box::pin(on_messages(&zones, channel_receiver, &socket)));

    join_all(futures).await;
}

fn main() {
    task::block_on(daemon())
}
