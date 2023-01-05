extern crate rust_mera_400;

use rust_mera_400::registers::simple_register::{UniversalRegister, Register};


fn main() {
    let mut example_register:UniversalRegister = Register::new(8);
    println!("{}",example_register.state());
    example_register.set_bit(0,true);
    println!("{}",example_register.state());
    example_register.set_bit(20,true);
    println!("{}",example_register.state());
}
