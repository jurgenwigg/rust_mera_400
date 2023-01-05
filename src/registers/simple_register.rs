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
   }
}
impl Register for UniversalRegister {
    fn new(size:usize) -> UniversalRegister {
        UniversalRegister{ register : vec![false;size] }
    }
}

#[cfg(test)]
mod tests {
    use super::{UniversalRegister,Register};
    
    #[test_case(UniversalRegister::new())]
    fn trait_tester(c: impl Register) {
        assert!(0==0);
    }
    // use super::{Bar, Calculator, Foo};
    // use test_case::test_case;

    // #[test_case(Foo::new())]
    // #[test_case(Bar::new())]
    // fn trait_tester(c: impl Calculator) {
    //     assert_eq!(c.add(2, 3), 5);
    //     assert_eq!(c.add(10, 43), 53);
    // }

}
