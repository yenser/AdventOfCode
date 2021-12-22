use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_PATH: &str = "resources/input.txt";

fn simulate_fish(days: usize) {
    println!("File: {}", FILE_PATH);

    let file = File::open(FILE_PATH).unwrap();
    let reader = BufReader::new(file);
    let list = reader.lines().next().unwrap();
    
    let mut lantern_fish: Vec<u8> = list.unwrap().split(',').map(|x| x.parse::<u8>().unwrap()).collect();

    let mut fish_to_add = 0;
    for i in 0..days {
        lantern_fish.iter_mut().for_each(|x| {
            if *x == 0 {
                fish_to_add += 1;
                *x = 6;
            } else {
                *x -= 1
            }
        });

        let mut new_fish: Vec<u8> = vec![8; fish_to_add];

        lantern_fish.append(&mut new_fish);
        // for _ in 0..fish_to_add {
        //     lantern_fish.push(8);
        // }

        fish_to_add = 0;
        println!("After {} day: {} total fish", i + 1, lantern_fish.len());
    }
    // println!("Total: {}", total);
}

// fn part2() {
//     println!("File: {}", FILE_PATH);
//     let mut board = Board::new("Board");
//     board.read_vents_from_file(FILE_PATH);

//     // board.print();

//     let total = board.add_twos_or_more();

//     println!("Total: {}", total);
// }

fn main() {
    // simulate_fish(80);
    simulate_fish(256);
}
