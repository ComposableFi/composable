#![cfg_attr(not(test), deny(clippy::disallowed_methods, clippy::disallowed_types,))]
extern crate alloc;

pub use xc_core::gateway as msg;

pub mod assets;
pub mod auth;
pub mod contract;
pub mod error;
mod events;
pub mod state;
