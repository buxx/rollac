use crate::client::Client;
use crate::error;
use crate::tile::world::WorldTiles;
use crate::util;

#[derive(Debug, Clone)]
pub struct WorldRow {
    pub cols: Vec<String>,
}

#[derive(Clone)]
pub struct World {
    pub width: i32,
    pub height: i32,
    pub rows: Vec<WorldRow>,
}

impl World {
    pub fn new(world_raw: &str, tiles: &WorldTiles) -> Result<Self, error::Error> {
        let height = world_raw.lines().count() as i32;
        let longest_line = if let Some(longest_line) = util::longest_line(world_raw) {
            longest_line
        } else {
            return Err(error::Error::new("World raw seem to be empty".to_string()));
        };

        let width = longest_line.chars().count() as i32;
        let mut rows: Vec<WorldRow> = Vec::new();

        for line in world_raw.lines() {
            let mut cols: Vec<String> = Vec::new();

            for tile_char in line.chars() {
                let tile_id = tiles.tile_id(tile_char as u16)?;
                cols.push(tile_id);
            }

            let world_row = WorldRow { cols };
            rows.push(world_row);
        }

        Ok(Self {
            width,
            height,
            rows,
        })
    }

    pub fn _tile_id(&self, row_i: i32, col_i: i32) -> Option<String> {
        if row_i < 0 || col_i < 0 {
            return None;
        }

        if let Some(row) = self.rows.get(row_i as usize) {
            if col_i as usize >= row.cols.len() {
                return None;
            }

            return Some(row.cols[col_i as usize].clone());
        }

        None
    }
}

pub fn new(client: &Client) -> Result<World, error::Error> {
    let world_source = client.get_world_source()?;
    let legend = util::extract_block_from_source("LEGEND", world_source.as_str())?;
    let world_raw = util::extract_block_from_source("GEO", world_source.as_str())?;
    let world_tiles = WorldTiles::new(legend.as_str())?;
    Ok(World::new(world_raw.as_str(), &world_tiles)?)
}
