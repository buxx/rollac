use async_std::channel::unbounded;
use async_std::pin::Pin;
use async_std::sync::Mutex;
use async_std::task;
use futures::future::join_all;
use log;

use crate::ac::AnimatedCorpse;
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
    let url = "http://127.0.0.1:5000/world/events".to_string();
    log::info!("Connect socket on {}", url);
    let mut socket = socket::Channel::new(url);
    socket.connect();

    // Grab world information
    log::info!("Retrieve world from api");
    let world = world::new(&client);

    // Create zones and place animated corpses
    let mut found_animated_corpses = 0;
    for (world_row_i, row) in world.rows.iter().enumerate() {
        for (world_col_i, _) in row.cols.iter().enumerate() {
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
            log::info!("Create zone {}.{}", world_row_i, world_col_i);
            zones.push(zone::new(
                &world,
                &client,
                world_row_i as u32,
                world_col_i as u32,
                zone_animated_corpses,
            ));
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
    futures.push(Box::pin(message::on_messages(
        &zones,
        channel_receiver,
        &socket,
    )));

    join_all(futures).await;
}

fn main() {
    env_logger::init();
    task::block_on(daemon());
}
