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

    println!("Possible moves for d2: {:?}", game.get_possible_moves(String::from("d2")).unwrap());
}