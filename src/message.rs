use crate::model;

pub type MessageAnimatedCorpseBase = (u32, u32, u32);
pub type MessageCharacterBase = (String, u32, u32);
pub type MessageBuildBase = (String, u32, u32);

#[derive(Debug, Clone)]
pub enum SendEventMessage {
    RequireMove(MessageAnimatedCorpseBase, u32, u32),
}

#[derive(Debug, Clone)]
pub enum ZoneMessage {
    UpdateAnimatedCorpsePosition(MessageAnimatedCorpseBase, u32, u32),
    UpdateCharacterPosition(MessageCharacterBase, u32, u32),
    AddBuild(model::Build),
    AddCharacter(MessageCharacterBase),
    RemoveCharacter(MessageCharacterBase),
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(SendEventMessage),
    Zone(ZoneMessage),
}
