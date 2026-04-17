use crate::common::enum_tools::Unwrap;
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Debug, Serialize, Deserialize)]
pub enum Floats {
    F32(f32),
    F64(f64),
}

impl Unwrap for Floats {
    fn unwrap<T: 'static>(&self) -> T {
        let n: Box<dyn Any> = match *self {
            Floats::F32(n) => Box::new(n),
            Floats::F64(n) => Box::new(n),
        };

        return *n.downcast::<T>().unwrap();
    }
}
