use std::fs;
use std::io::Result;
use std::str::Split;

const FILE_NAME: &str = "resources/input.txt";


fn part1(lines: Split<'_, &str>) -> Result<()> {
    let mut total = 0;

    for l in lines {
        let d = parse_dimentions(l)?;
        let sqft= get_surface_area(d[0], d[1], d[2]);
        let extra = get_area_of_smallest_side(d[0], d[1], d[2]);

        total += sqft + extra;
    }

    println!("Total sqft: {}", total);
    return Ok(());
}

fn part2(lines: Split<'_, &str>) -> Result<()> {
    let mut total = 0;

    for l in lines {
        let d = parse_dimentions(l)?;
        let ribbon_len = get_ribbon_length(d[0], d[1], d[2]);
        let bow_len = get_ribbon_bow_length(d[0], d[1], d[2]);

        // println!("{} + {}", ribbon_len, bow_len);
        total += ribbon_len + bow_len;
    }

    println!("Total ribbon length: {}", total);
    return Ok(());
}

fn get_surface_area(l: i32, w: i32, h: i32) -> i32 {
    return (2*l*w) + (2*w*h) + (2*h*l);
}

fn get_ribbon_length(l: i32, w: i32, h: i32) -> i32 {
    let mut short_side1: i32 = l;
    let mut short_side2: i32 = w;

    if w >= l && w >= h {
        short_side1 = l;
        short_side2 = h;
    } else if l >= h && l >= w {
        short_side1 = h;
        short_side2 = w;
    }

    return (2*short_side1) + (2 * short_side2);
}

fn get_ribbon_bow_length(l: i32, w: i32, h: i32) -> i32 {
    return l * w * h;
}

fn get_area_of_smallest_side(l: i32, w: i32, h: i32) -> i32 {
    
    let side1 = l * w;
    let side2 = w * h;
    let side3 = h * l;
    let mut smallest = side1;
    
    if side2 < side1 {
        smallest = side2;
    }

    if side3 < smallest {
        smallest = side3;
    }

    return smallest;
}

fn parse_dimentions(line: &str) -> Result<[i32; 3]> {
    let mut i: [i32; 3] = [0; 3];
    let parts: Vec<&str> = line.split("x").collect();

    i[0] = parts[0].parse::<i32>().unwrap();
    i[1] = parts[1].parse::<i32>().unwrap();
    i[2] = parts[2].parse::<i32>().unwrap();

    return Ok(i);
}

fn main() -> Result<()> {
    let content = fs::read_to_string(FILE_NAME)?;

    let lines = content.split("\n");

    part1(lines.clone())?;
    part2(lines.clone())?;

    return Ok(());
}
