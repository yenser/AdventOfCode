use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_PATH: &str = "resources/input.txt";

fn part1() {
    println!("File: {}", FILE_PATH);

    let file = File::open(FILE_PATH).unwrap();
    let reader = BufReader::new(file);
    let list = reader.lines().next().unwrap();
    
    let lantern_fish: Vec<i64> = list.unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect();

    let max_pos: i64 = *lantern_fish.iter().max().unwrap();

    let mut cheapest_fuel: i64 = 0;
    let mut cheapest_fuel_pos: i64 = 0;
    for i in 0..=max_pos {
        
        let mut total_fuel_used: i64 = 0;
        lantern_fish.iter().for_each(|f| {
            let fuel_used = (*f - i).abs();
            total_fuel_used += fuel_used;
            // println!("Move from {} to {}: {}", *f, i, fuel_used);
        });

        if total_fuel_used < cheapest_fuel || cheapest_fuel == 0 {
            cheapest_fuel = total_fuel_used;
            cheapest_fuel_pos = i;
        }
    }

    println!("Best fuel usage is {} at position {}", cheapest_fuel, cheapest_fuel_pos);
}

fn part2() {
    println!("File: {}", FILE_PATH);

    let file = File::open(FILE_PATH).unwrap();
    let reader = BufReader::new(file);
    let list = reader.lines().next().unwrap();
    
    let lantern_fish: Vec<i64> = list.unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect();

    let max_pos: i64 = *lantern_fish.iter().max().unwrap();

    let mut cheapest_fuel: i64 = 0;
    let mut cheapest_fuel_pos: i64 = 0;
    for i in 0..=max_pos {
        
        let mut total_fuel_used: i64 = 0;
        lantern_fish.iter().for_each(|f| {
            let fuel_used = calculate_fuel_usage(*f, i);
            total_fuel_used += fuel_used;
            // println!("Move from {} to {}: {}", *f, i, fuel_used);
        });

        if total_fuel_used < cheapest_fuel || cheapest_fuel == 0 {
            cheapest_fuel = total_fuel_used;
            cheapest_fuel_pos = i;
        }

        // println!();
    }

    println!("Best fuel usage is {} at position {}", cheapest_fuel, cheapest_fuel_pos);
}

fn calculate_fuel_usage(p1: i64, p2: i64) -> i64 {
    let mut total = 0;

    let distance = (p1 - p2).abs();

    for i in 1..=distance {
        total += i;
    }

    return total;
}

fn main() {
    // part1();
    part2();
}
