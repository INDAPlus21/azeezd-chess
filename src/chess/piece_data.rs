/// Pieces used in chess
#[derive(PartialEq, Debug)]
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
#[derive(PartialEq, Debug)]
pub enum Colour {
    White,
    Black
}

