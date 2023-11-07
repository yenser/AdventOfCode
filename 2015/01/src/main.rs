use std::fs;
use std::io::Result;

const FILE_NAME: &str = "resources/input.txt";


fn part1(content: &String) -> Result<()> {
    let mut floor = 0;

    for c in content.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }

    println!("Floor: {}", floor);

    return Ok(());
}

fn part2(content: &String) -> Result<()> {
    let mut floor = 0;

    for (i, c) in content.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        if floor == -1 {
            println!("Basement at: {}", i);
            break;
        }
    }

    println!("Floor: {}", floor);

    return Ok(());
}

fn main() -> Result<()> {
    let content = fs::read_to_string(FILE_NAME)?;

    part1(&content)?;
    part2(&content)?;

    return Ok(());
}
