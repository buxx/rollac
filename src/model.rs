use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Build {
    pub id: u32,
    pub build_id: String,
    pub row_i: u32,
    pub col_i: u32,
    pub traversable: HashMap<String, bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub id: String,
    pub zone_row_i: u32,
    pub zone_col_i: u32,
}
