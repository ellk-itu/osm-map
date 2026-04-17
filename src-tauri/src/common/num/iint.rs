use crate::common::enum_tools::Unwrap;
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Debug, Serialize, Deserialize)]
pub enum IInts {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
}

impl Unwrap for IInts {
    fn unwrap<T: 'static>(&self) -> T {
        let n: Box<dyn Any> = match *self {
            IInts::I8(n) => Box::new(n),
            IInts::I16(n) => Box::new(n),
            IInts::I32(n) => Box::new(n),
            IInts::I64(n) => Box::new(n),
            IInts::I128(n) => Box::new(n),
        };

        return *n.downcast::<T>().unwrap();
    }
}
