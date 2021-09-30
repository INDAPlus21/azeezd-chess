use super::piece::*;
use super::piece_data::*;

/// Macro used to create a piece using u8.
/// Mainly used to save space when creating the main default board in Board::new method
macro_rules! n_p {
    ($x:expr) => (Piece::from_u8($x));
}

pub struct Board{
    pub board: [[Piece; 8]; 8],
    pub white_threat_map: Vec<(i8, i8)>,
    pub black_threat_map: Vec<(i8, i8)>
}

impl Board {
    // Generate a standard chess board with pieces at their starting positions
    pub fn new() -> Board {
        Board {
            board: [
            [n_p!(0x89), n_p!(0x05), n_p!(0x07), n_p!(0x8B), n_p!(0x8D), n_p!(0x07), n_p!(0x05), n_p!(0x89)],
            [n_p!(0x83), n_p!(0x83), n_p!(0x83), n_p!(0x83), n_p!(0x83), n_p!(0x83), n_p!(0x83), n_p!(0x83)],
            [n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00)],
            [n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00)],
            [n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00)],
            [n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00)],
            [n_p!(0x82), n_p!(0x82), n_p!(0x82), n_p!(0x82), n_p!(0x82), n_p!(0x82), n_p!(0x82), n_p!(0x82)],
            [n_p!(0x88), n_p!(0x04), n_p!(0x06), n_p!(0x8A), n_p!(0x8C), n_p!(0x06), n_p!(0x04), n_p!(0x88)]],
            white_threat_map: vec![],
            black_threat_map: vec![]
        }
    }

    // Generate an empty board with two kings at their standard starting positions
    pub fn new_empty() -> Board {
        Board {
            board:[
            [n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x8D), n_p!(0x00), n_p!(0x00), n_p!(0x00)],
            [n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00)],
            [n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00)],
            [n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00)],
            [n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00)],
            [n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00)],
            [n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00)],
            [n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x8C), n_p!(0x00), n_p!(0x00), n_p!(0x00)]],
            white_threat_map: vec![],
            black_threat_map: vec![]
        }
    }

    pub fn copy(board: [[Piece; 8]; 8]) -> Board {
        Board {
            board: board,
            white_threat_map: vec![],
            black_threat_map: vec![]
        }
    }

    /// Returns a reference to the piece at a given coordinate position
    pub fn piece_at(&self, _coordinates: (i8, i8)) -> &Piece {
        &self.board[_coordinates.1 as usize][_coordinates.0 as usize]
    }

    pub fn mut_piece_at(&mut self, _coordinates: (i8, i8)) -> &mut Piece {
        &mut self.board[_coordinates.1 as usize][_coordinates.0 as usize]
    }

    // Returns true if the square given is PieceType::None
    pub fn is_empty(&self, _coordinate: (i8, i8)) -> bool {
        self.piece_at(_coordinate).get_type() == PieceType::None
    }

    /// Returns the numerical coordinates of a king of given colour
    pub fn get_king(&self, _colour: Colour) -> (i8, i8) {
        for row in 0..8 {
            for col in 0..8 {
                if self.piece_at((row, col)).get_piece_data() == (_colour, PieceType::King) {
                    return (row, col);
                }
            }
        }

        panic!("No king!")
    }

    // Make move
    pub fn make_move(&mut self, _from: &String, _to: &String) {
        let moves = self.get_legal_moves(&_from);

        if moves.contains(&_to) {
            self.make_pseudo_legal_move(Board::filerank_to_num(&_from), Board::filerank_to_num(&_to));
        }
        else {
            panic!("Illegal Move!")
        }
    }
}

