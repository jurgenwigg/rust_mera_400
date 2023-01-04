//use rust_mera_400::{Register, RegisterActions};
extern crate rust_mera_400;

use rust_mera_400::registers::api;

fn main() {
    println!("Hello, world!");
    //let address_register = Register{size: 16, is_available_from_sw : true};
    //println!("{}", address_register.state())
    api::set_bit();
}
