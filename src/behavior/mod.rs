use crate::ac::{AnimatedCorpse, Type};
use crate::event::ZoneEvent;
use crate::message::Message;
use crate::zone::Zone;

pub mod fear;
pub mod move_;

pub fn get_behaviors_for(
    animated_corpse: &Box<dyn AnimatedCorpse + Send + Sync>,
) -> Vec<Box<dyn Behavior + Send + Sync>> {
    match animated_corpse.type_() {
        Type::HARE => {
            vec![
                Box::new(move_::Move::from_animated_corpse(animated_corpse)),
                Box::new(fear::Fear::from_animated_corpse(animated_corpse)),
            ]
        }
    }
}

pub trait Behavior {
    fn animate_each(&self) -> Option<u8>;
    fn on_event(
        &self,
        animated_corpse: &Box<dyn AnimatedCorpse + Send + Sync>,
        event: &ZoneEvent,
        zone: &Zone,
    ) -> Vec<Message>;
    fn on_animate(
        &self,
        animated_corpse: &Box<dyn AnimatedCorpse + Send + Sync>,
        zone: &Zone,
    ) -> Vec<Message>;
}
