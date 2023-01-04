trait Register {
    fn new(name:&'static str, size:i8, available_from_software:bool);
}

struct AddressRegister {is_available_for_sw:bool, size:i8}

// impl Register for AddressRegister #
