//! just make all math/containers/number at hand. for now i concern to be as close as possible to the original python code in simplicity.
pub use itertools::*;
pub use rand::distributions::Standard;
pub use rand::prelude::*;
pub use rand::Rng;
pub use rand_distr::{Distribution, Normal};
pub use rust_decimal::Decimal;
pub use rust_decimal_macros::dec;
pub use std::cmp::max;
pub use std::cmp::min;
pub use std::cmp::Ordering;
pub use std::collections::HashMap;
pub use std::fmt::format;
pub use std::fmt::Debug;
pub use std::str::FromStr;
pub use tuples::*;