use serde_json::Value;
use std::collections::HashMap;

use crate::error;
use crate::tile::TileId;

#[derive(Debug)]
pub struct ZoneTiles {
    codes: HashMap<u16, TileId>,
    browseables: HashMap<TileId, bool>,
}

pub const NOTHING: &str = "NOTHING";
pub const UNKNOWN: &str = "UNKNOWN";

impl ZoneTiles {
    pub fn new(data: Value) -> Result<Self, error::Error> {
        let mut codes = HashMap::new();
        let mut browseables = HashMap::new();

        for tile_value in data.as_array().ok_or(error::Error::new(format!(
            "Unable to parse ZoneTiles array from '{}'",
            data
        )))? {
            let tile_id: &str = tile_value["id"].as_str().ok_or(error::Error::new(format!(
                "Unable to find tile id in '{}'",
                tile_value
            )))?;
            let char: u16 = tile_value["char"]
                .as_str()
                .ok_or(error::Error::new(format!(
                    "Unable to find tile char in '{}'",
                    tile_value
                )))?
                .chars()
                .nth(0)
                .ok_or(error::Error::new(format!(
                    "Unable to find tile id in '{}'",
                    tile_value
                )))? as u16;

            codes.insert(char, tile_id.to_string());
            // TODO evolve browseables schema (WALKING, etc)
            if let Some(traversable) = tile_value["traversable"].as_object() {
                if let Some(walking) = traversable.get("WALKING") {
                    if let Some(can_walk) = walking.as_bool() {
                        browseables.insert(tile_id.to_string(), can_walk);
                    }
                }
            }
        }

        Ok(ZoneTiles { codes, browseables })
    }

    pub fn tile_id(&self, code: u16) -> String {
        if let Some(tile_id) = self.codes.get(&code) {
            return String::from(tile_id);
        }
        String::from(UNKNOWN)
    }

    pub fn browseable(&self, tile_id: &str) -> bool {
        if let Some(browseable) = self.browseables.get(tile_id) {
            return *browseable;
        }
        false
    }
}
