#![cfg_attr(not(test), deny(clippy::disallowed_methods, clippy::disallowed_types,))]
extern crate alloc;

pub mod authenticate;
pub mod contract;
pub mod error;
pub mod events;
pub mod msg;
mod prelude;
pub mod state;
