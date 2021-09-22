/// Pieces used in chess
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
#[derive(PartialEq)]
pub enum Colour {
    White,
    Black
}

