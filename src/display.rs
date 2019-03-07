pub struct Display {
  pub memory: [bool; 2048],
}

impl Display {
  pub fn new() -> Display {
    Display {
      memory: [false; 2048]
    }
  }

  pub fn cls(&mut self) {

  }
}

pub static FONT_SET: [u8; 0] = [];