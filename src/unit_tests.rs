// --------------------------
// ######### TESTS ##########
// --------------------------

use super::*;

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

    println!("{:?}", game);

    assert_eq!(game.get_game_state(), GameState::InProgress);
}

#[test]
fn pawn_moves() {
    let mut game = Game::new();

    assert_eq!(game.get_possible_moves(String::from("e2")).unwrap(), vec!["e3", "e4"]);
    assert_eq!(game.get_possible_moves(String::from("e7")).unwrap(), vec!["e6", "e5"]);

    game.then("e2", "e4")
        .then("e7", "e6");

    assert_eq!(game.get_possible_moves(String::from("e4")).unwrap(), vec!["e5"]);
    assert_eq!(game.get_possible_moves(String::from("e6")).unwrap(), vec!["e5"]);
}

#[test]
fn knight_moves() {
    let mut game = Game::new_empty();

    game.and_add_at("e6", Colour::Black, PieceType::Knight)
        .and_add_at("d4", Colour::White, PieceType::Knight)
        .and_add_at("b4", Colour::White, PieceType::Pawn);

    println!("{:?}", game);

    assert_eq!(game.get_possible_moves(String::from("e6")).unwrap(), vec!["d8", "c7", "d4", "c5", "f8", "g7", "f4", "g5"]);
    assert_eq!(game.get_possible_moves(String::from("d4")).unwrap(), vec!["c6", "b5", "c2", "b3", "e6", "f5", "e2", "f3"]);

    game.then("b4", "b5");

    assert_eq!(game.get_possible_moves(String::from("d4")).unwrap(), vec!["c6", "c2", "b3", "e6", "f5", "e2", "f3"]);
}

#[test]
fn bishop_moves() {
    let mut game = Game::new_empty();

    game.and_add_at("b7", Colour::Black, PieceType::Bishop)
        .and_add_at("a7", Colour::White, PieceType::Pawn)
        .and_add_at("d6", Colour::Black, PieceType::Pawn);

    println!("{:?}", game);

    assert_eq!(game.get_possible_moves(String::from("b7")).unwrap(), vec!["a8", "c8", "a6", "c6", "d5", "e4", "f3", "g2", "h1"]);

    game.then("a7", "a8")
        .then("d6", "d5");

    assert_eq!(game.get_possible_moves(String::from("b7")).unwrap(), vec!["a8", "c8", "a6", "c6"]);
}

#[test]
fn rook_moves() {
    let mut game = Game::new_empty();

    game.and_add_at("c6", Colour::Black, PieceType::Rook)
        .and_add_at("d5", Colour::White, PieceType::Rook);

    println!("{:?}", game);

    assert_eq!(game.get_possible_moves(String::from("c6")).unwrap(), vec!["b6", "a6", "d6", "e6", "f6", "g6", "h6", "c7", "c8", "c5", "c4", "c3", "c2", "c1"]);
    assert_eq!(game.get_possible_moves(String::from("d5")).unwrap(), vec!["c5", "b5", "a5", "e5", "f5", "g5", "h5", "d6", "d7", "d8", "d4", "d3", "d2", "d1"]);

    game.then("c6", "d6");

    assert_eq!(game.get_possible_moves(String::from("d6")).unwrap(), vec!["c6", "b6", "a6", "e6", "f6", "g6", "h6", "d7", "d8", "d5"]);
    assert_eq!(game.get_possible_moves(String::from("d5")).unwrap(), vec!["c5", "b5", "a5", "e5", "f5", "g5", "h5", "d6", "d4", "d3", "d2", "d1"]);
}

#[test]
fn queen_moves_and_check() {
    let mut game = Game::new_empty();

    game.and_add_at("d4", Colour::White, PieceType::Queen)
        .and_add_at("f5", Colour::Black, PieceType::Queen);

    println!("{:?}", game);

    assert_eq!(game.get_possible_moves(String::from("d4")).unwrap(), vec!["c5", "b6", "a7", "e5", "f6", "g7", "h8", "c3", "b2", "a1", "e3", "f2", "g1", 
                                                                          "c4", "b4", "a4", "e4", "f4", "g4", "h4", "d5", "d6", "d7", "d8", "d3", "d2", "d1"]);

    assert_eq!(game.get_possible_moves(String::from("f5")).unwrap(), vec!["e6", "d7", "c8", "g6", "h7", "e4", "d3", "c2", "b1", "g4", "h3", "e5", "d5",
                                                                          "c5", "b5", "a5", "g5", "h5", "f6", "f7", "f8", "f4", "f3", "f2", "f1"]);

    game.then("f5", "e5");

    assert_eq!(game.get_possible_moves(String::from("d4")).unwrap(), vec!["c6", "b6", "a6", "e6", "f6", "g6", "h6", "d7", "d8", "d5"]);
    assert_eq!(game.get_possible_moves(String::from("e5")).unwrap(), vec!["c5", "b5", "a5", "e5", "f5", "g5", "h5", "d6", "d4", "d3", "d2", "d1"]);
}

#[test]
fn print_board() {
    let game = Game::new();

    println!("{:?}", game);
}

#[test]
fn fools_mate() {
    let mut game = Game::new();

    game.then("e2","e4")
        .then("e7","e5")
        .then("f1","c4")
        .then("b8","c6")
        .then("d1","h5")
        .then("g8","f6")
        .then("h5","f7");

    assert_eq!(game.state, GameState::Check);
}

#[test]
fn en_passanting() {
    let mut game = Game::new();

    game.then("e2", "e4")
        .then("e7", "e6")
        .then("e4", "e5")
        .then("d7", "d5")
        .then("e5", "d6");
}

#[test]
fn move_that_checks_own_king() {
    let mut game = Game::new();

    game.then("e2", "e4")
        .then("e7", "e6")
        .then("e4", "e5")
        .then("f8", "b4");

    assert_eq!(game.get_possible_moves(String::from("d2")).unwrap().len(), 0);
}

#[test]
fn new_empty() {
    let game = Game::new_empty();

    println!("{:?}", game);
}