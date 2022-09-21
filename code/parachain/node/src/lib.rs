#![cfg_attr(not(test), warn(clippy::disallowed_methods, clippy::indexing_slicing))] // allow in tests
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![recursion_limit = "1024"]

pub mod chain_spec;
pub mod cli;
mod client;
pub mod command;
pub mod rpc;
pub mod runtime;
pub mod service;
