use sortedvec::sortedvec;
use std::convert::TryInto;

fn more_complex() {
    #[derive(Debug, Default, PartialEq, Hash, Clone)]
    struct BribesStorage {
        p_id: u32,
        amount: u32,
        votes: u32,
    }
    struct SortedVec<T>(Vec<T>);

    impl<T: Ord> SortedVec<T> {
        fn insert() -> bool {
            true
        } // binary search here;
    }

    sortedvec! {
            /// lookup by (amount, votes) keys
            #[derive(Debug)]
            struct ComplexMap {
                fn derive_key(val: &BribesStorage) -> (u32, u32) {
    //		(val.amount)
                    (val.amount, val.votes)
                }
    //		fn search(val: &BribeStorage) -> u32{
    //		(val.amount)
    //		}
            }
        }

    impl ComplexMap {
        fn fastsearch(&self, key: u32) -> (u32, u32, u32) {
            let myinner = &self.inner;
            let out = myinner.binary_search_by_key(&key, |n| n.amount);
            //(val.p_id, val.amount, val.votes)
            (out.unwrap().try_into().unwrap(), 2, 3)
        } // binary search here;

        //        fn position2(&self, key: &u32) -> Result<usize, usize> {
        //            self.inner
        //                .binary_search_by(|probe| Self::derive_key(probe).cmp(key))
        //        }
    }

    let mut sv = ComplexMap::default();
    for xc in vec![1, 4, 1, 2, 3, 7, 5, 8, 9] {
        let pid: u32 = rand::random();
        let xcc: u32 = xc * 100;
        sv.insert(BribesStorage {
            p_id: pid,
            amount: xc,
            votes: pid / xcc,
        });
    }

    // extra insert
    let tmp_pid: u32 = rand::random();
    let tmp_xcc: u32 = 2000;
    let tmp_xc: u32 = 2;
    let lowest_value = BribesStorage {
        p_id: tmp_pid,
        amount: tmp_xc,
        votes: tmp_xcc,
    };

    sv.insert(lowest_value.clone());

    // Searchnotes:
    // let unsorted = vec![3, 5, 0, 10, 7, 1];
    // let sorted = SortedVec::from(unsorted.clone());
    //
    // // linear search (slow!)
    // let unsorted_contains_six: Option<_> = unsorted.iter().find(|&x| *x == 6);
    // assert!(unsorted_contains_six.is_none());
    //
    // // binary search (fast!)
    // let sorted_contains_six: Option<_> = sorted.find(&6);
    // assert!(sorted_contains_six.is_none());

    //	assert_ne!(sv.iter().collect::<Vec<BribesStorage>>().binary_search_by_key(&2, |&(a.amount, a.votes, a.p_id)| a.amount),  Ok(9));
	let st = sv.fastsearch(2);
	println!("Fast search: {:?}", st);
    let tmo: Vec<BribesStorage> = sv.clone().into();
    let r = tmo.binary_search_by_key(&2, |n| n.amount);
    println!("fast search: {:?}", r);
    //	assert_eq!(tmo.binary_search_by_key(&13, | a.amount| a),  Ok(9));
    println!("finding a result based on knowing both keys");
    let xs = sv.find(&(2, 2000));
    let probe = "amount";
    let key = 2;
    //	let fluff = sv.binary_search_by(|probe| ComplexMap::derive_key(probe).cmp(key));//.position(&(2,2));
    println!("Found this: {:?}", xs);
    // Binary fast search

    //slow and ugly search
    let b = sv.iter().filter(|&s| s.amount == 2).last();
    // test it
    println!("testing result:");
    assert_ne!(b.unwrap(), &lowest_value); //highest value should always return, the value with the most votes per amount
    println!("Working value: {:?}", b);

    // {
    //        println!("found it!: ");
    //    } else {
    //        println!("Nothing there...");
    //    }

    println!("len: {}", sv.len());

    for val in sv {
        println!("{:?}", val);
    }
}

fn main() {
    println!("Starting");
    more_complex();
    println!("Done");
}
