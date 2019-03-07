use crate::display::{Display, FONT_SET};
use crate::keypad::Keypad;
use rand::Rng;

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
    let x = ((opcode & 0x0F00) >> 8) as usize;
    let y = ((opcode & 0x00F0) >> 4) as usize;
    let vx = self.v[x];
    let vy = self.v[y];
    let nnn = opcode & 0x0FFF;
    let kk = (opcode & 0x00FF) as u8;
    let n = (opcode & 0x000F) as u8;

    // break up into nibbles
    let op_1 = ((opcode & 0xF000) >> 12) as u8;
    let op_2 = ((opcode & 0x0F00) >> 8) as u8;
    let op_3 = ((opcode & 0x00F0) >> 4) as u8;
    let op_4 = (opcode & 0x000F) as u8;

    // increment the program counter
    self.pc += 2;

    // http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1
    match(op_1, op_2, op_3, op_4) {
      // 00E0 - CLS
      (0, 0, 0xE, 0) => self.display.cls(),
      // 00EE - RET
      (0, 0, 0xE, 0xE) => {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
      },
      // 1nnn - JP addr
      (0x1, _, _, _) => self.pc = nnn,
      // 2nnn - CALL addr
      (0x2, _, _, _) => {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn;
      },
      // 3xkk - SE Vx, byte
      (0x3, _, _, _) => self.pc += if vx == kk { 2 } else { 0 },
      // 4xkk - SNE Vx, byte
      (0x4, _, _, _) => self.pc += if vx != kk { 2 } else { 0 },
      // 5xy0 - SE Vx, Vy
      (0x5, _, _, 0) => self.pc += if vx == vy { 2 } else { 0 },
      // 6xkk - LD Vx, byte
      (0x6, _, _, _) => self.v[x] = kk,
      // 7xkk - ADD Vx, byte
      (0x7, _, _, _) => self.v[x] += kk,
      // 8xy0 - LD Vx, Vy
      (0x8, _, _, 0x0) => self.v[x] = vy,
      // 8xy1 - OR Vx, Vy
      (0x8, _, _, 0x1) => self.v[x] = vx | vy,
      // 8xy2 - AND Vx, Vy
      (0x8, _, _, 0x2) => self.v[x] = vx & vy,
      // 8xy3 - XOR Vx, Vy
      (0x8, _, _, 0x3) => self.v[x] = vx ^ vy,
      // 8xy4 - ADD Vx, Vy
      (0x8, _, _, 0x4) => {
        let res = vx as u16  + vy as u16;
        self.v[0xF] = if res > 0xFF { 1 } else { 0 };
        self.v[x] = (res & 0xFF) as u8;
      },
      // 8xy5 - SUB Vx, Vy
      (0x8, _, _, 0x5) => {
        self.v[0xF] = if vx > vy { 1 } else { 0 };
        self.v[x] = (vx - vy) as u8;
      },
      // 8xy6 - SHR Vx {, Vy}
      (0x8, _, _, 0x6) => {
        self.v[0xF] = vx & 0x1;
        self.v[x] >>= 1;
      },
      // 8xy7 - SUBN Vx, Vy
      (0x8, _, _, 0x7) => {
        self.v[0xF] = if vy > vx { 1 } else { 0 };
        self.v[x] = (vy - vx) as u8;
      },
      // 8xyE - SHL Vx {, Vy}
      (0x8, _, _, 0xE) => {
        self.v[0xF] = vx & 0x80;
        self.v[x] <<= 1;
      },
      // 9xy0 - SNE Vx, Vy
      (0x9, _, _, 0) => self.pc += if vx != vy { 2 } else { 0 },
      // Annn - LD I, addr
      (0xA, _, _, _) => self.i = nnn,
      // Bnnn - JP V0, addr
      (0xB, _, _, _) => self.pc = nnn + self.v[0] as u16,
      // Cxkk - RND Vx, byte
      (0xC, _, _, _) => self.v[x] = kk & random_byte(),
      // Dxyn - DRW Vx, Vy, nibble
      // TODO
      (0xD, _, _, _) => {},
      // Ex9E - SKP Vx
      (0xE, _, 0x9, 0xE) => self.pc += if self.keypad.is_key_down(vx) { 2 }  else { 0 },
      // ExA1 - SKNP Vx
      (0xE, _, 0xA, 0x1) => self.pc += if self.keypad.is_key_down(vx) { 0 }  else { 2 },
      // Fx07 - LD Vx, DT
      (0xF, _, 0, 0x7) => self.v[x] = self.dt,
      // Fx0A - LD Vx, K
      (0xF, _, 0, 0xA) => {
        self.pc -= 2; // Keep repeating current opcode
        for (i, key) in self.keypad.keys.iter().enumerate() {
          if *key == true {
            self.v[x] = i as u8;
            self.pc += 2;
          }
        }
      },
      // Fx15 - LD DT, Vx
      (0xF, _, 0x1, 0x5) => self.dt = vx,
      // Fx18 - LD ST, Vx
      (0xF, _, 0x1, 0x8) => self.st = vx,
      // Fx1E - ADD I, Vx
      (0xF, _, 0x1, 0xE) => self.i += vx as u16,
      // Fx29 - LD F, Vx
      (0xF, _, 0x2, 0x9) => self.i = vx as u16 * 5,
      // Fx33 - LD B, Vx
      (0xF, _, 0x3, 0x3) => {
        self.memory[self.i as usize] = vx / 100;
        self.memory[self.i as usize + 1] = (vx / 10) % 10;
        self.memory[self.i as usize + 2] = (vx % 100) % 10;
      },
      // Fx55 - LD [I], Vx
      (0xF, _, 0x5, 0x5) => self.memory[(self.i as usize)..(self.i + x as u16 + 1) as usize].copy_from_slice(&self.v[0..(x as usize + 1)]),
      // Fx65 - LD Vx, [I]
      (0xF, _, 0x6, 0x5) => self.v[0..(x as usize + 1)].copy_from_slice(&self.memory[(self.i as usize)..(self.i + x as u16 + 1) as usize]),
      (_, _, _, _) => ()
    }
  }
}

fn read_word(memory: [u8; 4096], index: u16) -> u16 {
  // this is combining to 2 u8 values into 1 u16 value. Left shifted first by 8 OR unshifted second byte
  // for 00110011 and 11011101, it becomes 0011001100000000 OR 11011101 which equals 0011001111011101
  (memory[index as usize] as u16) << 8 | (memory[(index + 1) as usize] as u16)
}

fn random_byte() -> u8 {
  let mut rng = rand::thread_rng();
  rng.gen()
}
