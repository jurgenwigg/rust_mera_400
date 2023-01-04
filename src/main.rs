use rust_mera_400::{Register, RegisterActions};

fn main() {
    println!("Hello, world!");
    let address_register = Register{size: 16, is_available_from_sw : true};
    println!("{}", address_register.state())
}
