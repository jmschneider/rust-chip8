use display::Display;
use keypad::Keypad;

pub struct Cpu {
  // index register
  pub i: u16,
  // program counter
  pub pc: u16,
  // memory
  pub memory: [u8; 4096],
  // registers
  pub v: [u8; 16],
  // peripherals
  pub display: Display,
  pub keypad: Keypad,
  // stack
  pub stack: [u16; 16],
  // stack pointer
  pub sp: u8,
  // delay timer
  pub dt: u8
}

impl Cpu {
  pub fn execute_cycle(&mut self) {
    let opcode: u16 = read_word(self.memory, self.pc)
    self.process_opcode(opcode)
  }

  fn process_opcode(&mut self, opcode: u16) {

  }
}

fn read_word(memory: [u8, 4096], index: u16) -> u16 {
  // this is combining to 2 u8 values into 1 u16 value. Left shifted first by 8 OR unshifted second byte
  // for 00110011 and 11011101, it becomes 0011001100000000 OR 11011101 which equals 0011001111011101
  (memory[index as usize] as u16) << 8 | (memory[(index + 1) as usize] as u16)
}