
pub struct Coords {
  pub x: u64,
  pub y: u64,
}

impl Coords {
  pub fn new(x, y: u64) -> Coords {
    return Coords { x: x, y: y };
  }
}