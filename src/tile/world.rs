use crate::tile::TileId;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WorldTiles {
    codes: HashMap<u16, TileId>,
    pub default: Option<TileId>,
}

impl WorldTiles {
    pub fn new(legend: &str) -> Result<Self, String> {
        let mut default_tile_id: Option<TileId> = None;

        let mut codes: HashMap<u16, TileId> = HashMap::new();

        for line in legend.lines() {
            let mut split = line.split_ascii_whitespace();
            let char_ = split.next().unwrap().trim().chars().nth(0).unwrap() as u16;
            let mut id = split.next().unwrap().trim();
            if id.ends_with("*") {
                id = id.trim_end_matches("*");
                default_tile_id = Some(id.to_string());
            }

            codes.insert(char_, id.to_string());
        }

        Ok(WorldTiles {
            codes,
            default: default_tile_id,
        })
    }

    pub fn tile_id(&self, code: u16) -> String {
        self.codes.get(&code).unwrap().clone()
    }
}
