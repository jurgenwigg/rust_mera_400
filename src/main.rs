extern crate rust_mera_400;

// use rust_mera_400::registers::simple_register::{UniversalRegister, Register};

pub struct Cpu {
    pub reg_r0: u16,
    pub reg_r1: u16,
    pub reg_r2: u16,
    pub reg_r3: u16,
    pub reg_r4: u16,
    pub reg_r5: u16,
    pub reg_r6: u16,
    pub reg_r7: u16,
    pub reg_ac: u16, // 17bit width in real machine
    pub reg_ic: u16,
    pub reg_x: u16, //register x?
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            reg_r0: 0,
            reg_r1: 0,
            reg_r2: 0,
            reg_r3: 0,
            reg_r4: 0,
            reg_r5: 0,
            reg_r6: 0,
            reg_r7: 0,
            reg_ac: 0,
            reg_ic: 0,
            reg_x: 0,
        }
    }

    fn ld(&mut self, param_a: u16, param_b: u16) {
        self.reg_ac;
        // self.update_zero_and_negative_flags(self.reg_ac);
    }

    pub fn interpret(&mut self, program: Vec<u16>) {
        self.reg_ic = 0;

        loop {
            let opscode = program[self.reg_ic as usize];
            self.reg_ic += 1;

            match opscode {
                0o20 => {
                    self.reg_ic += 1;
                    let param_a = program[self.reg_ic as usize];
                    self.reg_ic += 1;
                    let param_b = program[self.reg_ic as usize];
                    self.ld(param_a, param_b);
                }

                // 0xA9 => {
                //     let param = program[self.reg_ic as usize];
                //     self.reg_ic += 1;

                //     self.lda(param);
                // }

                0x00 => return,
            }
        }
    }
}

fn main() {
    println!("Mera400 CPU");
    let mut cpu = Cpu::new();
    println!("Initial state:");
    println!("R3: {}", cpu.reg_r3);
    println!("R5: {}", cpu.reg_r5);
    println!("{}", cpu.reg_r3);
    cpu.reg_r3 = 16;
    println!("Before LD:");
    println!("R3: {}", cpu.reg_r3);
    println!("R5: {}", cpu.reg_r5);
    cpu.interpret(vec![0o20, 0o03, 0o05]);
    println!("After LD:");
    println!("R3: {}", cpu.reg_r3);
    println!("R5: {}", cpu.reg_r5);

}
