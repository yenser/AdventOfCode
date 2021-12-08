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
            let win: bool = board.call_and_validate_win(call);

            if win {
                println!("Win Found");

                return Ok((board.clone(), call));
            }
        }
    }

    Err(())
}

fn play_bingo_and_find_last_winner(game: Vec<u32>, mut boards: Vec<BingoBoard>) -> (BingoBoard, u32) {

    let mut recent_winner_board: Option<BingoBoard> = None;
    let mut recent_winner_number: Option<u32> = None;
    for call in game {

        boards = boards.into_iter().filter_map(|mut board| {

            board.call(call);
            let win: bool = board.check_win();

            if win {
                println!("Win Found");

                recent_winner_board = Some(board.clone());
                recent_winner_number = Some(call.clone());

                return None;
            }

            return Some(board);
        }).collect();
    }

    return (recent_winner_board.unwrap(), recent_winner_number.unwrap());
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
    let (game, boards) = bingo::read_bingo_game_from_file(FILE_PATH).unwrap();

    let (last_winner_board, winning_number)  = play_bingo_and_find_last_winner(game, boards);

    last_winner_board.print();

    let x = last_winner_board.add_unmarked();

    println!("Value: {}", x * winning_number);
}

fn main() {
    // part1();
    part2();
}