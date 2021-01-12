use crate::event::ZoneEvent;
use crate::message::Message;
use crate::tile::zone::ZoneTiles;
use crate::tile::TileId;
use crate::{ac, util};

#[derive(Debug)]
pub struct LevelRow {
    pub cols: Vec<String>,
}

pub struct Zone {
    pub world_row_i: u32,
    pub world_col_i: u32,
    pub animated_corpses: Vec<Box<dyn ac::AnimatedCorpse + Send + Sync>>,
    pub width: i32,
    pub height: i32,
    pub rows: Vec<LevelRow>,
    pub world_tile_type_id: TileId,
}

impl Zone {
    pub fn new(
        world_row_i: u32,
        world_col_i: u32,
        animated_corpses: Vec<Box<dyn ac::AnimatedCorpse + Send + Sync>>,
        zone_raw: &str,
        tiles: &ZoneTiles,
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
            width,
            height,
            rows,
            world_tile_type_id,
        })
    }

    pub fn react(&mut self, event: &ZoneEvent) -> Vec<Message> {
        let mut messages: Vec<Message> = vec![];

        // TODO: ici manage pop d'un nouveau ac; + task::spawn
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

    pub fn tile_id(&self, row_i: i16, col_i: i16) -> TileId {
        if col_i < 0 || row_i < 0 {
            return String::from("NOTHING");
        }

        if row_i >= self.rows.len() as i16 {
            return String::from("NOTHING");
        }

        let row = &self.rows[row_i as usize];

        if col_i >= row.cols.len() as i16 {
            return String::from("NOTHING");
        }

        row.cols[col_i as usize].clone()
    }
}
