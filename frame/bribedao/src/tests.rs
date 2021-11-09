use crate::vecstorage::{FastMap, BribesStorage};

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

	println!("Displaying sorted order:");
	for val in fast_vec {
		println!("{:?}", val);
	}
}
