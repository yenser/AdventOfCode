use std::collections::HashMap;
use std::fs;
use std::io::Result;

const FILE_NAME: &str = "resources/input.txt";


fn part1(content: &String) -> Result<()> {

    let mut x = 0;
    let mut y = 0;

    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    map.insert(x, vec![0]);

    for i in content.chars() {
        print!("{}", i);

        match i {
            '^' => handle_up(&x, &mut y, &mut map),
            'v' => handle_down(&x, &mut y, &mut map),
            '<' => handle_left(&mut x, &y, &mut map),
            '>' => handle_right(&mut x, &y, &mut map),
            _ => println!("\nunknown char {}", i)
        }
    }

    let unique_locations = calculate_unique_locations(&map);

    println!("\nPart 1: Total unique locations: {}", unique_locations);

    return Ok(());
}

fn part2(content: &String) -> Result<()> {
    let mut x1 = 0;
    let mut y1 = 0;

    let mut x2 = 0;
    let mut y2 = 0;

    let mut map1: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut map2: HashMap<i32, Vec<i32>> = HashMap::new();
    map1.insert(x1, vec![0]);
    map2.insert(x2, vec![0]);

    for (i, v) in content.chars().enumerate() {
        // print!("{}", i);

        if i % 2 == 0 {
            match v {
                '^' => handle_up(&x1, &mut y1, &mut map1),
                'v' => handle_down(&x1, &mut y1, &mut map1),
                '<' => handle_left(&mut x1, &y1, &mut map1),
                '>' => handle_right(&mut x1, &y1, &mut map1),
                _ => println!("\nunknown char {}", i)
            }
        } else {
            match v {
                '^' => handle_up(&x2, &mut y2, &mut map2),
                'v' => handle_down(&x2, &mut y2, &mut map2),
                '<' => handle_left(&mut x2, &y2, &mut map2),
                '>' => handle_right(&mut x2, &y2, &mut map2),
                _ => println!("\nunknown char {}", i)
            }
        }
    }

    let unique_locations = calculate_unique_locations_from_two_santas(&map1, &map2);

    println!("\nPart 2: Total unique locations: {}", unique_locations);

    return Ok(());
}






fn handle_up(x: &i32, y: &mut i32, map: &mut HashMap<i32, Vec<i32>>) {
    *y = *y + 1;
    record_map(x, y, map);
}

fn handle_down(x: &i32, y: &mut i32, map: &mut HashMap<i32, Vec<i32>>) {
    *y = *y - 1;
    record_map(x, y, map);
}

fn handle_left(x: &mut i32, y: & i32, map: &mut HashMap<i32, Vec<i32>>) {
    *x = *x - 1;
    record_map(x, y, map);
}

fn handle_right(x: &mut i32, y: &i32, map: &mut HashMap<i32, Vec<i32>>) {
    *x = *x + 1;
    record_map(x, y, map);
}

fn calculate_unique_locations(map: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut n = 0;

    for (x, value) in map {
        n += value.len() as i32;

        // println!("\n{} has {:?}", x, value);
    }

    return n;
}

fn calculate_unique_locations_from_two_santas(map1: &HashMap<i32, Vec<i32>>, map2: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut n = 0;

    // get all from map 1
    for (x, value) in map1 {
        n += value.len() as i32;

        // println!("\n{} has {:?}", x, value);
    }

    // get all from map 2 if not exists in map 1
    for (x, value) in map2 {
        let map1_y = map1.get(x);

        if map1_y.is_some() {
            value.iter().for_each(|f| {
                if !map1_y.unwrap().contains(f) {
                    n += 1;
                }
            });
        } else {
            n += value.len() as i32;
        }

        // println!("\n{} has {:?}", x, value);
    }


    return n;
}

fn record_map(x: &i32, y: &i32, map: &mut HashMap<i32, Vec<i32>>) {
    let exists = map.get_mut(x);

    if exists.is_some() {
        let list = exists.unwrap();
            if !list.contains(y) {
            list.push(*y);
            // println!("new location {} {}", *x, *y);
        }
    } else {
        map.insert(*x, vec![y.clone()]);
        // println!("new location {} {}", *x, *y);
    }
}

fn main() -> Result<()> {
    let content = fs::read_to_string(FILE_NAME)?;

    part1(&content)?;
    part2(&content)?;

    return Ok(());
}
