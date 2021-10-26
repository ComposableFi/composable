// Sorted Vec storage

use sortedvec::sortedvec;

fn first_test() {
	#[derive(Debug, Default)]
	struct BribesStorage {
		p_id: u32,
		amount: u32,
		votes: u32,
	}

	sortedvec! {
		/// lookup by (name, prio) keys
		#[derive(Debug)]
		struct ComplexMap {
			fn derive_key(val: &BribesStorage) -> (u32, u32) {
				(val.p_id, val.votes)
			}
		}
	}

	let mut sv = ComplexMap::default();
	for xc in vec![1, 4, 1, 2, 3, 7, 5, 8, 9] {
		let pid: u32 = rand::random();
		let xcc: u32 = xc * 100;
		sv.insert(BribesStorage { p_id: pid, amount: xc, votes: pid / xcc });
	}

	//        let find = sv.find(1,3);

	println!("len: {}", sv.len());
	for val in sv {
		println!("{:?}", val);
	}
}

fn main() {
	println!("Starting");
	first_test();
	println!("Done");
}
