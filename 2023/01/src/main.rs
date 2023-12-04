use std::fs;
use std::io::Result;

const FILE_NAME: &str = "resources/input.txt";


fn part1(content: &Vec<String>) -> Result<()> {
    println!("\nPart 1");

    let mut total = 0;
    for line in content {
        let value = get_calibration_value(line.to_string());

        println!("{} = {}", line, value);
        total += value;
    }

    println!("Total: {}", total);

    return Ok(());
}

fn part2(content: &Vec<String>) -> Result<()> {
    println!("\nPart 2");

    let mut total = 0;
    for line in content {
        let line_replaced = replace_words_with_numbers(line.to_string());
        let value = get_calibration_value(line_replaced.to_string());

        println!("{} = {}", line_replaced, value);
        total += value;
    }

    println!("Total: {}", total);

    return Ok(());
}

fn get_calibration_value(str: String) -> i32 {
    let mut first: char = '\0';
    let mut last: char = '\0';

    for c in str.chars() {
        if c.is_numeric()  {
            if first == '\0' {
                first = c;
                last = c;
            } else {
                last = c;
            }
        }
    }

    let combined = format!("{}{}", first, last);

    return combined.parse::<i32>().unwrap();
}

fn replace_words_with_numbers(str: String) -> String {
    let mut final_string = str;

    final_string = final_string.replace("one", "o1e");
    final_string = final_string.replace("two", "t2o");
    final_string = final_string.replace("three", "t3e");
    final_string = final_string.replace("four", "f4r");
    final_string = final_string.replace("five", "f5e");
    final_string = final_string.replace("six", "s6x");
    final_string = final_string.replace("seven", "s7n");
    final_string = final_string.replace("eight", "e8t");
    final_string = final_string.replace("nine", "n9e");

    return final_string;
}

fn main() -> Result<()> {
    let content = fs::read_to_string(FILE_NAME)?.lines().map(String::from).collect();

    // part1(&content)?;
    part2(&content)?;

    return Ok(());
}
