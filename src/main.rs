extern crate rust_mera_400;

use rust_mera_400::registers::simple_register::{UniversalRegister, Register};

pub struct Cpu {
    pub reg_ac:UniversalRegister,
    pub reg_r0:UniversalRegister,
    pub reg_ic:UniversalRegister
}

impl Cpu {
    fn new()-> Self {
        Cpu {
            reg_ac: Register::new(), // accumulator register
            reg_r0: Register::new(), // R0 register
            reg_ic: Register::new()  // IC register
        }
    }

    pub fn interpret(&mut self, program: Vec<u16>) {
        loop {
            let opscode = program[self.reg_ic];
            self.reg_ic += 1;

            match opscode {
                _ => todo!()
            }
        }
    }
}

fn main() {
}
