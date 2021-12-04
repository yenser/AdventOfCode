use std::fs::File;
use std::io::{ BufRead, BufReader };

const FILE_PATH: &str = "resources/input.txt";
const WINDOW_SIZE: usize = 3;

fn part1(reader: BufReader<File> ) {
    let mut prev: i32 = 0;

    let mut total_increased: i32 = 0;
    
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        
        let val:i32 = match line.parse::<i32>() {
            Ok(v) => v,
            Err(_) => break
        };
        
        let measurement: &str;
        if index != 0 {
            if (val - prev) >= 0 {
                measurement = "increased";
                total_increased += 1;
            } else  { measurement = "decreased"};
            prev = val;
        } else {
            measurement = "N/A - no previous measurement";
        }

        println!("{} ({})", val, measurement);
    }

    println!("Total Increased: {}", total_increased)
}

fn add_value_to_stack(mut arr: Vec<i32>, val: i32) -> Vec<i32> {
    arr.push(val);
    if arr.len() > WINDOW_SIZE {
        arr.remove(0);
    }
    return arr;
}

fn sum_stack(arr: &Vec<i32>) -> i32 {
    let mut i = 0;
    for val in arr {
        i += val;
    }
    return i;
}

fn part2(reader: BufReader<File> ) {
    let mut prev: i32 = 0;
    let mut window: Vec<i32> = Vec::new();

    let mut total_increased: i32 = 0;
    
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        
        let val:i32 = match line.parse::<i32>() {
            Ok(v) => v,
            Err(_) => break
        };

        window = add_value_to_stack(window, val);
        
        let measurement: &str;

        if window.len() == WINDOW_SIZE {
            if index != WINDOW_SIZE - 1 {
                let sum = sum_stack(&window);
                if (sum - prev) > 0 {
                    measurement = "increased";
                    total_increased += 1;
                } else if sum - prev < 0 { measurement = "decreased"}
                else { measurement = "no change"};
                prev = sum;
            } else {
                measurement = "N/A - no previous measurement";
            }
    
            println!("{} ({})", prev, measurement);
        }
    }

    println!("Total Increased: {}", total_increased)
}

fn main() {
    println!("File: {}", FILE_PATH);
    let file = File::open(FILE_PATH).unwrap();
    let reader = BufReader::new(file);

    // part1(reader);
    part2(reader);
}
