use crate::ac;
use crate::ac::rabbit::Rabbit;
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

    pub fn react(&mut self) -> Vec<Message> {
        let mut messages: Vec<Message> = vec![];

        for animated_corpse in self.animated_corpses.iter_mut() {
            if let Some(animated_corpse_messages) = animated_corpse.apply_event() {
                messages.extend(animated_corpse_messages);
            }
        }

        self.animated_corpses.push(Box::new(Rabbit::new(0, 0)));
        println!(
            "react event (animated_corpse: {})",
            self.animated_corpses.len()
        );
        messages.push(Message::HelloWorldZone);

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
