extern crate rust_mera_400;

// use rust_mera_400::registers::simple_register::{UniversalRegister, Register};

pub struct Cpu {
    pub reg_r0:u16,
    pub reg_ac:u16, // 17bit width in real machine
    pub reg_ic:u16,
    pub reg_x:u16 //register x?
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            reg_r0: 0,
            reg_ac: 0,
            reg_ic: 0,
            reg_x:0
        }
    }

    fn lda(&mut self, value: u16) {
        self.reg_ac = value;
        self.update_zero_and_negative_flags(self.reg_ac);
    }
  
    fn tax(&mut self) {
        self.reg_x = self.reg_ac;
        self.update_zero_and_negative_flags(self.reg_x);
    }
   
     fn update_zero_and_negative_flags(&mut self, result: u16) {
         if result == 0 {
             self.reg_r0 = self.reg_r0 | 0b0000_0010;
         } else {
             self.reg_r0 = self.reg_r0 & 0b1111_1101;
         }
 
         if result & 0b1000_0000 != 0 {
             self.reg_r0 = self.reg_r0 | 0b1000_0000;
         } else {
             self.reg_r0 = self.reg_r0 & 0b0111_1111;
         }
     }

    pub fn interpret(&mut self, program: Vec<u16>) {
        self.reg_ic = 0;
    
        loop {
            let opscode = program[self.reg_ic as usize];
            self.reg_ic += 1;
    
            match opscode {
                0xA9 => {
                    let param = program[self.reg_ic as usize];
                    self.reg_ic += 1;
                    
                    self.lda(param);
                }
    
                0xAA => self.tax(),
    
                0x00 => return,
                
                _ => todo!(),

            }
        }
    }
 }

fn main() {
    println!("Mera400 CPU");
}
