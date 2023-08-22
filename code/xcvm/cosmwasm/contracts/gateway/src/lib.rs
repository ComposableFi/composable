#![cfg_attr(not(test), deny(clippy::disallowed_methods, clippy::disallowed_types, unused_imports))]
extern crate alloc;

pub use xc_core::gateway as msg;

pub mod assets;
pub mod auth;
pub mod batch;
pub mod contract;
pub mod error;
mod events;
pub mod exchange;
pub mod interpreter;
mod network;
mod prelude;
pub mod state;
