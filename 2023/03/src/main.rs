use std::fs;
use std::io::Result;
use std::time::Instant;

const FILE_NAME: &str = "resources/input.txt";

struct Symbol {
    value: char,
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

    for (y, line) in content.iter().enumerate() {
        let (pn, s) = parse_line(line, y as i32)?;

        if pn.len() > 0 {
            part_numbers.extend(pn);
        }

        if s.len() > 0 {
            symbols.extend(s);
        }
    }

    return Ok((part_numbers, symbols));
}

fn parse_line(line: &str, y: i32) -> Result<(Vec<PartNumber>, Vec<Symbol>)> {
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let mut num: String = String::from("");
    let mut building_num = false;
    let mut xStart = 0;
    for (x, c) in line.chars().enumerate() {

        match c {
            '0'..='9' => {
                num.push(c);

                if !building_num {
                    xStart = x;
                }
                building_num = true;
            },
            _ => {
                if building_num {
                    part_numbers.push(PartNumber { value: num.parse::<i32>().unwrap(), x0: xStart as i32, x1: (x-1) as i32, y: y });
                    num = String::from("");
                    xStart = 0;
                    building_num = false;
                }

                if c != '.' {
                    symbols.push(Symbol { value: c, x: x as i32, y: y });
                }
            },
        }
    }

    if building_num { // catch numbers that end at the end of the line
        part_numbers.push(PartNumber { value: num.parse::<i32>().unwrap(), x0: xStart as i32, x1: line.len() as i32, y: y });
    }
    
    return Ok((part_numbers, symbols));
}


fn part1(content: &Vec<String>) -> Result<()> {
    println!("\nPart 1");

    let mut total = 0;

    let (part_numbers, symbols) = parse(content)?;

    println!("Part Numbers: {}", part_numbers.len());
    println!("Symbols: {}", symbols.len());

    for pn in part_numbers {
        for s in &symbols {

            let y_diff = (pn.y - s.y).abs();
            let x_min = pn.x0 - 1;
            let x_max = pn.x1 + 1;

            if s.x >= x_min && s.x <= x_max && y_diff <= 1 {
                println!("{}: {} {} {} | {}: {} {}", pn.value, pn.x0, pn.x1, pn.y, s.value, s.x, s.y);
                total += pn.value;
                break;
            }
        }
    }

    println!("Total: {}", total);

    return Ok(());
}

fn part2(content: &Vec<String>) -> Result<()> {
    println!("\nPart 2");

    let mut total = 0;

    let (part_numbers, symbols) = parse(content)?;

    println!("Part Numbers: {}", part_numbers.len());
    println!("Symbols: {}", symbols.len());

    for s in &symbols {
        let mut first_value = 0;
        let mut first_found = false;

        for pn in &part_numbers {
            let y_diff = (pn.y - s.y).abs();
            let x_min = pn.x0 - 1;
            let x_max = pn.x1 + 1;

            if pn.y >= s.y + 2 {
                break;
            }

            if s.x >= x_min && s.x <= x_max && y_diff <= 1 {
                if !first_found  {
                    first_value = pn.value;
                    first_found = true;
                } else {
                    println!("{} {} {} : {} {}", s.value, s.x, s.y, first_value, pn.value);
                    total += first_value * pn.value;
                    break;
                }
            }

        }
    }

    println!("Total: {}", total);

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
