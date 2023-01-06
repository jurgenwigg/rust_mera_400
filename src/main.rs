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
            reg_ac: Register::new(17), // accumalor register
            reg_r0: Register::new(16), // R0 register
            reg_ic: Register::new(16)  // IC register
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        loop {
            let opscode = program[self.reg_ic as usize];
            self.reg_ic += 1;

            match opscode {
                _ => todo!()
            }
        }
    }
}

fn main() {

}
