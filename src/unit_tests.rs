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

    game.state = game.make_move(String::from("e2"), String::from("e4")).unwrap();
    println!("{:?}\n{:?}", game, game.state);

    game.state = game.make_move(String::from("e7"), String::from("e5")).unwrap();
    println!("{:?}\n{:?}", game, game.state);

    game.state = game.make_move(String::from("f1"), String::from("c4")).unwrap();
    println!("{:?}\n{:?}", game, game.state);

    game.state = game.make_move(String::from("b8"), String::from("c6")).unwrap();
    println!("{:?}\n{:?}", game, game.state);

    game.state = game.make_move(String::from("d1"), String::from("h5")).unwrap();
    println!("{:?}\n{:?}", game, game.state);

    game.state = game.make_move(String::from("g8"), String::from("f6")).unwrap();
    println!("{:?}\n{:?}", game, game.state);

    game.state = game.make_move(String::from("h5"), String::from("f7")).unwrap();
    println!("{:?}\n{:?}", game, game.state);
}

#[test]
fn en_passanting() {
    let mut game = Game::new();

    game.state = game.make_move(String::from("e2"), String::from("e4")).unwrap();
    println!("{:?}", game);
    game.state = game.make_move(String::from("e7"), String::from("e6")).unwrap();
    println!("{:?}", game);
    game.state = game.make_move(String::from("e4"), String::from("e5")).unwrap();
    println!("{:?}", game);
    game.state = game.make_move(String::from("d7"), String::from("d5")).unwrap();
    println!("{:?}", game);
    game.state = game.make_move(String::from("e5"), String::from("d6")).unwrap();
    println!("{:?}", game);
}