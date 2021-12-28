use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(file_path: &str) {
    let table = read_table_from_file(file_path);

    let mut risk_levels = 0;

    for y in 0..table.len() {
        for x in 0..table[y].len() {

            if check_for_lowest_point(&table, y, x) {
                risk_levels += table[y][x] + 1;
            }
        }
    }

    println!("Risk Level: {}", risk_levels);
}

fn check_for_lowest_point(table: &Vec<Vec<u32>>, y: usize, x: usize) -> bool {
    let val = table[y][x];
    if x != 0 {
        if val >= table[y][x-1] {
            return false;
        } 
    }

    if x != table[y].len()-1 {
        if val >= table[y][x+1] {
            return false;
        } 
    }

    if y != 0 {
        if val >= table[y-1][x] {
            return false;
        } 
    }

    if y != table.len()-1 {
        if val >= table[y+1][x] {
            return false;
        } 
    }

    return true;
}

fn read_table_from_file(file_path: &str) -> Vec<Vec<u32>> {
    println!("File: {}", file_path);

    const RADIX: u32 = 10;

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut table: Vec<Vec<u32>> = vec![];


    for line in reader.lines() {
        let line = line.unwrap();

        let mut row: Vec<u32> = vec![0; line.len()];

        for (i, n) in line.chars().enumerate() {
            row[i] = n.to_digit(RADIX).unwrap();
        }

        table.push(row);
    }

    println!("Table Height: {}", table.len());
    if table.len() != 0 {
        println!("Table Length: {}", table[0].len());
    }
    return table;
}

fn main() {
    part1("resources/input.txt");
    // part2("resources/exampleInput2.txt");
}
