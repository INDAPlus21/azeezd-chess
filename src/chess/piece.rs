use super::piece_data::*;
use super::board::*;

/// Array that stores directions.
/// Used for iterating through different directions a piece can move.
const DIRECTIONS : [i8; 2] = [-1, 1];

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

        // Get the direction which the pawns moves, based on colour
        let move_vector : i8 = if colour == Colour::White {-1} else {1};

        // Store legal moves (is the returned value)
        let mut moves : Vec<String> = Vec::with_capacity(2);

        // Uses mask 1000000 to get the "is this the first move" bit flag
        let first_move = self.0 & 0x80;

        // The classical one step forward move
        let checked_coord = (position.0, position.1 + move_vector);
        if Board::within_bounds(&checked_coord) && board.piece_at(&checked_coord).is_empty()
        {
            moves.push(Board::convert_coord_pos(&checked_coord));
        }

        // If the first move bit flag is on then check and add the two-step pawn move
        if first_move == 0x80 {
            if board.piece_at(&checked_coord).is_empty(){
                let checked_coord = (position.0, position.1 + move_vector * 2);
                if board.piece_at(&checked_coord).is_empty()
                {
                    moves.push(Board::convert_coord_pos(&checked_coord));
                }
            }
        }

        // Check for possible on right side
        let checked_coord = (position.0 + move_vector, position.1 + move_vector);
        if Board::within_bounds(&checked_coord) && 
           !board.piece_at(&checked_coord).is_empty() &&
           board.piece_at(&checked_coord).get_colour() != colour
        {
            moves.push(Board::convert_coord_pos(&checked_coord));
        }

        // Check for possible attacks on the left side
        let checked_coord = (position.0 - move_vector, position.1 + move_vector);
        if Board::within_bounds(&checked_coord) && 
           !board.piece_at(&checked_coord).is_empty() &&
           board.piece_at(&checked_coord).get_colour() != colour
        {
            moves.push(Board::convert_coord_pos(&checked_coord));
        }

        // En passant to be added

        moves
    }

    /// Get the legal moves this knight piece.
    fn get_knight_moves(&self, position: &(i8,i8), board: &Board) -> Vec<String> {
        let colour = self.get_colour();

        // Store moves. Returned value
        let mut moves : Vec<String> = Vec::with_capacity(2);
        
        /* A knight can move up to maximum of 8 ways.
           It is the L shaped move in 2 ways: 
           2 for the front side's left and right = 4
           2 for the back's side left and right = 4
        */
        for direction_x in DIRECTIONS { // front and back
            for direction_y in DIRECTIONS { // left and right
                for l_long_side in 0..2 { // two directions of the L shape
                    let checked_coord = (position.0 + direction_x * (1 + l_long_side), 
                                        position.1 + direction_y * (2 - l_long_side));
                    if Board::within_bounds(&checked_coord) && (
                    board.piece_at(&checked_coord).is_empty() ||
                    board.piece_at(&checked_coord).get_colour() != colour)
                    {
                        moves.push(Board::convert_coord_pos(&checked_coord));
                    } 
                }
            }
        }

        moves
    }

    /// Get the legal moves this bishop piece.
    fn get_bishop_moves(&self, position: &(i8,i8), board: &Board) -> Vec<String> {
        let colour = self.get_colour();

        // Store moves. Returned value
        let mut moves : Vec<String> = Vec::with_capacity(8);

        // Moves in all 4 diagonal directions until reaching a piece.
        // If enemy piece, add all squares to last including the last
        // If ally piece, add all squares to last, excluding the last
        for direction_y in DIRECTIONS { // up and down
            for direction_x in DIRECTIONS { // left and right
                for square in 1..9 { // loop from min to max amount of moves for a bishop per direction
                    let checked_coord = (position.0 + direction_x * square, position.1 + direction_y * square);
                    if Board::within_bounds(&checked_coord) {
                        let current_square = board.piece_at(&checked_coord);
                        if current_square.is_empty() { // If the square being checked is empty add to legal moves
                            moves.push(Board::convert_coord_pos(&checked_coord));
                        }
                        else { // Reached a non-empty square!
                            if current_square.get_colour() != colour { // If opposite colour, add to legal moves (i.e can attack opponent)
                                moves.push(Board::convert_coord_pos(&checked_coord));
                            }
                            break; // When reaching the non-empty square, break and go to next diagonal direction (if any left)
                        }
                    }
                    else { break; } // If outside bound, break and go to next diagonal direction (if any left)
                }   
            }
        }

        moves
    }

    /// Get the legal moves this rook piece.
    fn get_rook_moves(&self, position: &(i8,i8), board: &Board) -> Vec<String> {
        let colour = self.get_colour();

        // Store moves
        let mut moves : Vec<String> = Vec::with_capacity(8);

        // Move in horizontal and vertical directions, each axis has two sides from the rook thus 4 sides total
        // Move until reaching a non-empty square:
        // If enemy piece, add all squares to last including the last
        // If ally piece, add all squares to last, excluding the last
        for axis in DIRECTIONS { // Horizontal -1 or Vertical 1
            for direction in DIRECTIONS { // left & right for horizontal, up & down for vertical
                for square in 1..9 {
                    // Current squared, mutable due to be added to later to determine next square
                    let mut checked_coord = (position.0, position.1);

                    // If horizontal then increment (or decrement) in horizontal axis
                    if axis == -1 { checked_coord.0 += square * direction; }
                    else { checked_coord.1 += square * direction; } // Else increment (or decrement) in vertical axis 

                    if Board::within_bounds(&checked_coord) {
                        let current_square = board.piece_at(&checked_coord);
                        if current_square.is_empty() { // If the square being checked is empty add to legal moves
                            moves.push(Board::convert_coord_pos(&checked_coord));
                        }
                        else { // Reached a non-empty square!
                            if current_square.get_colour() != colour { // If opposite colour, add to legal moves (i.e can attack opponent)
                                moves.push(Board::convert_coord_pos(&checked_coord));
                            }
                            break; // When reaching the non-empty square, break and go to next diagonal direction (if any left)
                        }
                    }
                    else { break; } // If outside bound, break and go to next diagonal direction (if any left)
                }
            }
        }

        moves
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