use async_std::pin::Pin;
use async_std::sync::Mutex;
use async_std::task;
use futures::future::join_all;
use std::cell::RefCell;
use std::time::Duration;
use async_std::channel::{unbounded, Sender, Receiver};

use crate::zone::Zone;
use crate::ac::AnimatedCorpse;
use crate::message::Message;

mod ac;
mod zone;
mod message;

async fn on_events(zones: &Mutex<RefCell<Vec<Zone>>>, channel_sender: &Sender<Message>) {
    loop {
        // Simulate websocket events
        task::sleep(Duration::from_secs(2)).await;

        {
            for zone in zones.lock().await.borrow_mut().iter_mut() {
                for message in zone.react() {
                    //channel_sender.send(message).await;
                }
            }
        };
    };
}

async fn animate(zones: &Mutex<RefCell<Vec<Zone>>>, channel_sender: &Sender<Message>) {
    loop {
        task::sleep(Duration::from_secs(1)).await; // TODO calculate to have 1 fps
        {
            for zone in zones.lock().await.borrow_mut().iter_mut() {
                for message in zone.animate() {
                    //channel_sender.send(message).await;
                }
            }
        };
    };
}

async fn on_messages(channel_receiver: Receiver<Message>) {
    while let Ok(message) = channel_receiver.recv().await {
        match message {
            Message::HelloWorldZone => {println!("HelloWorldZone")}
            Message::HelloWorldAnimatedCorpse => {println!("HelloWorldAnimatedCorpse")}
        }
    };

    // TODO: manage daemon close
    panic!("Channel is closed !")
}

async fn daemon(mut animated_corpses: Vec<Box<dyn AnimatedCorpse + Send + Sync>>) {
    let (channel_sender, channel_receiver) = unbounded();
    let mut zones: Vec<Zone> = vec![];

    // fake here by adding all animated_corpse in same zone
    for i in 0..animated_corpses.len() {
        let animated_corpse = animated_corpses.pop().unwrap();
        let zone = Zone::new(0, i as u32, vec![animated_corpse]);
        zones.push(zone);
    }

    let zones: Mutex<RefCell<Vec<Zone>>> = Mutex::new(RefCell::new(zones));
    let mut futures: Vec<Pin<Box<dyn futures::Future<Output = ()> + std::marker::Send>>> = vec![];

    futures.push(Box::pin(on_events(&zones, &channel_sender)));
    futures.push(Box::pin(animate(&zones, &channel_sender)));
    futures.push(Box::pin(on_messages(channel_receiver)));

    join_all(futures).await;
}

fn main() {
    let mut animated_corpses: Vec<Box<dyn AnimatedCorpse + Send + Sync>> = vec![];
    for i in 0..2 {
        animated_corpses.push(Box::new(ac::rabbit::Rabbit::new(0, i)));
    }

    task::block_on(daemon(animated_corpses))
}
