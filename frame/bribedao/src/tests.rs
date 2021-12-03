#![cfg(test)]

use crate::sortedvec::{BribesStorage, FastMap};
use composable_traits::{
	bribe::{Bribe, CreateBribeRequest},
	democracy::Democracy,
};
use pallet_democracy::{ReferendumIndex, Vote};

#[test]
fn bribe_request() {
	let mut FastVec: FastMap = FastMap::new();
	let ref_index: u32 = 20;
	let amount: u32 = 100;
	let votes: u32 = 3;

	/*
		let request = CreateBribeRequest{
		account_id: 2,
		ref_index: 2,
		total_reward: 3,
		asset_id: 3,
		requested_votes: 3,
		is_aye: false,
	};
	*/
	let id = 333;

	//BribeRequests::<T>::insert(id, request);

	FastVec::add(amount, ref_index, votes);

	FastVec::find_all_pid(ref_index);

	FastVec::remove_bribe(amount, ref_index, votes);
}

/// Test the Fast Vec functions and make sure they return a sorted result
#[test]
fn test_fastvec() {
	let mut fast_vec = FastMap::new();
	// insert a bunch of test values
	for xc in vec![1, 4, 2, 3, 7, 5, 8, 9] {
		let pid: u32 = rand::random(); // make all entries unqiue with a random u32 id
		fast_vec.add(pid, xc, pid / xc * 100);
	}

	// Lets manually insert a low value
	let lowest_value = BribesStorage { p_id: rand::random(), amount: 2, votes: 2000 };
	let lv = lowest_value.clone();
	fast_vec.add(lv.p_id, lv.amount, lv.votes);
	// testing getting the results
	let b = fast_vec.iter().filter(|&s| s.amount == 2).last();
	// if this test works, then it should not return the lower amount that we manually inserted, and
	// should instead return the highest amount
	assert_ne!(b.unwrap(), &lowest_value);

	println!("Displaying sorted order:");
	for val in fast_vec {
		println!("{:?}", val);
	}
}
