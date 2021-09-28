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

pub struct Game {
    /* save board, active colour, ... */
    board: Board,
    active_colour: Colour,
    state: GameState
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {

        Game {
            /* initialise board, set active colour to white, ... */
            board: Board::new(),
            active_colour: Colour::White,
            state: GameState::InProgress,
        }
    }

    pub fn new_empty() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            board: Board::new_empty(),
            active_colour: Colour::White,
            state: GameState::InProgress,
        }
    }

    /// If the current game state is InProgress and the move is legal, 
    /// move a piece and return the resulting state of the game.
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
            if self.board.piece_at(&new_pos)
                .checking_king(&self.board.get_king(&self.active_colour), &new_pos, None, &self.board) {
                    return Some(GameState::Check);
            }
        }
        else {
            panic!()
        }

        Some(GameState::InProgress)
    }

    /// Set the piece type that a peasant becames following a promotion.
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

    pub fn then(&mut self, from: &str, to: &str) -> &mut Game {
        self.state = self.make_move(String::from(from), String::from(to)).unwrap();
        println!("{:?}", self);
        println!("{:?}", self.state);
        self
    }

    pub fn and_promote(&mut self, _at: &str, _piece: &str) -> &mut Game {
        self.set_promotion(String::from(_at), String::from(_piece));
        println!("{:?}", self);
        println!("{:?}", self.state);
        self
    }
    
    pub fn and_add_at(&mut self, _at: &str, _colour: Colour, _piece_type: PieceType) -> &mut Game {
        self.board.set_piece(String::from(_at), _colour, _piece_type);

        self
    }

    pub fn and_remove_at(&mut self, _at: &str) -> &mut Game {
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