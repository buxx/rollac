use crate::ac::AnimatedCorpseBase;

#[derive(Debug, Clone, Copy)]
pub enum SendEventMessage {
    RequireMove(AnimatedCorpseBase, u32, u32),
}

#[derive(Debug, Clone, Copy)]
pub enum AnimatedCorpseMessage {
    UpdateZonePosition(AnimatedCorpseBase, u32, u32),
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Event(SendEventMessage),
    AnimatedCorpse(AnimatedCorpseMessage),
}
