pub type AnimatedCorpseInfo = (u32, u32, u32); // id, world_row_i, world_col_i

#[derive(Debug, Clone, Copy)]
pub enum Message {
    RequireMove(AnimatedCorpseInfo, u32, u32),
}
