pub struct UniversalRegister { register:Vec<bool> }

pub trait Register {
    fn new(size: usize) -> Self;
    // fn new(size:usize) -> UniversalRegister{
    //     UniversalRegister{ register : vec![false;size]}
    // }
}
impl UniversalRegister {
    pub fn len(&self) -> usize{
        self.register.len()
    }
}
impl Register for UniversalRegister {
    fn new(size:usize) -> UniversalRegister {
        UniversalRegister{ register : vec![false;size] }
    }
}
