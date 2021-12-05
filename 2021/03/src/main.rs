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

    let mut bit_arr: Vec<i32>;

    // run for first item to set up the vector
    let mut lines = reader.lines();
    let line = lines.next().unwrap();
    bit_arr = vec![0; line.as_ref().unwrap().chars().count()];
    process_line(&mut bit_arr, line.unwrap().to_string());
    size += 1;

    println!("Bit Size: {}", bit_arr.len());

    // add up all bits in all columns
    for line in lines {
        let line = line.unwrap();
        process_line(&mut bit_arr, line);
        size += 1;
    }

    // find most common bit type
    for bit_val in bit_arr {
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

fn part2(reader: BufReader<File>) {
    let mut data_arr: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        data_arr.push(line);
    }

    println!("Array Size: {}", data_arr.len());
    println!("String Lengths: {}\n", data_arr[0].len());

    let oxygen_rate = find_oxygen_generator_rating(data_arr.clone(), 0); // I wish I didn't have to clone here but for now its the best I can find.
    println!("Oxygen Rate: {}", oxygen_rate);
    let oxygen_val = string_binary_to_number(oxygen_rate);
    println!("Oxygen Value: {}\n", oxygen_val);

    let co2_rate = find_co2_generator_rating(data_arr.clone(), 0);
    println!("CO2 Rate: {}", co2_rate);
    let co2_val = string_binary_to_number(co2_rate);
    println!("CO2 Value: {}\n", co2_val);

    println!("Answer: {}", oxygen_val * co2_val);
}

fn find_oxygen_generator_rating(arr: Vec<String>, index: usize) -> String {
    let first_val = &arr[0];
    if index >= first_val.len() || arr.len() == 1 { return first_val.clone() } // end recursion

    let mut count: i32 = 0;
    let arr_size = arr.len();
    let mut one_arr: Vec<String> = vec![];
    let mut zero_arr: Vec<String> = vec![];

    for x in arr.into_iter().as_ref() {
        match x.chars().nth(index).unwrap() {
            '1' => {
                one_arr.push(x.clone());
                count += 1;
            },
            '0' => { zero_arr.push(x.clone()) },
            _ => {}
        }
    }

    if count * 2 >= arr_size as i32 {
        return find_oxygen_generator_rating(one_arr, index + 1);
    } else {
        return find_oxygen_generator_rating(zero_arr, index + 1);
    }
}

fn find_co2_generator_rating(arr: Vec<String>, index: usize) -> String {
    let first_val = &arr[0];

    if index >= first_val.len() || arr.len() == 1 { return first_val.clone() } // end recursion

    let mut count: i32 = 0;
    let arr_size = arr.len();
    let mut one_arr: Vec<String> = vec![];
    let mut zero_arr: Vec<String> = vec![];

    for x in arr.into_iter().as_ref() {
        match x.chars().nth(index).unwrap() {
            '1' => {
                one_arr.push(x.clone());
                count += 1;
            },
            '0' => { zero_arr.push(x.clone()) },
            _ => {}
        }
    }

    if count * 2 < arr_size as i32 {
        return find_co2_generator_rating(one_arr, index + 1);
    } else {
        return find_co2_generator_rating(zero_arr, index + 1);
    }
}

fn shift_bit_left(mut val: i32, is_one: bool) -> i32 {
    val = val << 1;
    if is_one {
        val = val | 1;
    }

    return val;
}

fn string_binary_to_number(val: String) -> i32 {

    let mut number: i32 = 0;

    for c in val.chars() {
        match c {
            '0' => number = shift_bit_left(number, false),
            '1' => number = shift_bit_left(number, true),
            _ => {}
        }
    }

    return number;
}

fn main() {
    println!("File: {}", FILE_PATH);
    let file = File::open(FILE_PATH).unwrap();
    let reader = BufReader::new(file);

    // part1(reader);
    part2(reader);
}
