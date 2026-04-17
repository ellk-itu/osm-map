use num_traits::Num;
use serde::{Deserialize, Serialize};

///Two dimentinal tuple containing a number
#[derive(Debug, Serialize, Deserialize)]
pub struct Point<T: Num>(pub(crate) T, pub(crate) T);
