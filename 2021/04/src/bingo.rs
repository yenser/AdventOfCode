use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};

const BOARD_SIZE: usize = 5;

#[derive(Copy, Clone)]
pub struct BingoTile {
  pub value: u32,
  pub is_marked: bool,
}

#[derive(Copy, Clone)]
pub struct BingoBoard {
  pub layout: [[BingoTile; BOARD_SIZE]; BOARD_SIZE],
}

impl BingoBoard {
  pub fn new() -> BingoBoard {
    let &tile = &BingoTile {
      value: 0,
      is_marked: false,
    };
    let board = BingoBoard {
      layout: [[tile; BOARD_SIZE]; BOARD_SIZE],
    };
    return board;
  }

  pub fn print(&self) {
    for y in self.layout.iter() {
      for (i, x) in y.iter().enumerate() {
        if x.is_marked {
          print!("[{}]", x.value);
        } else {
          print!("{}", x.value);
        }
        if i != y.len() {
          print!("\t");
        }
      }
      print!("\n")
    }
    print!("\n");
  }

  pub fn call(&mut self, val: u32) {
    'outer: for (i, row) in self.layout.iter().enumerate() {
      for (j, tile) in row.iter().enumerate() {
        if tile.value == val {
          self.layout[i][j].is_marked = true;
          break 'outer;
        }
      }
    }
  }

  pub fn call_and_validate_win(&mut self, val: u32) -> bool {
    self.call(val);

    return self.check_win();
  }

  pub fn check_win(&self) -> bool {
    // horizontal win
    for row in self.layout {
      let mut is_win = true;

      for tile in row {
        is_win = is_win && tile.is_marked;
      }

      if is_win {
        return true;
      }
    }

    // vertical win
    for (i, _) in self.layout.iter().enumerate() {
      let mut is_win = true;

      for tile in self.layout {
        is_win = is_win && tile[i].is_marked;
      }

      if is_win {
        return true;
      }
    }

    // diagonal win left
    let mut is_win = true;
    for (i, row) in self.layout.iter().enumerate() {
      is_win = is_win && row[i].is_marked;
    }
    if is_win {
      return true;
    }

    // diagonal win right
    let mut is_win = true;
    for (i, row) in self.layout.iter().enumerate() {
      is_win = is_win && row[row.len() - 1 - i].is_marked;
    }
    if is_win {
      return true;
    }

    return false;
  }

  pub fn add_unmarked(&self) -> u32 {
    let mut num: u32 = 0;

    for row in self.layout {
      for tile in row {
          if !tile.is_marked {
            num += tile.value;
          }
      }
    }

    return num;
  }
}

pub fn read_bingo_game_from_file(file_name: &str) -> Result<(Vec<u32>, Vec<BingoBoard>)> {
  let file = File::open(file_name)?;
  let reader = BufReader::new(file);

  let mut lines = reader.lines();

  let first_line = lines.next().unwrap()?;

  let calls: Vec<u32> = first_line
    .split(",")
    .into_iter()
    .map(|x| x.parse::<u32>().unwrap())
    .collect(); // grab numbers from line and convert to i32

  let mut boards: Vec<BingoBoard> = vec![];

  let mut i = 0;
  for line in lines {
    let line = line.unwrap();
    let row: Vec<&str> = line.trim().split_whitespace().collect();
    if row.len() != BOARD_SIZE {
      boards.push(BingoBoard::new());
    } else {
      let row: Vec<u32> = row.iter().map(|x| x.parse::<u32>().unwrap()).collect(); // grab numbers from line and convert to i32

      // insert fields into board
      for (j, x) in row.iter().enumerate() {
        let r = i % BOARD_SIZE;

        boards.last_mut().unwrap().layout[r][j].value = *x;
      }
      i += 1;
    }
  }

  return Ok((calls, boards));
}
