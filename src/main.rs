use async_std::pin::Pin;
use async_std::task;
use futures::future::join_all;
use async_std::sync::Mutex;
use async_std::prelude::*;
use async_std::stream;
use async_std::stream::Interval;
use async_trait::async_trait;
use std::time::Duration;

mod ac;
mod zone;

async fn execute_at_interval(ac: &Mutex<Box<dyn ac::AnimatedCorpse + Send + Sync>>) {
    let duration = ac::get_interval(ac.lock().await.get_type());
    let mut interval = stream::interval(duration);
    while let Some(_) = interval.next().await {
        ac.lock().await.execute_once();
    }
}

async fn daemon(mut acs: Vec<Mutex<Box<dyn ac::AnimatedCorpse + Send + Sync>>>) {
    let mut zones: Vec<zone::Zone> = vec![];
    let mut futures: Vec<Pin<Box<dyn futures::Future<Output = ()> + std::marker::Send>>> = vec![];


    // fake here by adding all ac in same zone
    for i in 0..acs.len() {
        let ac = acs.pop().unwrap();
        let zone = zone::Zone::new(0, i as u32, vec![ac]);
        zones.push(zone);
    }

    for zone in zones.iter() {
        futures.push(Box::pin(zone.listen_on_events()));
    }

    for zone in zones.iter() {
        for ac in zone.get_acs() {
            futures.push(Box::pin(execute_at_interval(ac)));
        }
    }

    join_all(futures).await;
}

fn main() {
    let mut acs: Vec<Mutex<Box<dyn ac::AnimatedCorpse + Send + Sync>>> = vec![];
    for i in 0..2 {
        acs.push(Mutex::new(Box::new(ac::rabbit::Rabbit::new(0, i))));
    }

    task::block_on(daemon(acs))
}
