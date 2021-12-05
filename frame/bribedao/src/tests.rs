#![cfg(test)]

use crate::sortedvec::{BribesStorage, FastMap};

/// Test the Fast Vec functions and make sure they return a sorted result
#[test]
fn test_fastvec() {
	let mut fast_vec = FastMap::new();
	// insert a bunch of test values
	let myvec = vec![1, 4, 2, 3, 7, 5, 8, 9];
	for xc in myvec.iter() {
		let pid: u32 = rand::random(); // make all entries unqiue with a random u32 id
		let num: u32 = *xc;
		let xnum: u32 = num * 100;
		fast_vec.add(num, pid, pid / xnum);
	}

	// Lets manually insert a low value
	let lowest_value = BribesStorage { p_id: rand::random(), amount: 2, votes: 5 };
	let lv = lowest_value.clone();
	fast_vec.add(lv.amount, lv.p_id, lv.votes);

	let testit: FastMap = fast_vec.clone();

	let awn = testit.find_amount(2);
	assert_ne!(&awn, &lowest_value); // Check that the function does not return the lowest value in the list

	println!("Displaying sorted order:");
	for val in fast_vec {
		println!("{:?}", val);
	}
}
