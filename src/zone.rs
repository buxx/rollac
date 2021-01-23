use crate::behavior::get_behaviors_for;
use crate::event::{ZoneEvent};
use crate::message::{Message, ZoneMessage};
use crate::model::Character;
use crate::tile::zone::{ZoneTiles, NOTHING};
use crate::tile::TileId;
use crate::{ac, model, util};

#[derive(Debug)]
pub struct LevelRow {
    pub cols: Vec<String>,
}

pub struct Zone {
    pub world_row_i: u32,
    pub world_col_i: u32,
    pub animated_corpses: Vec<Box<dyn ac::AnimatedCorpse + Send + Sync>>,
    pub characters: Vec<model::Character>,
    pub builds: Vec<model::Build>,
    pub width: i32,
    pub height: i32,
    pub rows: Vec<LevelRow>,
    pub world_tile_type_id: TileId,
    pub tiles: ZoneTiles,
}

impl Zone {
    pub fn new(
        world_row_i: u32,
        world_col_i: u32,
        animated_corpses: Vec<Box<dyn ac::AnimatedCorpse + Send + Sync>>,
        characters: Vec<model::Character>,
        builds: Vec<model::Build>,
        zone_raw: &str,
        tiles: ZoneTiles,
        world_tile_type_id: String,
    ) -> Result<Self, String> {
        let height = zone_raw.lines().count() as i32;
        let longest_line = util::longest_line(zone_raw);
        if !longest_line.is_some() {
            return Err("There is no line in given zone source".to_string());
        }

        let width = longest_line.unwrap().chars().count() as i32;
        let mut rows: Vec<LevelRow> = Vec::new();

        for line in zone_raw.lines() {
            let mut cols: Vec<String> = Vec::new();

            for tile_char in line.chars() {
                let tile_id = tiles.tile_id(tile_char as u16);
                cols.push(tile_id);
            }

            let level_row = LevelRow { cols };
            rows.push(level_row);
        }

        Ok(Zone {
            world_row_i,
            world_col_i,
            animated_corpses,
            characters,
            builds,
            width,
            height,
            rows,
            world_tile_type_id,
            tiles,
        })
    }

    pub fn on_event(&mut self, event: &ZoneEvent) -> Vec<Message> {
        let mut messages: Vec<Message> = vec![];

        for animated_corpse in self.animated_corpses.iter() {
            for message_ in animated_corpse.on_event(event, self) {
                messages.push(message_);
            }

            for behavior in get_behaviors_for(animated_corpse).iter() {
                for message_ in behavior.on_event(animated_corpse, event, self) {
                    messages.push(message_);
                }
            }
        }

        messages
    }

    pub fn animate(&self, tick_count: u64) -> Vec<Message> {
        let mut messages: Vec<Message> = vec![];

        for animated_corpse in self.animated_corpses.iter() {
            for message_ in animated_corpse.animate(self, tick_count) {
                messages.push(message_)
            }

            for behavior in get_behaviors_for(animated_corpse).iter() {
                if let Some(animate_each) = behavior.animate_each() {
                    if tick_count % animate_each as u64 == 0 {
                        for message_ in behavior.on_animate(animated_corpse, self) {
                            messages.push(message_);
                        }
                    }
                }
            }
        }

        messages
    }

    pub fn on_message(&mut self, message: ZoneMessage) {
        match message {
            ZoneMessage::UpdateAnimatedCorpsePosition(_, _, _) => {
                for animated_corpse in self.animated_corpses.iter_mut() {
                    match message {
                        ZoneMessage::UpdateAnimatedCorpsePosition(
                            animated_corpse_id,
                            zone_row_id,
                            zone_col_id,
                        ) => {
                            if animated_corpse.id() == animated_corpse_id {
                                animated_corpse.set_zone_row_i(zone_row_id);
                                animated_corpse.set_zone_col_i(zone_col_id);
                            }
                        }
                        _ => {}
                    }
                }
            }
            ZoneMessage::UpdateCharacterPosition(character_id, to_row_i, to_col_i) => {
                for character in self.characters.iter_mut() {
                    if character.id == character_id {
                        character.zone_row_i = to_row_i;
                        character.zone_col_i = to_col_i;
                    }
                }
            }
            ZoneMessage::RemoveCharacter(character_id) => {
                if let Some(position_to_remove) = self
                    .characters
                    .iter()
                    .position(|character| character.id == character_id)
                {
                    self.characters.remove(position_to_remove);
                }
            }
            ZoneMessage::AddCharacter(character_id, row_i, col_i) => {
                self.characters.push(Character {
                    id: character_id,
                    zone_row_i: row_i,
                    zone_col_i: col_i,
                })
            }
            ZoneMessage::AddBuild(build) => {
                self.builds.push(build);
            }
        }
    }

    pub fn tile_id(&self, row_i: u32, col_i: u32) -> TileId {
        if row_i >= self.rows.len() as u32 {
            return String::from(NOTHING);
        }

        let row = &self.rows[row_i as usize];

        if col_i >= row.cols.len() as u32 {
            return String::from(NOTHING);
        }

        row.cols[col_i as usize].clone()
    }

    pub fn get_successors(&self, row_i: u32, col_i: u32) -> Vec<((u32, u32), u32)> {
        let mut successors = vec![];
        let row_i = row_i as i32;
        let col_i = col_i as i32;

        for (modifier_row_i, modifier_col_i) in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            // no center pos
            (0, 1),
            (1, 1),
            (1, -1),
            (1, 0),
        ]
        .iter()
        {
            let new_row_i = row_i + *modifier_row_i;
            let new_col_i = col_i + *modifier_col_i;

            // Ignore outside coordinates
            if new_row_i >= 0 && new_col_i >= 0 {
                if self
                    .tiles
                    .browseable(&self.tile_id(new_row_i as u32, new_col_i as u32))
                {
                    successors.push(((new_row_i as u32, new_col_i as u32), 1));
                }
            }
        }

        successors
    }
}
