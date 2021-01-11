use crate::ac;
use crate::ac::hare::Hare;
use crate::event::ZoneEvent;
use crate::message::Message;

pub struct Zone {
    world_row_i: u32,
    world_col_i: u32,
    animated_corpses: Vec<Box<dyn ac::AnimatedCorpse + Send + Sync>>,
}

impl Zone {
    pub fn new(
        world_row_i: u32,
        world_col_i: u32,
        animated_corpses: Vec<Box<dyn ac::AnimatedCorpse + Send + Sync>>,
    ) -> Self {
        Zone {
            world_row_i,
            world_col_i,
            animated_corpses,
        }
    }

    pub fn react(&mut self, event: &ZoneEvent) -> Vec<Message> {
        let mut messages: Vec<Message> = vec![];

        // TODO: ici manage pop d'un nouveau ac; + task::spawn
        println!("event recu: {:?}", event);
        for animated_corpse in self.animated_corpses.iter_mut() {
            if let Some(animated_corpse_messages) = animated_corpse.apply_event(event) {
                messages.extend(animated_corpse_messages);
            }
        }

        messages
    }

    pub fn animate(&mut self) -> Vec<Message> {
        let mut messages: Vec<Message> = vec![];

        for animated_corpse in self.animated_corpses.iter_mut() {
            if let Some(animated_corpse_messages) = animated_corpse.animate() {
                messages.extend(animated_corpse_messages);
            }
        }

        messages
    }
}
