#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate common;
extern crate pallet_cosmwasm;

fuzz_target!(|data: &[u8]| {
	// fuzzed code goes here
});
