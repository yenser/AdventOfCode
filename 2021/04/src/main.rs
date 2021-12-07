mod bingo;

use bingo::BingoBoard;

const FILE_PATH: &str = "resources/input.txt";

fn process_line(arr: &mut Vec<i32>, line: String) {
    for (i, c) in line.chars().enumerate() {
        match c {
            '1' => arr[i] += 1,
            _ => {}
        }
    }
}

fn play_bingo(game: Vec<u32>, boards: &mut Vec<BingoBoard>) -> Result<(BingoBoard, u32), ()> {
    for call in game {
        for board in boards.iter_mut() {
            let win: bool = board.call(call);

            if win {
                println!("Win Found");

                return Ok((board.clone(), call));
            }
        }
    }

    Err(())
}

fn main() {
    println!("File: {}", FILE_PATH);
    let (game, mut boards) = bingo::read_bingo_game_from_file(FILE_PATH).unwrap();

    let winner_board = play_bingo(game, &mut boards);
    let (winner_board, winning_number) = winner_board.unwrap();

    winner_board.print();

    let x = winner_board.add_unmarked();

    println!("Value: {}", x * winning_number);
}
