use std::fs;
use std::io::Result;
use std::time::Instant;

const FILE_NAME: &str = "resources/exampleInput.txt";

struct Symbol {
    x: i32,
    y: i32,
}

struct PartNumber {
    value: i32,
    x0: i32,
    x1: i32,
    y: i32,
}

fn parse(content: &Vec<String>) -> Result<(Vec<PartNumber>, Vec<Symbol>)> {
    println!("Parsing {} lines", content.len());

    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for line in content {
        
    }

    return Ok((part_numbers, symbols));
}

fn parse_line(line: &str, y: i32) -> Result<(Vec<PartNumber>, Vec<Symbol>)> {
    let mut part_number: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (x, c) in line.chars().enumerate() {
        let mut num: String = String::from("");
        let mut building_num = false;

        match c {
            '0'..='9' => {
                let value = c.to_digit(10).unwrap() as i32;
                part_number.push(PartNumber { value, x0: 0, x1: 0, y: 0 });
            },
            _ => {
                if building_num {
                    num.push(c);
                    building_num = true;
                }

                if c != '.' {
                    symbols.push(Symbol { x: x as i32, y: y });
                }
            },
        }
    }
    
    return Ok((part_number, symbols));
}


fn part1(content: &Vec<String>) -> Result<()> {
    println!("\nPart 1");


    return Ok(());
}

fn part2(content: &Vec<String>) -> Result<()> {
    println!("\nPart 2");


    return Ok(());
}


fn main() -> Result<()> {
    let content = fs::read_to_string(FILE_NAME)?.lines().map(String::from).collect();
    let mut now = Instant::now();

    part1(&content)?;
    println!("process took {:?}", now.elapsed());

    now = Instant::now();
    part2(&content)?;
    println!("process took {:?}", now.elapsed());

    return Ok(());
}
