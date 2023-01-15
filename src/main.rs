extern crate rust_mera_400;
use rust_mera_400::registers::universal::universal_register;

fn main() {
    // packet_data: &mut [u8]
    let mut first_register = universal_register::View::new(0b0001000010000100_u16.to_be_bytes());
    println!("{:b}", first_register.data().read());
    let reg_value = first_register.data().read();
    first_register.data_mut().write(reg_value+0b0110);
    println!("{:b}", first_register.data().read());
}
