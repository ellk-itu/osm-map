use num_traits::{Num, NumCast, ToBytes};
use serde::{Deserialize, Serialize};

///Two dimentinal tuple containing a number
#[derive(Debug, Serialize, Deserialize)]
pub struct Point<T: Num + ToBytes>(pub(crate) T, pub(crate) T);

impl<T: Num + ToBytes> Point<T> {
    pub fn to_le_bytes(&self) -> [T::Bytes; 2] {
        [self.0.to_le_bytes(), self.1.to_le_bytes()]
    }

    pub fn to_be_bytes(&self) -> [T::Bytes; 2] {
        [self.0.to_be_bytes(), self.1.to_be_bytes()]
    }
}
