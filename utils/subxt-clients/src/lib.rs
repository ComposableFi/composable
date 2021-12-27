//! Generated subxt clients.
#![allow(clippy::all)]

pub mod chachacha;
pub mod picasso;

#[subxt::subxt(runtime_metadata_path = "src/metadata/picasso_develop.scale")]
pub mod picasso_develop {}
