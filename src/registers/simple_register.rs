pub struct UniversalRegister { register:usize }

pub trait Register {
    // fn new(size: usize) -> Self;
    fn new() -> Self;
    // fn set_bit(&mut self, bit_position:usize, value:bool);
    // fn state(&self) -> String;
    // fn size(&self) -> usize;
}

impl Register for UniversalRegister {
    // fn new(size:usize) -> UniversalRegister {
    fn new() -> UniversalRegister {
        UniversalRegister{ register : 0 }
    }
    // fn size(&self) -> usize{
    //     self.register.
    // }

    // fn set_bit(&mut self, bit_position:usize, value:bool) {
    //     self.register[bit_position] = value;
    // }

    // fn state(&self) -> String {
    //     let reg = &self.register;
    //     let temp = reg.into_iter().map(|elem| *elem as i8).into_iter();
    //     String::from_iter(temp.map(|elem| elem.to_string()))
    // }
}

impl From<u16> for UniversalRegister {
    fn from(item: u16) -> Self {
        UniversalRegister { register: usize::from(item) }
    }
}

impl Into<u16> for UniversalRegister {
    fn into(self) -> u16 {
        let new_value = self.register.to_owned() as u16;
        new_value
    }
}
