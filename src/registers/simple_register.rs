pub struct UniversalRegister { register:Vec<bool> }

pub trait Register {
    fn new(size: usize) -> Self;
    fn set_bit(&mut self, bit_position:usize, value:bool);
    fn state(&self) -> String;
    fn size(&self) -> usize;
}

impl Register for UniversalRegister {
    fn new(size:usize) -> UniversalRegister {
        UniversalRegister{ register : vec![false;size] }
    }
    fn size(&self) -> usize{
        self.register.len()
    }

    fn set_bit(&mut self, bit_position:usize, value:bool) {
        self.register[bit_position] = value;
    }

    fn state(&self) -> String {
        let reg = &self.register;
        let temp = reg.into_iter().map(|elem| *elem as i8).into_iter();
        String::from_iter(temp.map(|elem| elem.to_string()))
   }
}

impl From<usize> for UniversalRegister {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
