
pub enum Size{
    Fixed(u32),
    Relative(u32) // relative to the screen as a whole
}

impl Size {
    pub fn new_fixed(val: u32) -> Self {
        Self::Fixed(val)
    }

    pub fn new_relative(val: u32) -> Self {
        Self::Relative(val)
    }

    pub fn is_relative(&self) -> bool {
        match self {
            Self::Relative(_) => true,
            _ => false
        }
    }

    pub fn is_fixed(&self ) -> bool {
        match self {
            Self::Fixed(_) => true,
            _ => false,
        }
    }

    pub fn abs(&self) -> u32 {
        match self {
            Self::Fixed(val) => *val,
            Self::Relative(val) => *val
        }
    }
}