use crate::model;

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
