use super::piece::*;
use super::piece_data::*;

/// Macro used to create a piece using u8.
/// Mainly used to save space when creating the main default board in Board::new method
macro_rules! n_p {
    ($x:expr) => (Piece::from_u8($x));
}

/// Represents a 8x8 board as a 8x8 array where every element is a Piece struct
#[derive(Copy, Clone)]
pub struct Board([[Piece; 8]; 8]);

impl Board {

    /// Create a brand new fresh default board
    pub fn new() -> Board {
        Board([
            [n_p!(0x89), n_p!(0x05), n_p!(0x07), n_p!(0x8B), n_p!(0x8D), n_p!(0x07), n_p!(0x05), n_p!(0x89)],
            [n_p!(0x83), n_p!(0x83), n_p!(0x83), n_p!(0x83), n_p!(0x83), n_p!(0x83), n_p!(0x83), n_p!(0x83)],
            [n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00)],
            [n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00)],
            [n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00)],
            [n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00), n_p!(0x00)],
            [n_p!(0x82), n_p!(0x82), n_p!(0x82), n_p!(0x82), n_p!(0x82), n_p!(0x82), n_p!(0x82), n_p!(0x82)],
            [n_p!(0x88), n_p!(0x04), n_p!(0x06), n_p!(0x8A), n_p!(0x8C), n_p!(0x06), n_p!(0x04), n_p!(0x88)]
        ])
    }

    /// Print the Piece as a string in format "<Colour> <Piece Type>" at the given position given in "<Rank><File>".
    pub fn print_at(&self, position: &String) -> String {
        let coords = Board::convert_str_pos(position);
        self.0[coords.1 as usize][coords.0 as usize].get_str()
    }

    /// Print the entire board in a 8x8 structure where every piece is represented as "<Colour> <Piece Type>"
    pub fn print_all(&self) {
        for piece in &self.0 {
            for elem in piece {
                print!("{} ", elem.get_str());
            }
            println!();
        }
    }

    /// Converts the "<Rank><File>" format into array indices to work with the array board
    pub fn convert_str_pos(position: &String) -> (i8, i8) {
        let mut coords = (0,0);
        
        let position = position.as_bytes();

        // The rank represnts the y-axis thus it is the 1st coordinate
        // And the file the x-axes hence the 0th coordinate
        coords.0 = (position[0] - 97) as i8; // Lowercase alphabet to u8 using ascii value different between letter and numerical value that is 1-indexed

        /* Convert number as ascii char to actual numerical value by doing minus 49 (ascii difference) but the the board's origin is at bottom left
           So the x coordinate must shift by 7 - (top left origin coord) thus
           7 - ([ascii val] - 49) gives 56 - [ascii val]
        */
        coords.1 = (56 - position[1]) as i8; 

        coords
    }

    /// Converts numerical coordinates to string "<File><Rank>"" format
    pub fn convert_coord_pos(coords: &(i8, i8)) -> String {
        let mut string_coords = String::with_capacity(2);

        string_coords.push((coords.0 as u8 + 97) as char);
        string_coords.push((56 - coords.1 as u8) as char);

        string_coords
    }

    /// Returns a reference to the piece at a given coordinate
    pub fn piece_at(&self, coords: &(i8, i8)) -> Piece {
        self.0[coords.1 as usize][coords.0 as usize]
    }

    /// Get legal moves of the piece
    pub fn get_moves(&self, position: &String) -> Vec<String> {
        let coords = Board::convert_str_pos(position);
        self.piece_at(&coords).get_moves(&coords, &self)
    }

    /// Coordinates within the board's coordinates
    pub fn within_bounds(coords: &(i8, i8)) -> bool {
        coords.0 <= 7 && coords.0 >= 0 &&
        coords.1 <= 7 && coords.1 >= 0 
    }

    /// Returns the coords of the king of a given colour
    pub fn get_king(&self, colour: &Colour) -> (i8, i8) {
        for row in 0..8 {
            for col in 0..8 {
                let coord : (i8, i8) = (row, col);
                let piece = self.piece_at(&coord);
                if piece.get_type() == PieceType::King && piece.get_colour() == *colour {
                    return coord;
                }
            }
        }

        panic!()
    }

    /// Make a move that is legal
    pub fn make_move(&mut self, from: String, to: String) {
        let from = Board::convert_str_pos(&from);
        let to = Board::convert_str_pos(&to);
        let mut moved_piece = self.piece_at(&from);

        // Set the en passant-able bitflag of all pawns who didn't get captured by en passant to 0
        for row in 0..8 {
            for col in 0..8 {
                if self.0[row][col].0 & 0x20 == 0x20 {
                    self.0[row][col].0 = self.0[row][col].0 & 0xdf;
                }
            }
        }

        // Set move bitflag to 0 by using 01111111 mask
        moved_piece.0 = moved_piece.0 & 0x7f;

        // If piece is a pawn and did a double-step then set the en passant flag on using the 00100000 mask
        if moved_piece.get_type() == PieceType::Pawn && to.1 - from.1 == 2 {
            moved_piece.0 = moved_piece.0 | 0x20;
        }

        self.0[to.1 as usize][to.0 as usize] = moved_piece;
        self.0[from.1 as usize][from.0 as usize] = Piece::from_u8(0);
    }

    /// Change the type of the piece, used for promotion
    pub fn change_piece_type(&mut self, at: String, piece_type: PieceType) {
        self.piece_at(&Board::convert_str_pos(&at)).set_type(piece_type);
    }
}