use std::fs;
use std::io::Result;

const FILE_NAME: &str = "resources/input.txt";


fn part1(content: &String) -> Result<()> {
    return Ok(());
}

fn part2(content: &String) -> Result<()> {
    return Ok(());
}

fn main() -> Result<()> {
    let content = fs::read_to_string(FILE_NAME)?;

    part1(&content)?;
    part2(&content)?;

    return Ok(());
}
