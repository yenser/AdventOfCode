mod bingo;

use std::fs::File;
use std::io::{BufRead, BufReader};
use bingo::{Bingo_Board, Bingo_Tile};

const FILE_PATH: &str = "resources/input.txt";

fn process_line(arr: &mut Vec<i32>, line: String) {
    for (i, c) in line.chars().enumerate() {
        match c {
            '1' => arr[i] += 1,
            _ => {}
        }
    }
}

fn part1(reader: BufReader<File>) {
    let b: Bingo_Board;
    b.print_board();
}


fn main() {
    println!("File: {}", FILE_PATH);
    let file = File::open(FILE_PATH).unwrap();
    let reader = BufReader::new(file);

    part1(reader);
}
