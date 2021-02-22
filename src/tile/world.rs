use crate::error;
use crate::tile::TileId;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WorldTiles {
    codes: HashMap<u16, TileId>,
    pub default: Option<TileId>,
}

impl WorldTiles {
    pub fn new(legend: &str) -> Result<Self, error::Error> {
        let mut default_tile_id: Option<TileId> = None;

        let mut codes: HashMap<u16, TileId> = HashMap::new();

        for line in legend.lines() {
            let mut split = line.split_ascii_whitespace();
            let char_ = split
                .next()
                .ok_or(error::Error::new(format!(
                    "Unable to split tile line '{}'",
                    line
                )))?
                .trim()
                .chars()
                .nth(0)
                .ok_or(error::Error::new(format!(
                    "Unable to read char from line '{}'",
                    line
                )))? as u16;
            let mut id = split
                .next()
                .ok_or(error::Error::new(format!(
                    "Unable to read second tile part from line '{}'",
                    line
                )))?
                .trim();
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

    pub fn tile_id(&self, code: u16) -> Result<String, error::Error> {
        Ok(self
            .codes
            .get(&code)
            .ok_or(error::Error::new(format!(
                "Unable to find tile_id for code '{}'",
                code
            )))?
            .clone())
    }
}
