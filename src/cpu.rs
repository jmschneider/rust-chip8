use crate::display::{Display, FONT_SET};
use crate::keypad::Keypad;

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
  pub dt: u8,
  // sound timer
  pub st: u8
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      i: 0,
      pc: 0x200,
      memory: [0; 4096],
      v: [0; 16],
      display: Display::new(),
      keypad: Keypad::new(),
      stack: [0; 16],
      sp: 0,
      dt: 0,
      st: 0
      // rand: ComplementaryMultiplyWithCarryGen::new(1)
    }
  }
  
  pub fn reset(&mut self) {
    self.i = 0;
    self.pc = 0x200;
    self.memory = [0; 4096];
    self.v = [0; 16];
    self.stack = [0; 16];
    self.sp = 0;
    self.dt = 0;
    self.st = 0;
    // self.rand = ComplementaryMultiplyWithCarryGen::new(1);
    self.display.cls();
    for i in 0..80 {
      self.memory[i] = FONT_SET[i];
    }
  }

  pub fn execute_cycle(&mut self) {
    let opcode: u16 = read_word(self.memory, self.pc);
    self.process_opcode(opcode);
  }

  pub fn decrement_timers(&mut self) {
    if self.dt > 0 {
      self.dt -= 1;
    }

    if self.st > 0 {
      self.st -= 1;
    }
  }

  fn process_opcode(&mut self, opcode: u16) {
    // extract opcode parameters
    let nnn = opcode & 0x0FFF;

    // break up into nibbles
    let op_1 = (opcode & 0xF000) >> 12;
    let op_2 = (opcode & 0x0F00) >> 8;
    let op_3 = (opcode & 0x00F0) >> 4;
    let op_4 = opcode & 0x000F;

    // increment the program counter
    self.pc += 2;

    match(op_1, op_2, op_3, op_4) {
      // CLS
      (0, 0, 0xE, 0) => self.display.cls(),
      // RET
      (0, 0, 0xE, 0xE) => {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
      },
      // JP addr
      (0x1, _, _, _) => self.pc = nnn,
      // CALL addr
      (0x2, _, _, _) => {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn;
      },
    }
  }
}

fn read_word(memory: [u8; 4096], index: u16) -> u16 {
  // this is combining to 2 u8 values into 1 u16 value. Left shifted first by 8 OR unshifted second byte
  // for 00110011 and 11011101, it becomes 0011001100000000 OR 11011101 which equals 0011001111011101
  (memory[index as usize] as u16) << 8 | (memory[(index + 1) as usize] as u16)
}