use std::fs::File;
use std::io::{ BufRead, BufReader };

const FILE_PATH: &str = "resources/input.txt";

fn part1(reader: BufReader<File> ) {
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;
    
    for line in reader.lines() {
        let line = line.unwrap();

        let split = line.split(" ");
        let instructions: Vec<&str> = split.collect();
        let direction = instructions[0];
        let distance = instructions[1];

        match direction {
            "forward" => horizontal += distance.parse::<i32>().unwrap(),
            "down" => depth += distance.parse::<i32>().unwrap(),
            "up" => depth -= distance.parse::<i32>().unwrap(),
            _ => {}
        }
    }

    println!("Horizontal: {}", horizontal);
    println!("Depth: {}", depth);
    println!("Value: {}", horizontal * depth);
}


fn part2(reader: BufReader<File> ) {
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;
    let mut aim: i32 = 0;
    
    for line in reader.lines() {
        let line = line.unwrap();

        let split = line.split(" ");
        let instructions: Vec<&str> = split.collect();
        let direction = instructions[0];
        let distance = instructions[1];

        match direction {
            "forward" => {
                let dis: i32 = distance.parse::<i32>().unwrap();
                horizontal += dis;
                depth += dis * aim;
            },
            "down" => aim += distance.parse::<i32>().unwrap(),
            "up" => aim -= distance.parse::<i32>().unwrap(),
            _ => {}
        }
    }

    println!("Horizontal: {}", horizontal);
    println!("Depth: {}", depth);
    println!("Value: {}", horizontal * depth);
}


fn main() {
    println!("File: {}", FILE_PATH);
    let file = File::open(FILE_PATH).unwrap();
    let reader = BufReader::new(file);

    // part1(reader);
    part2(reader);
}
