use async_std::channel::{unbounded, Receiver, Sender};
use async_std::pin::Pin;
use async_std::sync::Mutex;
use async_std::task;
use futures::future::join_all;
use std::time::Duration;

use crate::ac::AnimatedCorpse;
use crate::message::Message;
use crate::zone::Zone;

mod ac;
mod client;
mod event;
mod message;
mod socket;
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

async fn daemon(mut animated_corpses: Vec<Box<dyn AnimatedCorpse + Send + Sync>>) {
    let (channel_sender, channel_receiver) = unbounded();
    let mut socket = socket::Channel::new("http://127.0.0.1:5000/world/events".to_string());
    socket.connect();
    let mut zones: Vec<Zone> = vec![];

    // fake here by adding all animated_corpse in same zone
    for i in 0..animated_corpses.len() {
        let animated_corpse = animated_corpses.pop().unwrap();
        let zone = Zone::new(0, i as u32, vec![animated_corpse]);
        zones.push(zone);
    }

    let zones: Mutex<Vec<Zone>> = Mutex::new(zones);
    let mut futures: Vec<Pin<Box<dyn futures::Future<Output = ()> + std::marker::Send>>> = vec![];

    futures.push(Box::pin(on_events(&zones, &channel_sender, &socket)));
    futures.push(Box::pin(animate(&zones, &channel_sender)));
    futures.push(Box::pin(on_messages(channel_receiver, &socket)));

    join_all(futures).await;
}

fn main() {
    let client = client::Client::new("127.0.0.1", 5000);
    let animated_corpses: Vec<Box<dyn AnimatedCorpse + Send + Sync>> =
        client.get_animated_corpses().unwrap();
    println!("Found {} animated corpses", animated_corpses.len());

    task::block_on(daemon(animated_corpses))
}
