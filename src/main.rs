extern crate rust_mera_400;

use rust_mera_400::registers::simple_register::{UniversalRegister, Register};


fn main() {
    let example_register:UniversalRegister = Register::new(3);
    println!("{}",example_register.len())
}
