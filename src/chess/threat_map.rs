use super::board::*;
use super::piece_data::*;
use super::piece::*;
use std::collections::HashSet;

impl Board {

    /// Fills the the threat map vectors with squares that are defended or attacked by a colour
    pub fn generate_threat_maps(&mut self){
        let mut black_map: HashSet<(i8, i8)> = HashSet::with_capacity(40);
        let mut white_map: HashSet<(i8, i8)> = HashSet::with_capacity(40);

        for row in 0..8 {
            for col in 0..8 {
                let piece = self.piece_at((row, col));
                let colour = piece.get_colour();
                let moves = self.get_piece_threat_moves((row, col), *piece, colour).unwrap();

                if colour == Colour::Black {
                    for _move in moves {
                        black_map.insert(_move);
                    }
                }
                else {
                    for _move in moves {
                        white_map.insert(_move);
                    }
                }
            }
        }

        self.black_threat_map = black_map.into_iter().collect();
        self.white_threat_map = white_map.into_iter().collect();
    }

    /// Get all squares that a piece can attack or defend
    fn get_piece_threat_moves(&self, _coordinates: (i8, i8), _piece: Piece, _colour: Colour) -> Option<Vec<(i8, i8)>> {
        // Store the movements
        let mut moves: Vec<(i8, i8)> = Vec::with_capacity(10);

        match _piece.get_type() {
            PieceType::Pawn => {moves = self.get_pawn_threats(_coordinates, _colour, self).unwrap();},
            PieceType::Knight => {moves = self.get_knight_threats(_coordinates, _colour, self).unwrap();}
            PieceType::Bishop => {moves = self.get_bishop_threats(_coordinates, _colour, self).unwrap();}
            PieceType::Rook => {moves = self.get_rook_threats(_coordinates, _colour, self).unwrap();}
            PieceType::Queen => {moves = self.get_queen_threats(_coordinates, _colour, self).unwrap();}
            PieceType::King => {moves = self.get_king_threats(_coordinates, _colour, self).unwrap();}
            _ => {}
        }
        
        Some(moves)
    }

    fn get_pawn_threats(&self, _coordinates: (i8, i8), _colour: Colour, _board: &Board) -> Option<Vec<(i8, i8)>> {
        // Direction of the movement of the pawn
        let move_direction = if _colour == Colour::White {-1} else {1};

        // Store moves
        let mut moves: Vec<(i8, i8)> = Vec::with_capacity(2);

        // Get attacks
        for direction in DIRECTIONS {
            let checked_coord = (_coordinates.0 + direction, _coordinates.1 + move_direction);
            if Board::within_bounds(checked_coord) {
                
                if _board.piece_at(checked_coord).get_colour() != _colour {moves.push(checked_coord);}
            }
        }

        // Get en passants
        for direction in DIRECTIONS {
            let passanted_square = (_coordinates.0 + direction, _coordinates.1 + move_direction );
            let checked_coord = (_coordinates.0 + direction, _coordinates.1);

            if !Board::within_bounds(passanted_square) {continue;}

            let passanting_piece = _board.piece_at(checked_coord);

            if Board::within_bounds(passanted_square)
            && !_board.is_empty(checked_coord)
            && _board.is_empty(passanted_square)
            && passanting_piece.get_colour() != _colour
            && passanting_piece.as_u8() & 0x20 == 0x20 // Get en passant square
            {moves.push(passanted_square);}
        }

        Some(moves)
    }

    fn get_knight_threats(&self, _coordinates: (i8, i8), _colour: Colour, _board: &Board) -> Option<Vec<(i8, i8)>> {
        // Store moves
        let mut moves: Vec<(i8, i8)> = Vec::with_capacity(4);

        for direction_x in DIRECTIONS { // front and back
            for direction_y in DIRECTIONS { // left and right
                for l_long_side in 0..2 { // two directions of the L shape
                    let checked_coord = (_coordinates.0 + direction_x * (1 + l_long_side), 
                                        _coordinates.1 + direction_y * (2 - l_long_side));

                    if Board::within_bounds(checked_coord)
                    {moves.push(checked_coord);}
                }
            }
        }

        Some(moves)
    }

    fn get_bishop_threats(&self, _coordinates: (i8, i8), _colour: Colour, _board: &Board) -> Option<Vec<(i8, i8)>> {
        // store moves
        let mut moves: Vec<(i8, i8)> = Vec::with_capacity(8);

        for direction_y in DIRECTIONS { // up and down
            for direction_x in DIRECTIONS { // left and right
                for square in 1..9 { // loop from min to max amount of moves for a bishop per direction
                    let checked_coord = (_coordinates.0 + direction_x * square, _coordinates.1 + direction_y * square);

                    if Board::within_bounds(checked_coord) {
                        if _board.is_empty(checked_coord) { // If the square being checked is empty add to moves
                            moves.push(checked_coord);
                        }
                        else { // Reached a non-empty square!
                            moves.push(checked_coord);
                            break; // When reaching the non-empty square, break and go to next diagonal direction (if any left)
                        }
                    }
                    else {break;} // Break if outside bounds
                }
            }
        }

        Some(moves)
    }

    fn get_rook_threats(&self, _coordinates: (i8, i8), _colour: Colour, _board: &Board) -> Option<Vec<(i8, i8)>> {
        // store moves
        let mut moves: Vec<(i8, i8)> = Vec::with_capacity(8);

        for axis in DIRECTIONS { // Horizontal -1 or Vertical 1
            for direction in DIRECTIONS { // left & right for horizontal, up & down for vertical
                for square in 1..9 {
                    // Current squared, mutable due to be added to later to determine next square
                    let mut checked_coord = (_coordinates.0, _coordinates.1);

                    // If horizontal then increment (or decrement) in horizontal axis
                    if axis == -1 { checked_coord.0 += square * direction; }
                    else { checked_coord.1 += square * direction; }

                    if Board::within_bounds(checked_coord) {
                        if _board.is_empty(checked_coord) {
                            moves.push(checked_coord);
                        }
                        else { // Reached a non-empty square!
                            moves.push(checked_coord);
                            break; // When reaching the non-empty square, break and go to next diagonal direction (if any left)
                        }
                    }
                    else {break;} // out of bounds break and move on to next direction if any exists
                }
            }
        }

        Some(moves)
    }

    fn get_queen_threats(&self, _coordinates: (i8, i8), _colour: Colour, _board: &Board) -> Option<Vec<(i8, i8)>> {
        // Store moves
        let mut moves : Vec<(i8, i8)> = Vec::with_capacity(16);

        // Queen = Bishop + Rook
        moves.append(&mut self.get_bishop_threats(_coordinates, _colour, _board).unwrap());
        moves.append(&mut self.get_rook_threats(_coordinates, _colour, _board).unwrap());

        Some(moves)
    }

    fn get_king_threats(&self, _coordinates: (i8, i8), _colour: Colour, _board: &Board) -> Option<Vec<(i8, i8)>> {
        // Store moves
        let mut moves : Vec<(i8, i8)> = Vec::with_capacity(4);

        // Check the 3x3 square around the king
        for row in -1..2 {
            for col in -1..2 {
                let checked_coord = (_coordinates.0 + col, _coordinates.1 + row);
                if Board::within_bounds(checked_coord) && (row != 0 || col != 0) {
                    moves.push(checked_coord);
                }
            }
        }

        Some(moves)
    }
}