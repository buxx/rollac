use async_std::channel::Receiver;
use async_std::sync::Mutex;

use crate::event::ZoneEvent;
use crate::zone::Zone;
use crate::{model, socket};

pub type ZoneRowI = u32;
pub type ZoneColI = u32;
pub type WorldRowI = u32;
pub type WorldColI = u32;
pub type AnimatedCorpseId = u32;
pub type CharacterId = String;
pub type ZoneCoordinates = (WorldRowI, WorldColI);

#[derive(Debug, Clone)]
pub enum SendEventMessage {
    RequireAnimatedCorpseMove(AnimatedCorpseId, ZoneRowI, ZoneColI),
}

#[derive(Debug, Clone)]
pub enum ZoneMessage {
    UpdateAnimatedCorpsePosition(AnimatedCorpseId, ZoneRowI, ZoneColI),
    UpdateCharacterPosition(CharacterId, ZoneRowI, ZoneColI),
    AddBuild(model::Build),
    AddCharacter(CharacterId, ZoneRowI, ZoneColI), // FIXME model::Character
    RemoveCharacter(CharacterId),
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(SendEventMessage, ZoneCoordinates),
    Zone(ZoneMessage, ZoneCoordinates),
}

pub async fn on_messages(
    zones: &Mutex<Vec<Zone>>,
    channel_receiver: Receiver<Message>,
    socket: &socket::Channel,
) {
    log::info!("Listening on messages");
    while let Ok(message) = channel_receiver.recv().await {
        match message {
            Message::Event(event_message, (world_row_i, world_col_i)) => {
                socket
                    .send(ZoneEvent::from_message(
                        event_message,
                        world_row_i,
                        world_col_i,
                    ))
                    .await
            }
            Message::Zone(zone_message, (world_row_i, world_col_i)) => {
                // FIXME BS: check zone match
                for zone in zones.lock().await.iter_mut() {
                    if zone.world_row_i == world_row_i && zone.world_col_i == world_col_i {
                        zone.on_message(zone_message.clone())
                    }
                }
            }
        }
    }

    // TODO: manage daemon close
    panic!("Channel is closed !");
}
