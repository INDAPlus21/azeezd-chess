use super::board::*;
use super::piece_data::*;

impl Board {
    /// Returns true if the coordinates are within the board
    pub fn within_bounds(_coordinates: (i8, i8)) -> bool {
        _coordinates.0 >= 0 && _coordinates.0 <= 7 &&
        _coordinates.1 >= 0 && _coordinates.1 <= 7
    }

    /// Converts a (i8, i8) to String in the form "\<file\>\<rank\>"
    pub fn num_to_filerank(_coords: &(i8, i8)) -> String {
        let mut string_coords = String::with_capacity(2);

        string_coords.push((_coords.0 as u8 + 97) as char);
        string_coords.push((56 - _coords.1 as u8) as char);

        string_coords
    }

    /// Converts a String from "\<file\>\<rank\>" to a coord in the form of (i8, i8)
    pub fn filerank_to_num(_filerank: &String) -> (i8, i8) {
        let mut coords = (0,0);
        
        let _filerank = _filerank.as_bytes();

        // The rank represnts the y-axis thus it is the 1st coordinate
        // And the file the x-axes hence the 0th coordinate
        coords.0 = (_filerank[0] - 97) as i8; // Lowercase alphabet to u8 using ascii value different between letter and numerical value that is 1-indexed

        /* Convert number as ascii char to actual numerical value by doing minus 49 (ascii difference) but the the board's origin is at bottom left
           So the x coordinate must shift by 7 - (top left origin coord) thus
           7 - ([ascii val] - 49) gives 56 - [ascii val]
        */
        coords.1 = (56 - _filerank[1]) as i8; 

        coords
    }

    /// Returns true if the king is in check. By seeing if the king is in the threat map
    pub fn king_in_check(_board: &mut Board, _colour: Colour) -> bool {
        _board.generate_threat_maps();
        let king = _board.get_king(_colour);
        
        if _colour == Colour::Black {
            return _board.white_threat_map.contains(&king);
        }
        else {
            return _board.black_threat_map.contains(&king);
        }
    }
}