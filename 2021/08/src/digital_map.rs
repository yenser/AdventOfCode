
pub struct DigitalMap {
  map: [char; 8],
  unprocessed_codes: Vec<String>
}

impl DigitalMap {
  pub fn new() -> DigitalMap {
    DigitalMap {
      map: ['\0'; 8],
      unprocessed_codes: vec![],
    }
  }

  pub fn map_value(&mut self, val: &str) {
    match val.len() {
      2 => { // map 1 value
        self.map[2] = val.chars().nth(0).unwrap();
        self.map[5] = val.chars().nth(1).unwrap();
      },
      4 => { // map 4 value
        self.map[1] = val.chars().nth(0).unwrap();
        self.map[2] = val.chars().nth(1).unwrap();
        self.map[3] = val.chars().nth(2).unwrap();
        self.map[5] = val.chars().nth(3).unwrap();
      },
      3 => { // map 7 value
        self.map[0] = val.chars().nth(0).unwrap();
        self.map[2] = val.chars().nth(1).unwrap();
        self.map[5] = val.chars().nth(2).unwrap();
      },
      7 => { // map 8 value
        for i in 0..val.len() { self.map[i] = val.chars().nth(i).unwrap() }
      },
      _ => self.unprocessed_codes.push(val.to_string())
    }
  }

  pub fn print(&self) {
    println!("\n {}{}{}{} ", self.map[0], self.map[0], self.map[0], self.map[0]);
    println!("{}    {}", self.map[1], self.map[2]);
    println!("{}    {}", self.map[1], self.map[2]);
    println!(" {}{}{}{} ", self.map[3], self.map[3], self.map[3], self.map[3]);
    println!("{}    {}", self.map[4], self.map[5]);
    println!("{}    {}", self.map[4], self.map[5]);
    println!(" {}{}{}{} \n", self.map[6], self.map[6], self.map[6], self.map[6]);

    println!("{} total unprocessed\n", self.unprocessed_codes.len());
    // println!("[{}] [{}] [{}] [{}] [{}] [{}] [{}] | {} unprocessed", self.map[0], self.map[1], self.map[2], self.map[3], self.map[4], self.map[5], self.map[6], self.unprocessed_codes.len())
  }
}