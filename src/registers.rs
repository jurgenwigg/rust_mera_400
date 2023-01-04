trait Register {
    fn new(name:&'static str, size:i8, available_from_software:bool);
    fn set_bit(bit_position:i8);
}

struct AddressRegister {is_available_for_sw:bool, size:i8}

// impl Register for AddressRegister #
