use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveError {
    NoPieceSelected,
    SameTile,
    FriendlyCapture,
    IllegalMove,
    WrongTurn,
    PiecePinned,
    Stalemate,
    Checkmate,
    Cancelled,
}
impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MoveError::IllegalMove => write!(f, "Illegal move"),
            MoveError::WrongTurn => write!(f, "It's the wrong player's turn"),
            MoveError::PiecePinned => write!(f, "Piece is pinned"),
            MoveError::Stalemate => write!(f, "The board is in a stalemate"),
            MoveError::Checkmate => write!(f, "The board is in a checkmate"),
            MoveError::NoPieceSelected => write!(f, "No piece is selected"),
            MoveError::SameTile => write!(f, "Same tile selected"),
            MoveError::FriendlyCapture => write!(f, "Cannot capture own piece"),
            MoveError::Cancelled => write!(f, "Cancelled move"),
        }
    }
}