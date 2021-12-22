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

fn simulate_fish_v2(days: usize) {
    println!("File: {}", FILE_PATH);

    let mut lantern_fish: Vec<u64> = vec![0; 9];
    
    let file = File::open(FILE_PATH).unwrap();
    let reader = BufReader::new(file);
    let list = reader.lines().next().unwrap();
    
    list.unwrap().split(',').for_each(|x| {
        let num = x.parse::<usize>().unwrap();
        
        lantern_fish[num] += 1
    });


    for i in 0..days {
        let fish_to_add = lantern_fish.remove(0);
        lantern_fish[6] += fish_to_add;
        lantern_fish.push(fish_to_add);

        let sum: u64 = lantern_fish.iter().sum();
        println!("After {} day: {} total fish", i + 1, sum);
    }
}

fn main() {
    // simulate_fish(80);
    simulate_fish_v2(256);
}
