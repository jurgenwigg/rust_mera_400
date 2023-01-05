extern crate rust_mera_400;

use rust_mera_400::registers::simple_register::{UniversalRegister, Register};

struct CpuRegisters {
    r0:UniversalRegister,
    r1:UniversalRegister,
    r2:UniversalRegister,
    r3:UniversalRegister,
    r4:UniversalRegister,
    r5:UniversalRegister,
    r6:UniversalRegister,
    r7:UniversalRegister,
}


fn main() {
    let mut cpu= CpuRegisters{
        r0:Register::new(16),
        r1:Register::new(16),
        r2:Register::new(16),
        r3:Register::new(16),
        r4:Register::new(16),
        r5:Register::new(16),
        r6:Register::new(16),
        r7:Register::new(16),
    };
    let mut example_register:UniversalRegister = Register::new(8);
    println!("{}",example_register.state());
    example_register.set_bit(0,true);
    println!("{}",example_register.state());
    cpu.r0.set_bit(13,true);
    cpu.r1.set_bit(12,true);
    println!("{}",cpu.r0.state());
    println!("{}",cpu.r1.state());
}
