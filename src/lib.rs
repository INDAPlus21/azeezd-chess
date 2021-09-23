pub mod chess;
use chess::*;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

pub struct Game {
    /* save board, active colour, ... */
    board: board::Board,
    active_colour: piece_data::Colour,
    state: GameState
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            board: board::Board::new(),
            active_colour: piece_data::Colour::White,
            state: GameState::InProgress
        }
    }

    /// If the current game state is InProgress and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        None // To be added
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _piece: String) -> () {
        () // to be added
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _position: String) -> Option<Vec<String>> {
        Some(self.board.get_moves(&_position))
    }
}

/// Implement print routine for Game.
/// 
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_str = String::with_capacity(800);

        board_str.push_str("|:------------------------:|\n");

        for col in 0..8 {
            board_str.push_str("|  ");
            for row in 0..8 {
                board_str.push(self.board.piece_at(&(row, col)).get_icon());
                board_str.push_str("  ");
            }
            board_str.push_str("|\n");
        }

        board_str.push_str("|:------------------------:|\n");

        write!(f, "{}", board_str)
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {

        let game = Game::new();

        game.board.print_all();

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    #[test]
    fn print_board() {
        let game = Game::new();

        println!("{:?}", game);
    }
}