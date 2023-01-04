pub struct Register {
    pub size: i8,
    pub is_available_from_sw: bool
}

pub trait RegisterActions {
    fn state(&self) -> String;
}

impl RegisterActions for Register {
    fn state(&self) -> String {
        format!("Register has {} size and it's availability from software is: {}", self.size, self.is_available_from_sw)
    }
}

pub mod SimpleRegister{
    
}
