use super::piece_data::*;

/// Piece used in the game, stores colour, piece type and other flags in a u8
pub struct Piece(u8);

impl Piece {
    /// Generate a piece using a u8. Instructions found in the main README
    pub fn from_u8(data: u8) -> Piece {
        Piece(data)
    }

    /// Create a new piece using the Colour and PieceType enums (This is only used for debugging at the moment and might be removed)
    pub fn new(colour: Colour, piece_type: PieceType) -> Piece {
        let mut piece_data : u8 = 0;

        if let Colour::Black = colour {
            piece_data = 1;
        }

        match piece_type {
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

    /// Get the data of the piece in this format "<Colour> <Piece Type>"
    pub fn get_str(&self) -> String {
        let mut piece_name = String::new();

        // 00000000 => no piece
        if self.0 == 0 {
            piece_name.push_str("None");
            return piece_name;
        }

        // Uses mask 00000001 to the colour: Black 1, White 0
        match self.0 & 1 {
            1 => piece_name.push_str("Black"),
            _ => piece_name.push_str("White")
        }

        // Uses mask 00001110 to get the type (read more in the main README)
        match self.0 & 14 {
            2 => piece_name.push_str(" Pawn"),
            4 => piece_name.push_str(" Knight"),
            6 => piece_name.push_str(" Bishop"),
            8 => piece_name.push_str(" Rook"),
            10 => piece_name.push_str(" Queen"),
            12 => piece_name.push_str(" King"),
            _ => ()
        }

        piece_name
    }

    /// Get the type of piece as represented in the PieceType enum
    /// Uses the mask 00001110 to get the bits that represent the type
    fn get_type(&self) -> PieceType {
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

    // Get possible moves of the piece (TO BE IMPLEMENTED)
    fn get_moves(&self, position: &(u8, u8)) -> Vec<String> {
        match self.get_type() {
            PieceType::None => (vec![]),
            PieceType::Pawn => {self.get_pawn_moves(position)}
            PieceType::Knight => {self.get_knight_moves(position)}
            PieceType::Bishop => {self.get_bishop_moves(position)}
            PieceType::Rook => {self.get_rook_moves(position)}
            PieceType::Queen => {self.get_queen_moves(position)}
            PieceType::King => {self.get_king_moves(position)}
        }
    }

    
    /// Get the legal moves this pawn piece.
    fn get_pawn_moves(&self, position: &(u8,u8)) -> Vec<String> {
        vec![] // To be implemented
    }

    /// Get the legal moves this knight piece.
    fn get_knight_moves(&self, position: &(u8,u8)) -> Vec<String> {
        vec![] // To be implemented
    }

    /// Get the legal moves this bishop piece.
    fn get_bishop_moves(&self, position: &(u8,u8)) -> Vec<String> {
        vec![] // To be implemented
    }

    /// Get the legal moves this rook piece.
    fn get_rook_moves(&self, position: &(u8,u8)) -> Vec<String> {
        vec![] // To be implemented
    }

    /// Get the legal moves this queen piece.
    fn get_queen_moves(&self, position: &(u8,u8)) -> Vec<String> {
        vec![] // To be implemented
    }

    /// Get the legal moves this king piece.
    fn get_king_moves(&self, position: &(u8,u8)) -> Vec<String> {
        vec![] // To be implemented        
    }
}