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
