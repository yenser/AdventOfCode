use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(file_path: &str) {
    println!("File: {}", file_path);

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut digits: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        let input_output: Vec<&str> = line.split('|').collect();

        let output: Vec<&str> = input_output[1].split_whitespace().collect();
        output.iter().for_each(|x| digits.push(x.to_string()));
    }

    let mut total = 0;
    digits.iter().for_each(|x| {
        let len = x.len();
        if len == 2 || len == 4 || len == 3 || len == 7 { // 1, 4, 7, 8
            total += 1;
        }
    });

    println!("Total unique numbers: {}", total);
    // digits.iter().for_each(|x| {
    //     println!("{} | {}", x, x.len());
    // });
}

fn main() {
    part1("resources/input.txt");
    // part2();
}
