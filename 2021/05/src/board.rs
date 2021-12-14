use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};

const BOARD_SIZE: usize = 1000;

struct Coord {
  pub x: u32,
  pub y: u32,
}

pub struct Board {
  pub name: String,
  pub layout: Vec<Vec<u32>>,
}

impl Board {
  pub fn new(name: &str) -> Board {
    let board = Board {
      name: name.to_string(),
      layout: vec![vec![0; BOARD_SIZE]; BOARD_SIZE],
    };
    return board;
  }

  pub fn print(&self) {
    println!("{}", self.name);
    for y in self.layout.iter() {
      for (i, x) in y.iter().enumerate() {
        if *x == 0 {
          print!(".");
        } else {
          print!("{}", x);
        }
        if i != y.len() {
          print!(" ");
        }
      }
      print!("\n")
    }
    print!("\n");
  }

  pub fn add_point(&mut self, x: usize, y: usize) {
    self.layout[y][x] += 1;
  }

  pub fn read_vents_from_file_no_diagonal(&mut self, file_name: &str) {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
      let line = line.unwrap();

      let (coord1, coord2) = read_coord_from_string(line).unwrap();

      if coord1.x == coord2.x {
        // println!("{},{} -> {},{}", coord1.x, coord1.y, coord2.x, coord2.y);
        let diff = coord1.y as i32 - coord2.y as i32;

        if diff < 0 {
          for i in 0..(diff.abs() + 1) {
            // println!("dy: {}", coord1.y as i32 + i);
            self.add_point(coord1.x as usize, (coord1.y as i32 + i) as usize);
          }
        } else {
          for i in 0..(diff + 1) {
            // println!("dy: {}", coord1.y as i32 - i);
            self.add_point(coord1.x as usize, (coord1.y as i32 - i) as usize);
          }
        }
        // self.print();
      } else if coord1.y == coord2.y {
        // println!("{},{} -> {},{}", coord1.x, coord1.y, coord2.x, coord2.y);
        let diff = coord1.x as i32 - coord2.x as i32;

        if diff < 0 {
          for i in 0..(diff.abs() + 1) {
            // println!("dx: {}\t{}", coord1.y as i32 + i, diff);
            self.add_point((coord1.x as i32 + i) as usize, coord1.y as usize);
          }
        } else {
          for i in 0..(diff + 1) {
            // println!("dx: {}\t{}", coord1.x as i32 - i, diff);
            self.add_point((coord1.x as i32 - i) as usize, coord1.y as usize);
          }
        }

        // self.print();
      } else {
        println!(
          "Throwing out {},{} -> {},{}",
          coord1.x, coord1.y, coord2.x, coord2.y
        );
      }
    }
  }

  pub fn read_vents_from_file(&mut self, file_name: &str) {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut throw_out_count: u32 = 0;
    let mut horizontal_count: u32 = 0;
    let mut vertical_count: u32 = 0;
    let mut diagonal_count: u32 = 0;

    for line in reader.lines() {
      let line = line.unwrap();

      let (coord1, coord2) = read_coord_from_string(line).unwrap();

      let incline_value: f32 =
        (coord1.y as i32 - coord2.y as i32) as f32 / (coord1.x as i32 - coord2.x as i32) as f32;
      // Horizontal
      if coord1.x == coord2.x {
        let diff = coord1.y as i32 - coord2.y as i32;

        if diff < 0 {
          for i in 0..(diff.abs() + 1) {
            self.add_point(coord1.x as usize, (coord1.y as i32 + i) as usize);
          }
        } else {
          for i in 0..(diff + 1) {
            self.add_point(coord1.x as usize, (coord1.y as i32 - i) as usize);
          }
        }
        horizontal_count += 1;

        //Vertical
      } else if coord1.y == coord2.y {
        let diff = coord1.x as i32 - coord2.x as i32;

        if diff < 0 {
          for i in 0..(diff.abs() + 1) {
            self.add_point((coord1.x as i32 + i) as usize, coord1.y as usize);
          }
        } else {
          for i in 0..(diff + 1) {
            self.add_point((coord1.x as i32 - i) as usize, coord1.y as usize);
          }
        }
        vertical_count += 1;

        // diagonal
      } else if incline_value.abs() == 1.0 {
        // upper left to bottom right

        let diff = coord1.x as i32 - coord2.x as i32;

        // println!("Inc: {}\tDiff: {}", incline_value, diff);

        if incline_value == 1.0 {
          for i in 0..(diff.abs() + 1) {
            if coord1.x < coord2.x {
              self.add_point(
                (coord1.x as i32 + i) as usize,
                (coord1.y as i32 + i) as usize,
              );
            } else {
              self.add_point(
                (coord2.x as i32 + i) as usize,
                (coord2.y as i32 + i) as usize,
              );
            }
          }

          // bottom left to upper right
        } else {
          for i in 0..(diff.abs() + 1) {
            if coord1.y > coord2.y {
              self.add_point(
                (coord1.x as i32 + i) as usize,
                (coord1.y as i32 - i) as usize,
              );
            } else {
              self.add_point(
                (coord2.x as i32 + i) as usize,
                (coord2.y as i32 - i) as usize,
              );
            }
          }
        }

        diagonal_count += 1;
      } else {
        throw_out_count += 1;
        // println!("Throwing out {},{} -> {},{}", coord1.x, coord1.y, coord2.x, coord2.y);
      }
    }
    println!("Horizontal: {}", horizontal_count);
    println!("Vertical: {}", vertical_count);
    println!("Diagonal: {}", diagonal_count);
    println!("Thrown out: {}", throw_out_count);
  }

  pub fn add_twos_or_more(&self) -> i32 {
    let mut total: i32 = 0;

    for y in self.layout.iter() {
      for x in y.iter() {
        if *x >= 2 {
          total += 1;
        }
      }
    }

    return total;
  }
}

fn read_coord_from_string(line: String) -> Result<(Coord, Coord)> {
  let vent_instructions: Vec<&str> = line.trim().split_whitespace().collect();

  if vent_instructions.len() != 3 {
    return Err(Error::new(
      ErrorKind::Other,
      "Failed to parse line from file",
    ));
  }

  let pos1_str: Vec<u32> = vent_instructions[0]
    .split(",")
    .into_iter()
    .map(|x| x.parse::<u32>().unwrap())
    .collect();
  let pos1: Coord = Coord {
    x: pos1_str[0],
    y: pos1_str[1],
  };

  let pos2_str: Vec<u32> = vent_instructions[2]
    .split(",")
    .into_iter()
    .map(|x| x.parse::<u32>().unwrap())
    .collect();

  let pos2: Coord = Coord {
    x: pos2_str[0],
    y: pos2_str[1],
  };

  return Ok((pos1, pos2));
}
