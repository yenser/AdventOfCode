pub struct Bingo_Tile {
  value: u32,
  is_marked: bool,
}

pub struct Bingo_Board {
  layout: [[Bingo_Tile; 5];5],
}

impl Bingo_Board {

  pub fn print_board(&self) {
    println!("BINGO BOARD PRINTED")
  }
}
