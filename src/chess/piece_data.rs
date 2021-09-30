/// Pieces used in chess
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PieceType {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

/// Colours of piece and of the current turn
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Colour {
    White,
    Black
}

