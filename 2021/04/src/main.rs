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

fn play_bingo_and_find_last_winner(game: Vec<u32>, boards: &Vec<BingoBoard>) -> (BingoBoard, u32) {

    let mut recent_winner_board: BingoBoard;
    let mut recent_winner_number: u32;
    for call in game {
        boards = boards.iter().filter_map(|&board| {

            let win: bool = board.call(call);

            if win {
                println!("Win Found");

                recent_winner_board = board.clone();
                recent_winner_number = call;

                return Some(board);
            }

            return None;
        }).collect::<Vec<BingoBoard>>();
    }

    return (recent_winner_board, recent_winner_number);
}

fn part1() {
    println!("File: {}", FILE_PATH);
    let (game, mut boards) = bingo::read_bingo_game_from_file(FILE_PATH).unwrap();

    let winner_board = play_bingo(game, &mut boards);
    let (winner_board, winning_number) = winner_board.unwrap();

    winner_board.print();

    let x = winner_board.add_unmarked();

    println!("Value: {}", x * winning_number);
}

fn part2() {
    println!("File: {}", FILE_PATH);
    let (game, mut boards) = bingo::read_bingo_game_from_file(FILE_PATH).unwrap();

    let (last_winner_board, winning_number)  = play_bingo_and_find_last_winner(game, &boards);

    last_winner_board.print();

    let x = last_winner_board.add_unmarked();

    println!("Value: {}", x * winning_number);
}

fn main() {
    // part1();
    part2();
}