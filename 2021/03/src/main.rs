use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;

    let mut size: i32 = 0;

    let mut bit_count: Vec<i32>;

    // run for first item to set up the vector
    let mut lines = reader.lines();
    let line = lines.next().unwrap();
    bit_count = vec![0; line.as_ref().unwrap().chars().count()];
    process_line(&mut bit_count, line.unwrap().to_string());
    size += 1;

    println!("Bit Size: {}", bit_count.len());

    // add up all bits in all columns
    for line in lines {
        let line = line.unwrap();
        process_line(&mut bit_count, line);
        size += 1;
    }

    // find most common bit type
    for bit_val in bit_count {
        if bit_val * 2 > size {
            gamma = shift_bit_left(gamma, true);
            epsilon = shift_bit_left(epsilon, false);
        } else {
            gamma = shift_bit_left(gamma, false);
            epsilon = shift_bit_left(epsilon, true);
        }
    }

    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    println!("Power: {}", gamma * epsilon);
}

fn shift_bit_left(mut val: i32, is_one: bool) -> i32 {
    val = val << 1;
    if is_one {
        val = val | 1;
    }

    return val;
}

fn main() {
    println!("File: {}", FILE_PATH);
    let file = File::open(FILE_PATH).unwrap();
    let reader = BufReader::new(file);

    part1(reader);
}
