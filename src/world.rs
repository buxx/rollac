use crate::client::Client;
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
    pub fn new(world_raw: &str, tiles: &WorldTiles) -> Result<Self, String> {
        let height = world_raw.lines().count() as i32;
        let longest_line = util::longest_line(world_raw);
        if !longest_line.is_some() {
            return Err(String::from("There is no line in given world source"));
        }

        let width = longest_line.unwrap().chars().count() as i32;
        let mut rows: Vec<WorldRow> = Vec::new();

        for line in world_raw.lines() {
            let mut cols: Vec<String> = Vec::new();

            for tile_char in line.chars() {
                let tile_id = tiles.tile_id(tile_char as u16);
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

    pub fn tile_id(&self, row_i: i32, col_i: i32) -> Option<String> {
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

pub fn new(client: &Client) -> World {
    let world_source = match client.get_world_source() {
        Ok(world_source) => world_source,
        Err(msg) => {
            panic!(msg)
        }
    };
    let legend = match util::extract_block_from_source("LEGEND", world_source.as_str()) {
        Ok(legend) => legend,
        Err(msg) => {
            panic!(msg)
        }
    };
    let world_raw = match util::extract_block_from_source("GEO", world_source.as_str()) {
        Ok(world_raw) => world_raw,
        Err(msg) => {
            panic!(msg)
        }
    };
    let world_tiles = match WorldTiles::new(legend.as_str()) {
        Ok(world_tiles) => world_tiles,
        Err(msg) => {
            panic!(msg)
        }
    };
    let world = match World::new(world_raw.as_str(), &world_tiles) {
        Ok(world) => world,
        Err(msg) => {
            panic!(msg)
        }
    };

    world
}
