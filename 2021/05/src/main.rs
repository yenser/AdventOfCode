mod board;

use board::{Board};

const FILE_PATH: &str = "resources/input.txt";

fn part1() {
    println!("File: {}", FILE_PATH);
    let mut board = Board::new("Board");
    board.read_vents_from_file(FILE_PATH);

    // board.print();

    let total = board.add_twos_or_more();

    println!("Total: {}", total);
}

fn main() {
    part1();
    // part2();
}
