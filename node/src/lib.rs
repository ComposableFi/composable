#![cfg_attr(not(test), warn(clippy::disallowed_method))] // allow in tests

pub mod chain_spec;
mod client;
pub mod rpc;
pub mod runtime;
pub mod service;
