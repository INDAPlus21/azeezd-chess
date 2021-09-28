use super::piece_data::*;
use super::board::*;

/// Array that stores directions.
/// Used for iterating through different directions a piece can move.
const DIRECTIONS : [i8; 2] = [-1, 1];

/// Piece used in the game, stores colour, piece type and other flags in a u8
#[derive(Copy, Clone)]
pub struct Piece(pub (super) u8);

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

    /// Get the type of piece as represented in the PieceType enum
    /// Uses the mask 00001110 to get the bits that represent the type
    pub fn get_type(&self) -> PieceType {
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

    pub fn set_type(&mut self, piece_type: PieceType) {
        // Gets a typeless piece by setting its type bits to 000 by using the mask 111100011 (f1 in HEX)
        let masked = self.0 & 0xf1;

        // Adding the new piece type into the masks
        match piece_type {
            PieceType::None => {self.0 = 0},
            PieceType::Pawn => {self.0 = masked | 2}
            PieceType::Knight => {self.0 = masked | 4}
            PieceType::Bishop => {self.0 = masked | 6}
            PieceType::Rook => {self.0 = masked | 8}
            PieceType::Queen => {self.0 = masked | 10}
            PieceType::King => {self.0 = masked | 12}
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn get_colour(&self) -> Colour {
        match self.0 & 1 {
            0 => Colour::White,
            _ => Colour::Black
        }
    }

    // === MOVES ===

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
            self.add_move_if_legal(&mut moves, position, &checked_coord, *board);
        }

        // If the first move bit flag is on then check and add the two-step pawn move
        if first_move == 0x80 {
            if board.piece_at(&checked_coord).is_empty(){
                let checked_coord = (position.0, position.1 + move_vector * 2);
                if board.piece_at(&checked_coord).is_empty()
                {
                    self.add_move_if_legal(&mut moves, position, &checked_coord, *board);
                }
            }
        }

        // Check for possible on right side
        let checked_coord = (position.0 + move_vector, position.1 + move_vector);
        if Board::within_bounds(&checked_coord) && 
           !board.piece_at(&checked_coord).is_empty() &&
           board.piece_at(&checked_coord).get_colour() != colour
        {
            self.add_move_if_legal(&mut moves, position, &checked_coord, *board);
        }

        // Check for possible attacks on the left side
        let checked_coord = (position.0 - move_vector, position.1 + move_vector);
        if Board::within_bounds(&checked_coord) && 
           !board.piece_at(&checked_coord).is_empty() &&
           board.piece_at(&checked_coord).get_colour() != colour
        {
            self.add_move_if_legal(&mut moves, position, &checked_coord, *board);
        }

        // Check if pawn is in its fifth rank for en passant
        if position.0 == 3 || position.0 == 4 {
            for direction in DIRECTIONS {
                let checked_coord = (position.0 + direction, position.1 + move_vector);
                let en_passanting_piece = (position.0 + direction, position.1);
                
                if Board::within_bounds(&en_passanting_piece) && Board::within_bounds(&checked_coord)
                   && board.piece_at(&en_passanting_piece).0 & 0x20 == 0x20
                   && board.piece_at(&checked_coord).is_empty()
                {
                    self.add_move_if_legal(&mut moves, position, &checked_coord, *board);
                }
            }
        }

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
                        self.add_move_if_legal(&mut moves, position, &checked_coord, *board);
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
                            self.add_move_if_legal(&mut moves, position, &checked_coord, *board);
                        }
                        else { // Reached a non-empty square!
                            if current_square.get_colour() != colour { // If opposite colour, add to legal moves (i.e can attack opponent)
                                self.add_move_if_legal(&mut moves, position, &checked_coord, *board);
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
                            self.add_move_if_legal(&mut moves, position, &checked_coord, *board);
                        }
                        else { // Reached a non-empty square!
                            if current_square.get_colour() != colour { // If opposite colour, add to legal moves (i.e can attack opponent)
                                self.add_move_if_legal(&mut moves, position, &checked_coord, *board);
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
        let mut moves : Vec<String> = Vec::new();

        moves.append(&mut self.get_bishop_moves(position, board));
        moves.append(&mut self.get_rook_moves(position, board));

        moves
    }

    /// Get the legal moves this king piece.
    fn get_king_moves(&self, position: &(i8,i8), board: &Board) -> Vec<String> {
        let colour = self.get_colour();

        // Store moves
        let mut moves : Vec<String> = Vec::with_capacity(4);

        // Check the 3x3 square around the king
        for col in -1..2 {
            for row in -1..2 {
                let checked_coord = (position.0 + col, position.1 + row);
                if Board::within_bounds(&checked_coord) && 
                   !board.piece_at(&checked_coord).is_empty() &&
                   board.piece_at(&checked_coord).get_colour() != colour
                {
                    self.add_move_if_legal(&mut moves, position, &checked_coord, *board);
                } 
            }
        }
        

        // Castling: Done by checking if king and rooks haven't moved before and king is not in check
        // Then checking if the path the king takes is not threatened
        if self.0 & 0x80 == 0x80 {
            let rank = if colour == Colour::White {7} else {0};

            if !board.king_in_check(&colour) {
                if board.piece_at(&(rank, 7)).0 & 0x80 == 0x80 {
                    let mut new_board = *board;
                    let mut checked = false;

                    for file_move in 0..2 {
                        let new_pos = (rank, 5 + file_move);
                        if new_board.piece_at(&new_pos).is_empty() {break;}
                        new_board.make_move(Board::convert_coord_pos(&(rank, 4 + file_move)), Board::convert_coord_pos(&new_pos));
                        if new_board.king_in_check(&colour) {
                            checked = true;
                            break;
                        }
                    }

                    if !checked {
                        moves.push(Board::convert_coord_pos(&(rank, 4 + 1)));
                    }
                }

                if board.piece_at(&(rank, 0)).0 & 0x80 == 0x80 {
                    let mut new_board = *board;
                    let mut checked = false;

                    for file_move in 0..4 {
                        let new_pos = (rank, 5 + file_move);
                        if new_board.piece_at(&new_pos).is_empty() {break;}
                        new_board.make_move(Board::convert_coord_pos(&(rank, 4 - file_move)), Board::convert_coord_pos(&new_pos));
                        if new_board.king_in_check(&colour) {
                            checked = true;
                            break;
                        }
                    }

                    if !checked {
                        moves.push(Board::convert_coord_pos(&(rank, 4 + 1)));
                    }
                }
            }
        }

        moves
    }

    // === CHECK AND CHECKMATE ===

    pub fn checking_king(&self, king_piece: &(i8, i8), this_piece_coord : &(i8, i8), precalculated_moves: Option<&Vec<String>>, board: &Board) -> bool {
        if self.0 == 0 { return false; }

        // Get piece type bits
        match self.0 & 14 {
            2 => self.pawn_checking(king_piece, this_piece_coord, precalculated_moves, board),
            4 => self.knight_checking(king_piece, this_piece_coord, precalculated_moves, board),
            6 => self.bishop_checking(king_piece, this_piece_coord, precalculated_moves, board),
            8 => self.rook_checking(king_piece, this_piece_coord, precalculated_moves, board),
            10 => self.queen_checking(king_piece, this_piece_coord, precalculated_moves, board),
            _ => false
        }
    }

    fn pawn_checking(&self, king_piece: &(i8, i8), this_piece_coord : &(i8, i8), precalculated_moves: Option<&Vec<String>>, board: &Board) -> bool {
        // If not within 2 square radius or if king is behind pawn, return false
        if king_piece.0 - this_piece_coord.0 > 1 || this_piece_coord.0 - king_piece.0 > 1 ||
           king_piece.1 - this_piece_coord.1 > 1 || this_piece_coord.1 - king_piece.1 > 1 {return false;}

        if precalculated_moves.is_none() {
            let moves = self.get_pawn_moves(this_piece_coord, board);
            println!("{:?}", moves);

            return Piece::check_for_moves(king_piece, &moves);
        }
        
        Piece::check_for_moves(king_piece, precalculated_moves.unwrap())
    }

    fn knight_checking(&self, king_piece: &(i8, i8), this_piece_coord : &(i8, i8), precalculated_moves: Option<&Vec<String>>, board: &Board) -> bool {
        // If king outside radius; 2 squares on each axis, return false
        if king_piece.0 - this_piece_coord.0 > 2 || this_piece_coord.0 - king_piece.0 > 2 ||
           king_piece.1 - this_piece_coord.1 > 2 || this_piece_coord.1 - king_piece.1 > 2 {return false;}

        if precalculated_moves.is_none() {
            let moves = self.get_knight_moves(this_piece_coord, board);
            return Piece::check_for_moves(king_piece, &moves);
        }

        Piece::check_for_moves(king_piece, precalculated_moves.unwrap())
    }

    fn bishop_checking(&self, king_piece: &(i8, i8), this_piece_coord : &(i8, i8), precalculated_moves: Option<&Vec<String>>, board: &Board) -> bool {
        // Send bishop coords to origin (0,0) remove the difference from the king's coordinates
        // If the bishop and king are on the same diagonal line then the difference of the coordinates is either: 0, 2x or -2x where x is the x-coordinate (y-coord would work fine as well)
        // Since the lines become the like the linear functions y=x and y=-x
        // If they do not share a line then return false
        let origined_coord_difference = king_piece.0 - this_piece_coord.0 - king_piece.1 + this_piece_coord.1;
        if !(origined_coord_difference == 0 || origined_coord_difference == 2 * this_piece_coord.0 || origined_coord_difference == -2 * origined_coord_difference)
        {return false;}
        
        if precalculated_moves.is_none() {
            let moves = self.get_bishop_moves(this_piece_coord, board);
            return Piece::check_for_moves(king_piece, &moves);
        }

        Piece::check_for_moves(king_piece, precalculated_moves.unwrap())
    }

    fn rook_checking(&self, king_piece: &(i8, i8), this_piece_coord : &(i8, i8), precalculated_moves: Option<&Vec<String>>, board: &Board) -> bool {
        // If king and rook don't share a file and rank, return false
        if king_piece.0 != this_piece_coord.0 && king_piece.1 != this_piece_coord.1 {return false;}
        
        if precalculated_moves.is_none() {
            let moves = self.get_rook_moves(this_piece_coord, board);
            return Piece::check_for_moves(king_piece, &moves);
        }

        Piece::check_for_moves(king_piece, precalculated_moves.unwrap())
    }

    fn queen_checking(&self, king_piece: &(i8, i8), this_piece_coord : &(i8, i8), precalculated_moves: Option<&Vec<String>>, board: &Board) -> bool {
        // Queen checks = Bishop checks OR Rook checks
        self.rook_checking(king_piece, this_piece_coord, precalculated_moves, board) ||
        self.bishop_checking(king_piece, this_piece_coord, precalculated_moves, board)
    }

    fn check_for_moves(king_piece: &(i8, i8), moves: &Vec<String>) -> bool {
        let str_coord = Board::convert_coord_pos(king_piece);
        for _move in moves {
            if *_move == str_coord {return true;}
        }
        false
    }

    fn add_move_if_legal(&self, moves_vec: &mut Vec<String>, from: &(i8, i8), to: &(i8, i8), board: Board) {
        let mut new_board = board;
        new_board.make_move(Board::convert_coord_pos(from),Board::convert_coord_pos(to));
        let king = new_board.get_king(&self.get_colour());
        let colour = self.get_colour();

        for row in 0..8 {
            for square in 0..8 {
                let current_piece = new_board.piece_at(&(row, square));
                if !current_piece.is_empty() && current_piece.get_colour() != colour && current_piece.checking_king(&king, &(row, square), None, &new_board) {
                    return;
                }
            }
        }

        moves_vec.push(Board::convert_coord_pos(to));
    }
}