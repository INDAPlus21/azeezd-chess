use super::piece::*;

/// Macro used to create a piece using u8.
/// Mainly used to save space when creating the main default board in Board::new method
macro_rules! n_p {
    ($x:expr) => (Piece::from_u8($x));
}

/// Represents a 8x8 board as a 8x8 array where every element is a Piece struct
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
        self.0[coords.0 as usize][coords.1 as usize].get_str()
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
    fn convert_str_pos(position: &String) -> (u8, u8) {
        let mut coords = (0,0);
        
        let position = position.as_bytes();

        // The rank represnts the y-axis thus it is the 1st coordinate
        // And the file the x-axes hence the 0th coordinate
        coords.1 = position[0] - 97; // Lowercase alphabet to u8 using ascii value different between letter and numerical value that is 1-indexed

        /* Convert number as ascii char to actual numerical value by doing minus 49 (ascii difference) but the the board's origin is at bottom left
           So the x coordinate must shift by 7 - (top left origin coord) thus
           7 - ([ascii val] - 49) gives 56 - [ascii val]
        */
        coords.0 = 56 - position[1]; 

        coords
    }
}