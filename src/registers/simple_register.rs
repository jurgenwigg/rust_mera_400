pub struct UniversalRegister { register:Vec<bool> }

pub trait Register {
    fn new(size: usize) -> Self;
}
impl UniversalRegister {
    pub fn len(&mut self) -> usize{
        self.register.len()
    }

    pub fn set_bit(&mut self, bit_position:usize, value:bool) {
        self.register[bit_position] = value;
    }

    pub fn state(&mut self) -> String {
        let reg = &self.register;
        let temp = reg.into_iter().map(|elem| *elem as i8).into_iter();
        String::from_iter(temp.map(|elem| elem.to_string()))
        //self.register.iter().map(|elem| elem.to_int()).
   }
}
impl Register for UniversalRegister {
    fn new(size:usize) -> UniversalRegister {
        UniversalRegister{ register : vec![false;size] }
    }
}
