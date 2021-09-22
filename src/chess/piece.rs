use super::piece_data::*;
use super::board::*;

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

    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    fn get_colour(&self) -> Colour {
        match self.0 & 1 {
            0 => Colour::White,
            _ => Colour::Black
        }
    }

    // Get possible moves of the piece (TO BE IMPLEMENTED)
    pub fn get_moves(&self, position: &(i8, i8), board: &Board) -> Vec<String> {
        match self.get_type() {
            PieceType::None => (vec![]),
            PieceType::Pawn => {self.get_pawn_moves(position, board)}
            PieceType::Knight => {self.get_knight_moves(position, board)}
            PieceType::Bishop => {self.get_bishop_moves(position, board)}
            PieceType::Rook => {self.get_rook_moves(position, board)}
            PieceType::Queen => {self.get_queen_moves(position, board)}
            PieceType::King => {self.get_king_moves(position, board)}
        }
    }

    
    /// Get the legal moves this pawn piece.
    fn get_pawn_moves(&self, position: &(i8,i8), board: &Board) -> Vec<String> {
        let colour = self.get_colour();
        let move_vector : i8 = if colour == Colour::White {-1} else {1};
        let mut moves : Vec<String> = Vec::with_capacity(2);
        let first_move = self.0 & 0x80;

        let checked_coord = (position.0, (position.1 + move_vector));
        if Board::within_bounds(&checked_coord) && board.piece_at(&checked_coord).is_empty()
        {
            moves.push(Board::convert_coord_pos(&checked_coord));
        }

        if first_move == 0x80 {
            if board.piece_at(&checked_coord).is_empty(){
                let checked_coord = (position.0, (position.1 + move_vector * 2));
                if board.piece_at(&checked_coord).is_empty()
                {
                    moves.push(Board::convert_coord_pos(&checked_coord));
                }
            }
        }

        let checked_coord = (position.0 + move_vector, (position.1 + move_vector));
        if Board::within_bounds(&checked_coord) && 
           !board.piece_at(&checked_coord).is_empty() &&
           board.piece_at(&checked_coord).get_colour() != colour
        {
            moves.push(Board::convert_coord_pos(&checked_coord));
        }

        let checked_coord = (position.0 - move_vector, (position.1 + move_vector));
        if Board::within_bounds(&checked_coord) && 
           !board.piece_at(&checked_coord).is_empty() &&
           board.piece_at(&checked_coord).get_colour() != colour
        {
            moves.push(Board::convert_coord_pos(&checked_coord));
        }

        moves
    }

    /// Get the legal moves this knight piece.
    fn get_knight_moves(&self, position: &(i8,i8), board: &Board) -> Vec<String> {
        vec![] // To be implemented
    }

    /// Get the legal moves this bishop piece.
    fn get_bishop_moves(&self, position: &(i8,i8), board: &Board) -> Vec<String> {
        vec![] // To be implemented
    }

    /// Get the legal moves this rook piece.
    fn get_rook_moves(&self, position: &(i8,i8), board: &Board) -> Vec<String> {
        vec![] // To be implemented
    }

    /// Get the legal moves this queen piece.
    fn get_queen_moves(&self, position: &(i8,i8), board: &Board) -> Vec<String> {
        vec![] // To be implemented
    }

    /// Get the legal moves this king piece.
    fn get_king_moves(&self, position: &(i8,i8), board: &Board) -> Vec<String> {
        vec![] // To be implemented        
    }
}