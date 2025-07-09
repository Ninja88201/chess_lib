use crate::Tile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disambig {
    File(u8),        
    Rank(u8),        
    FileRank(Tile) 
}

impl std::fmt::Display for Disambig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Disambig::File(c) => write!(f, "{}", Tile::FILE_CHARS[*c as usize]),
            Disambig::Rank(c) => write!(f, "{}", Tile::RANK_CHARS[*c as usize]),
            Disambig::FileRank(t) => write!(f, "{}", t),
        }
    }
}