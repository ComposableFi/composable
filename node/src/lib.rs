#![allow(unknown_lints, panics)]
#![cfg_attr(not(test), warn(clippy::disallowed_method, clippy::indexing_slicing))] // allow in tests
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_type)]
pub mod chain_spec;
pub mod cli;
mod client;
pub mod command;
pub mod rpc;
pub mod runtime;
pub mod service;
