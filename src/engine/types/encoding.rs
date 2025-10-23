/*
* I think that bincode crate might be a better option then this...
*/
pub struct Encoded {
    data: Vec<u8>,
}

pub trait Encode {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(data: Vec<u8>) -> Result<Self,()>;
}

impl Encode for u8 {
    fn to_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
    fn from_bytes(data: vec<u8>)
}

impl Encoded {
    pub fn encoded<T: Sized + Encode>(d: T) -> Self {
        Self { data: d.to_bytes() }
    }
    pub fn decode<T: Sized + Encode>(self) -> T {
        T::from_bytes(self.data)
    }
}
