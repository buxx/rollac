use async_std::pin::Pin;
use async_std::task;
use futures::future::join_all;

mod ac;
mod behavior;
mod zone;

async fn daemon(mut acs: Vec<Box<dyn ac::AnimatedCorpse + Send + Sync>>) {
    let mut zones: Vec<zone::Zone> = vec![];
    let mut futures: Vec<Pin<Box<dyn futures::Future<Output = ()> + std::marker::Send>>> = vec![];

    // fake here by adding all ac in same zone
    for i in 0..2 {
        let ac = acs.pop().unwrap();
        let zone = zone::Zone::new(0, i, vec![ac]);
        zones.push(zone);
    }

    for zone in zones.iter() {
        futures.push(Box::pin(zone.listen_on_events()));
    }

    for zone in zones.iter() {
        for ac in zone.get_acs() {
            for behavior in ac.get_behaviors() {
                futures.push(Box::pin(behavior.execute_at_interval(ac)));
            }
        }
    }

    join_all(futures).await;
}

fn main() {
    let mut acs: Vec<Box<dyn ac::AnimatedCorpse + Send + Sync>> = vec![];
    acs.push(Box::new(ac::rabbit::Rabbit::new(0, 0)));
    acs.push(Box::new(ac::rabbit::Rabbit::new(0, 1)));

    task::block_on(daemon(acs))
}
