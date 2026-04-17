use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::common::enum_tools::Unwrap;

#[derive(Debug, Serialize, Deserialize)]
pub enum UInts {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

impl Unwrap for UInts {
    fn unwrap<T: 'static>(&self) -> T {
        let n: Box<dyn Any> = match *self {
            UInts::U8(n) => Box::new(n),
            UInts::U16(n) => Box::new(n),
            UInts::U32(n) => Box::new(n),
            UInts::U64(n) => Box::new(n),
            UInts::U128(n) => Box::new(n),
        };

        return *n.downcast::<T>().unwrap();
    }
}
