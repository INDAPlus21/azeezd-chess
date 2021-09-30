use super::piece_data::*;

/// Directions used for different movements as a vectorial value for left, right, up and down
pub const DIRECTIONS : [i8; 2] = [-1, 1];

/// Piece struct, holds all data for a piece, accessed through methods
#[derive(Copy, Clone)]
pub struct Piece(u8);

impl Piece {

    /// Create piece from a u8
    pub fn from_u8(_piece_data: u8) -> Piece {
        Piece(_piece_data)
    }

    pub fn new(_colour: Colour, _piece_type: PieceType) -> Piece {
        let mut piece_data : u8 = 0;

        if let Colour::Black = _colour {
            piece_data = 1;
        }

        match _piece_type {
            PieceType::None => (),
            PieceType::Pawn => {piece_data += 2}
            PieceType::Knight => {piece_data += 4}
            PieceType::Bishop => {piece_data += 6}
            PieceType::Rook => {piece_data += 8}
            PieceType::Queen => {piece_data += 10}
            PieceType::King => {piece_data += 12}
        }
        
        Piece(piece_data)
    }

    /// Get the colour of the piece
    pub fn get_colour(&self) -> Colour {
        // Mask with 00000001 to get colour bit
        match self.0 & 1 {
            0 => Colour::White,
            _ => Colour::Black
        }
    }

    /// Get the type of the piece
    pub fn get_type(&self) -> PieceType {
        // Use the 00001110 mask to get piece type bits
        match self.0 & 14 {
            2 => PieceType::Pawn,
            4 => PieceType::Knight,
            6 => PieceType::Bishop,
            8 => PieceType::Rook,
            10 => PieceType::Queen,
            12 => PieceType::King,
            _ => PieceType::None
        }
    }

    /// Return the piece type and colour as a tuple
    pub fn get_piece_data(&self) -> (Colour, PieceType) {
        (self.get_colour(), self.get_type())
    }

    // Sets the type of the piece, used for promotion
    pub fn set_type(&mut self, _piece_type: PieceType) {
        // Use the 11110001 mask to remove all piece bits
        let data = self.0 & 0xf1;

        // Add the piece bit
        match _piece_type {
            PieceType::None => {self.0 = 0}
            PieceType::Pawn => {self.0 = data | 2},
            PieceType::Knight => {self.0 = data | 4},
            PieceType::Bishop => {self.0 = data | 6},
            PieceType::Rook => {self.0 = data | 8},
            PieceType::Queen => {self.0 = data | 10},
            PieceType::King => {self.0 = data | 12}
        }
    }

    /// Set the piece type to none. Used for removing a piece from a square, setting that square to None
    pub fn set_to_none(&mut self) {
        self.set_type(PieceType::None);
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }

    pub fn get_icon(&self) -> char {
        // Uses mask 00001110 to get the type (read more in the main README)
        match self.0 & 14 {
            2 => 'P',
            4 => 'N',
            6 => 'B',
            8 => 'R',
            10 => 'Q',
            12 => 'K',
            _ => '■'
        }
    }

    pub fn set_data (&mut self, u8_data: u8) {
        self.0 = u8_data;
    }
}