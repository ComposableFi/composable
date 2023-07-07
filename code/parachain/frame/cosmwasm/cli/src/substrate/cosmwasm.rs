use crate::{
	args::{parse_funds, StoreCommand},
	error::Error,
};
use std::{fs, io::Read, path::PathBuf};
use subxt::utils::AccountId32;

pub fn fetch_code(this: &StoreCommand) -> Result<Vec<u8>, Error> {
	let mut file = fs::File::open(&this.wasm_file)?;
	let metadata = fs::metadata(&this.wasm_file)?;
	let mut buffer = vec![0u8; metadata.len() as usize];
	file.read_exact(&mut buffer)?;
	Ok(buffer)
}
