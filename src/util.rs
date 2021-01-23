pub const BLOCK_GEO: &str = "GEO";

pub fn extract_block_from_source(block_name: &str, source: &str) -> Result<String, String> {
    let mut block_found = false;
    let mut block_lines: Vec<&str> = Vec::new();

    for line in source.lines() {
        if line.starts_with("::") {
            // TODO BS 2019-04-03: there is strip method ?
            let line_block_name = line.replace("::", "").replace("\n", "").replace(" ", "");
            if line_block_name == block_name {
                block_found = true;
            } else if block_found {
                return Ok(block_lines.join("\n"));
            }
        } else if block_found {
            block_lines.push(line);
        }
    }

    if block_found {
        return Ok(block_lines.join("\n"));
    }
    Err(format!("Block \"{}\" not found", block_name))
}

pub fn longest_line(text: &str) -> Option<&str> {
    let mut max_length = 0;
    let mut longest_line: Option<&str> = None;

    for line in text.lines() {
        let contents = line.trim_end();
        let line_length = contents.len();
        if line_length > max_length {
            max_length = line_length;
            longest_line = Some(contents);
        }
    }

    longest_line
}

#[derive(Debug)]
pub enum Direction {
    North,
    NorthEst,
    Est,
    SouthEst,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub fn direction_modifier(direction: Direction) -> (i8, i8) {
    match direction {
        Direction::North => (-1, 0),
        Direction::NorthEst => (-1, 1),
        Direction::Est => (0, 1),
        Direction::SouthEst => (1, 1),
        Direction::South => (1, 0),
        Direction::SouthWest => (1, -1),
        Direction::West => (0, -1),
        Direction::NorthWest => (-1, -1),
    }
}

pub fn opposite_direction(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::NorthEst => Direction::SouthWest,
        Direction::Est => Direction::West,
        Direction::SouthEst => Direction::NorthWest,
        Direction::South => Direction::North,
        Direction::SouthWest => Direction::NorthEst,
        Direction::West => Direction::Est,
        Direction::NorthWest => Direction::SouthEst,
    }
}

pub fn position_direction_from(reference: (u32, u32), position: (u32, u32)) -> Option<Direction> {
    let row_modifier: i32 = position.0 as i32 - reference.0 as i32;
    let col_modifier: i32 = position.1 as i32 - reference.1 as i32;

    match (row_modifier, col_modifier) {
        (-2, -2) => Some(Direction::North),
        (-2, -1) => Some(Direction::North),
        (-2, 0) => Some(Direction::North),
        (-2, 1) => Some(Direction::North),
        (-2, 2) => Some(Direction::North),

        (-1, -2) => Some(Direction::West),
        (-1, -1) => Some(Direction::North),
        (-1, 0) => Some(Direction::North),
        (-1, 1) => Some(Direction::North),
        (-1, 2) => Some(Direction::Est),

        (0, -2) => Some(Direction::West),
        (0, -1) => Some(Direction::West),
        (0, 0) => None,
        (0, 1) => Some(Direction::Est),
        (0, 2) => Some(Direction::Est),

        (1, -2) => Some(Direction::West),
        (1, -1) => Some(Direction::South),
        (1, 0) => Some(Direction::South),
        (1, 1) => Some(Direction::South),
        (1, 2) => Some(Direction::Est),

        (2, -2) => Some(Direction::South),
        (2, -1) => Some(Direction::South),
        (2, 0) => Some(Direction::South),
        (2, 1) => Some(Direction::South),
        (2, 2) => Some(Direction::South),

        (_, _) => None,
    }
}

pub fn is_near(position1: (u32, u32), position2: (u32, u32), distance: u32) -> bool {
    let row_distance = (position1.0 as i32 - position2.0 as i32).abs() as u32;
    let col_distance = (position1.1 as i32 - position2.1 as i32).abs() as u32;
    row_distance <= distance && col_distance <= distance
}
