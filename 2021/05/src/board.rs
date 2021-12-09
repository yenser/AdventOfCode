use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};

const BOARD_SIZE: usize = 10;

struct Coord {
  pub x: usize,
  pub y: usize,
}

pub struct Board {
  pub name: String,
  pub layout: [[u32; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
  pub fn new(name: &str) -> Board {
    let board = Board {
      name: name.to_string(),
      layout: [[0; BOARD_SIZE]; BOARD_SIZE],
    };
    return board;
  }

  // pub fn new() -> Board {
  //   Board {
  //     name: "Board",
  //     layout: [[0; BOARD_SIZE]; BOARD_SIZE],
  //   }
  // }

  pub fn print(&self) {
    println!("{}", self.name);
    for y in self.layout {
      for (i, x) in y.iter().enumerate() {
        if *x == 0 {
          print!(".");
        } else {
          print!("{}", x);
        }
        if i != y.len() {
          print!("\t");
        }
      }
      print!("\n")
    }
    print!("\n");
  }

  // pub fn call(&mut self, val: u32) {
  //   'outer: for (i, row) in self.layout.iter().enumerate() {
  //     for (j, tile) in row.iter().enumerate() {
  //       if tile.value == val {
  //         self.layout[i][j].is_marked = true;
  //         break 'outer;
  //       }
  //     }
  //   }
  // }

  // pub fn call_and_validate_win(&mut self, val: u32) -> bool {
  //   self.call(val);

  //   return self.check_win();
  // }

  // pub fn add_unmarked(&self) -> u32 {
  //   let mut num: u32 = 0;

  //   for row in self.layout {
  //     for tile in row {
  //         if !tile.is_marked {
  //           num += tile.value;
  //         }
  //     }
  //   }

  //   return num;
  // }
  pub fn add_point(&mut self, x: usize, y: usize) {
    self.layout[x][y] += 1;
  }

  pub fn read_vents_from_file(&mut self, file_name: &str) {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
      let line = line.unwrap();

      let (mut coord1, coord2) = read_coord_from_string(line).unwrap();

      let x_dir: i32 = if (coord1.x - coord2.x) < 0 { 1 } else if coord1.x - coord2.x > 0 { -1 } else { 0 };
      let y_dir: i32 = if coord1.y - coord2.y < 0 { 1 } else if coord1.y - coord2.y > 0 { -1 } else { 0 };

      while coord1.x != coord2.x && coord1.y != coord2.y {
        self.add_point(coord1.x, coord1.y);

        if coord1.x != coord2.x {
          coord1.x = add_dir(coord1.x, x_dir);
        }

        if coord1.y != coord2.y {
          coord1.y = add_dir(coord1.y, y_dir);
        }
      }

      self.add_point(0, 0);
    }
  }
}

fn add_dir(val: usize, dir: i32) -> usize {
  if dir < 0 {
    val - 1
  } else if dir > 0 {
    val + 1
  } else {
    val
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

  let pos1_str: Vec<usize> = vent_instructions[0]
    .split(",")
    .into_iter()
    .map(|x| x.parse::<usize>().unwrap())
    .collect();
  let pos1: Coord = Coord {
    x: pos1_str[0],
    y: pos1_str[1],
  };

  let pos2_str: Vec<usize> = vent_instructions[2]
    .split(",")
    .into_iter()
    .map(|x| x.parse::<usize>().unwrap())
    .collect();

  let pos2: Coord = Coord {
    x: pos2_str[0],
    y: pos2_str[1],
  };

  return Ok((pos1, pos2));
}
