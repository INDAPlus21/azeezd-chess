pub mod chess;
use chess::piece_data::Colour;
use chess::piece_data::PieceType;
use chess::board::Board;
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

/// ## Game
/// ### Type: `struct`
/// Holds data neccessary for the game and handles the actual gameplay of chess.
/// ### Members:
/// - `board`: Representation of te board
/// - `Colour`: The current player's colour
/// - `GameState`: The state of the game, `InProgress`, `Check` or `GameOver`
pub struct Game {
    /* save board, active colour, ... */
    board: Board,
    active_colour: Colour,
    state: GameState
}

impl Game {
    /// ## `new`
    /// Returns a new instance of the `Game` struct with the standard default values for chess.
    /// - Standard starting board
    /// - Active Colour: White
    /// - Game State: `InProgress`
    pub fn new() -> Game {

        Game {
            /* initialise board, set active colour to white, ... */
            board: Board::new(),
            active_colour: Colour::White,
            state: GameState::InProgress,
        }
    }

    /// ## DEBUG METHOD: `new_empty`
    /// Creates an empty board with two kings each at their starting square, e1 and e8.
    /// - Empty Board with only two kings
    /// - Active Colour: White
    /// - Game State: `InProgress`
    pub fn new_empty() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            board: Board::new_empty(),
            active_colour: Colour::White,
            state: GameState::InProgress,
        }
    }

    /// ## `make_move`
    /// Takes squares and changes the position of the piece at the first to the second, checking for legality. If the move is illegal it panics.
    /// ### Parameters
    /// - `_from: String`: The square the piece to be moved is at given as "\<File\>\<Rank\>"
    /// - `_to: String`: The square the piece to be moved will be given as "\<File\>\<Rank\>"
    /// 
    /// ### Returns
    /// Returns `Option<>` wrapping a `GameState`
    /// `
    /// Option<GameState>
    /// `
    /// with the current state of the game.
    /// 
    /// ### Panics!
    /// - If a move is illegal:
    /// `
    /// panic!("Illegal Move!")
    /// `
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        let moves = self.board.get_moves(&_from);
        let mut is_legal_move = false;
        let new_pos = Board::convert_str_pos(&_to);

        for _move in moves {
            if _move == _to {
                is_legal_move = true;
                break;
            }
        }

        // If the move is legal then proceed normally else panic
        if is_legal_move {
            self.board.make_move(_from, _to);
            self.active_colour = if self.active_colour == Colour::White {Colour::Black} else {Colour::White};

            // Check if this moves puts the enemy at check and return GameState::Check if so.
            if self.board.piece_at(&new_pos).checking_king(&self.board.get_king(&self.active_colour), &new_pos, None, &self.board) {
                    return Some(GameState::Check);
            }

            if self.board.piece_at(&new_pos).get_type() == PieceType::King {
                return Some(GameState::GameOver);
            }
        }
        else {
            panic!("Illegal Move!")
        }

        Some(GameState::InProgress)
    }

    /// ## `set_promotion`
    /// Takes a square position and a piece type name and promotes that piece at the square to the given piece type.
    /// ### Parameters
    /// - `_square: String`: The position of the piece given as "\<File\>\<Rank\>"
    /// - `_piece: String`: The type of the piece to promote to. Read below for accepted input
    /// 
    /// ### `_piece` formatting
    /// - `"queen"`: promotes to a queen
    /// - `"knight"`: promotes to a knight
    /// - `"rook"`: promotes to a rook
    /// - `"bishop"`: promotes to a bishop
    pub fn set_promotion(&mut self, _square: String, _piece: String) {
        let piece = self.board.piece_at(&Board::convert_str_pos(&_square));
        let colour = piece.get_colour();


        if _piece.eq_ignore_ascii_case("queen") {
            self.board.change_piece_type(_square, PieceType::Queen);
        }
        else if _piece.eq_ignore_ascii_case("knight") {
            self.board.change_piece_type(_square, PieceType::Knight);
        }
        else if _piece.eq_ignore_ascii_case("rook") {
            self.board.change_piece_type(_square, PieceType::Rook);
        }
        else if _piece.eq_ignore_ascii_case("bishop") {
            self.board.change_piece_type(_square, PieceType::Bishop);
        }

        if self.board.king_in_check(if colour == Colour::White {&Colour::Black} else {&Colour::White}) {
            self.state = GameState::Check;
        }
    }

    /// ## get_game_state
    /// Returns the current state of the game
    /// ### Return
    /// - `GamneState::InProgress`: Game is still on!
    /// - `GamneState::Check`: Some king is in check!
    /// - `GamneState::GameOver`: A king is dead!
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    
    /// ## `get_possible_moves`
    /// Takes a square position and returns all possible legal moves of the piece at that square
    /// ### Parameters
    /// - `_position: String`: The position of the piece given as "\<File\>\<Rank\>".
    /// ### Return
    /// Returns an `Option<>` wrapping a `Vec<String>`
    /// `
    /// Option<Vec<String>>
    /// `
    /// Holding all legal possible moves of the given square
    pub fn get_possible_moves(&self, _position: String) -> Option<Vec<String>> {
        Some(self.board.get_moves(&_position))
    }

    /// ## DEBUG METHOD: `_then`
    /// Takes two string literals to make a move and return the Game. This method is used for method chaining and debugging in unit tests
    /// ### Parameters:
    /// - `_from: &str`: The square which the moving piece is at.
    /// - `_to: &str`: The square to which the moving piece will move.
    /// 
    /// ### Return
    /// Returns a mutable reference to current game
    /// `
    /// &mut Game
    /// `
    /// after the move
    fn _then(&mut self, _from: &str, _to: &str) -> &mut Game {
        self.state = self.make_move(String::from(_from), String::from(_to)).unwrap();
        println!("{:?}", self);
        println!("{:?}", self.state);
        self
    }

    /// ## DEBUG METHOD: `_and_promote`
    /// Takes a string literal for square position and another for piece type to promote a piece at a given square. This method is used for method chaining and debugging in unit tests
    /// ### Parameters:
    /// - `_from: &str`: The square which the promoted piece is at.
    /// - `_piece: &str`: The type of the piece to promote to. Read below for formatting
    /// 
    /// ### Return
    /// Returns a mutable reference to current game
    /// `
    /// &mut Game
    /// `
    /// after the promotion
    /// 
    /// ### `_piece` formatting
    /// - `"queen"`: promotes to a queen
    /// - `"knight"`: promotes to a knight
    /// - `"rook"`: promotes to a rook
    /// - `"bishop"`: promotes to a bishop
    fn _and_promote(&mut self, _at: &str, _piece: &str) -> &mut Game {
        self.set_promotion(String::from(_at), String::from(_piece));
        println!("{:?}", self);
        println!("{:?}", self.state);
        self
    }
    
    /// ## DEBUG METHOD: `_and_add_at`
    /// Takes a string literal for square position and a `Colour` and `PieceType` enums to add a piece (from outside the game) at the given square. This method is used for method chaining and debugging in unit tests
    /// ### Parameters:
    /// - `_at: &str`: The square which the added piece will be at.
    /// - `_colour: Colour`: The colour of the piece that will be added
    /// - `_piece_type: PieceType`: The type of the piece that will be added
    /// 
    /// ### Return
    /// Returns a mutable reference to current game
    /// `
    /// &mut Game
    /// `
    /// after the addition
    fn _and_add_at(&mut self, _at: &str, _colour: Colour, _piece_type: PieceType) -> &mut Game {
        self.board.set_piece(String::from(_at), _colour, _piece_type);

        self
    }

    /// ## DEBUG METHOD: `_and_remove_at`
    /// Takes a string literal for square position removes any piece that is there. This method is used for method chaining and debugging in unit tests
    /// ### Parameters:
    /// - `_at: &str`: The square which the removed piece is at
    /// 
    /// ### Return
    /// Returns a mutable reference to current game
    /// `
    /// &mut Game
    /// `
    /// after the removal
    fn _and_remove_at(&mut self, _at: &str) -> &mut Game {
        self.board.set_piece(String::from(_at), Colour::White, PieceType::None);

        self
    }
}

/// Implement print routine for Game.
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

#[cfg(test)]
mod unit_tests;