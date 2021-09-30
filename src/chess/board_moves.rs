use super::board::*;
use super::piece::*;
use super::piece_data::*;

impl Board {
    pub fn get_pseudo_legal_moves(&self, _coordinates: (i8, i8)) -> Vec<(i8, i8)> {
        // Get piece
        let piece = self.piece_at(_coordinates);

        // Get colour of piece
        let colour = piece.get_colour();

        // Checks if move bit is on
        let first_move = piece.as_u8() & 0x80 == 0x80;

        // Store the movements
        let mut moves: Vec<(i8, i8)> = Vec::with_capacity(10);

        match piece.get_type() {
            PieceType::Pawn => {moves = self.get_pawn_moves(_coordinates, colour, first_move, self).unwrap();},
            PieceType::Knight => {moves = self.get_knight_moves(_coordinates, colour, self).unwrap();}
            PieceType::Bishop => {moves = self.get_bishop_moves(_coordinates, colour, self).unwrap();}
            PieceType::Rook => {moves = self.get_rook_moves(_coordinates, colour, self).unwrap();}
            PieceType::Queen => {moves = self.get_queen_moves(_coordinates, colour, self).unwrap();}
            PieceType::King => {moves= self.get_king_moves(_coordinates, colour, self).unwrap();}
            _ => {}
        }
        
        moves
    }

    pub fn get_legal_moves(&self, _filerank: &String) -> Vec<String> {
        let coordinates = Board::filerank_to_num(_filerank);

        let pseudo = self.get_pseudo_legal_moves(coordinates);
        let mut legal_moves : Vec<String> = Vec::with_capacity(15);
        let piece = self.piece_at(coordinates).get_piece_data();

        for _move in 0..pseudo.len() {
            let mut new_board = Board::copy(self.board);
            new_board.make_pseudo_legal_move(coordinates, pseudo[_move]);
            if !Board::king_in_check(&mut new_board, piece.0) {
                legal_moves.push(Board::num_to_filerank(&pseudo[_move]));
            }
        }

        legal_moves
    }

    fn get_pawn_moves(&self, _coordinates: (i8, i8), _colour: Colour, _first_move: bool, _board: &Board) -> Option<Vec<(i8, i8)>> {
        // Direction of the movement of the pawn
        let move_direction = if _colour == Colour::White {-1} else {1};

        // Store moves
        let mut moves: Vec<(i8, i8)> = Vec::with_capacity(2);

        // Get standard pawn move
        let mut checked_coord = (_coordinates.0, _coordinates.1 + move_direction);
        if _board.is_empty(checked_coord) {moves.push(checked_coord)}

        // Get double step
        checked_coord = (_coordinates.0, _coordinates.1 + move_direction * 2);
        if _first_move && moves.len() == 1 && _board.is_empty(checked_coord) {moves.push(checked_coord);}

        // Get attacks
        for direction in DIRECTIONS {
            checked_coord = (_coordinates.0 + direction, _coordinates.1 + move_direction);
            if Board::within_bounds(checked_coord) {
                if !_board.is_empty(checked_coord)
                && _board.piece_at(checked_coord).get_colour() != _colour {moves.push(checked_coord);}
            }
        }

        // Get en passants
        if _coordinates.0 == 3 || _coordinates.0 == 4 {
            for direction in DIRECTIONS {
                let checked_coord = (_coordinates.0 + direction, _coordinates.1 + move_direction);
                let en_passanting_piece = (_coordinates.0 + direction, _coordinates.1);

                if Board::within_bounds(en_passanting_piece) 
                && Board::within_bounds(checked_coord)
                && _board.piece_at(en_passanting_piece).as_u8() & 0x20 == 0x20 // Get en passant bit
                && _board.is_empty(checked_coord)
                {moves.push(checked_coord);}
            }
        }

        Some(moves)
    }

    fn get_knight_moves(&self, _coordinates: (i8, i8), _colour: Colour, board: &Board) -> Option<Vec<(i8, i8)>> {
        // Store moves
        let mut moves: Vec<(i8, i8)> = Vec::with_capacity(4);

        for direction_x in DIRECTIONS { // front and back
            for direction_y in DIRECTIONS { // left and right
                for l_long_side in 0..2 { // two directions of the L shape
                    let checked_coord = (_coordinates.0 + direction_x * (1 + l_long_side), 
                                        _coordinates.1 + direction_y * (2 - l_long_side));

                    if Board::within_bounds(checked_coord)
                    && (board.is_empty(checked_coord)
                    || board.piece_at(checked_coord).get_colour() != _colour) 
                    {moves.push(checked_coord);}
                }
            }
        }

        Some(moves)
    }

    fn get_bishop_moves(&self, _coordinates: (i8, i8), _colour: Colour, _board: &Board) -> Option<Vec<(i8, i8)>> {
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
                            if _board.piece_at(checked_coord).get_colour() != _colour { // If opposite colour, add to legal moves (i.e can attack opponent)
                                moves.push(checked_coord);
                            }
                            break; // When reaching the non-empty square, break and go to next diagonal direction (if any left)
                        }
                    }
                    else {break;} // Break if outside bounds
                }
            }
        }

        Some(moves)
    }

    fn get_rook_moves(&self, _coordinates: (i8, i8), _colour: Colour, _board: &Board) -> Option<Vec<(i8, i8)>> {
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
                            if _board.piece_at(checked_coord).get_colour() != _colour { // If opposite colour, add to legal moves (i.e can attack opponent)
                                moves.push(checked_coord);
                            }
                            break; // When reaching the non-empty square, break and go to next diagonal direction (if any left)
                        }
                    }
                    else {break;} // out of bounds break and move on to next direction if any exists
                }
            }
        }

        Some(moves)
    }

    fn get_queen_moves(&self, _coordinates: (i8, i8), _colour: Colour, _board: &Board) -> Option<Vec<(i8, i8)>> {
        // Store moves
        let mut moves : Vec<(i8, i8)> = Vec::with_capacity(16);

        // Queen = Bishop + Rook
        moves.append(&mut self.get_bishop_moves(_coordinates, _colour, _board).unwrap());
        moves.append(&mut self.get_rook_moves(_coordinates, _colour, _board).unwrap());

        Some(moves)
    }

    fn get_king_moves(&self, _coordinates: (i8, i8), _colour: Colour, _board: &Board) -> Option<Vec<(i8, i8)>> {
        // Store moves
        let mut moves : Vec<(i8, i8)> = Vec::with_capacity(4);

        for row in -1..2 {
            for col in -1..2 {
                let checked_coord = (_coordinates.0 + row, _coordinates.1 + col);
                if Board::within_bounds(checked_coord) {
                    if (!_board.is_empty(checked_coord) &&
                    _board.piece_at(checked_coord).get_colour() != _colour) ||
                    _board.is_empty(checked_coord)
                    {
                        moves.push(checked_coord);
                    }
                }
            }
        }

        Some(moves)
    }

    pub fn make_pseudo_legal_move(&mut self, _from: (i8, i8), _to: (i8, i8)) {
        self.board[_to.1 as usize][_to.0 as usize] = self.board[_from.1 as usize][_from.0 as usize];
        self.board[_from.1 as usize][_from.0 as usize] = Piece::from_u8(0x0);
    }
}