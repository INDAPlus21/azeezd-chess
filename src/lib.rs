pub mod chess;

#[cfg(test)]
mod tests {
    use super::chess;

    #[test]
    fn test_print_all() {
        let a = chess::board::Board::new();
        a.print_all();
    }

    #[test]
    fn test_print_pos() {
        let a = chess::board::Board::new();
        let r = std::io::stdin();
        loop {
            let mut input = String::new();
            r.read_line(&mut input).expect("Error");

            println!("{:?}", a.print_at(&input));
        }
    }

    #[test]
    fn get_moves() {
        let a = chess::board::Board::new();
        let moves = a.get_moves(&String::from("b8"));

        println!("{:?}", moves);
    }
}