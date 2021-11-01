
# How to add benchmarking for a pallet

In order to set the correct Weights(the execution time of function/transaction fee) on our functions, we need to test our functions so we can
get a good estimate of how much computing power is needed and how long the process will take, so that we can put the correct numbers.
This is where benchmarking comes in, a way to dry-run our functions without affecting the chain directly.

There are 3 major parts we need in order to implement benchmarking:

*  Create a weight file

*  Write benchmark tests

*  Add the benchmark to the runtime


Let's try adding benchmarking to the scheduler pallet to picasso's runtime.


## Configuring Weights
All of substrate's own pallets weights.rs files can be automatically generated using the WeightInfo trait.

Read more here:
https://substrate.dev/docs/en/knowledgebase/runtime/benchmarking#auto-generated-weightinfo-implementation


Find the pallets weight file, add it to the weights folder and
include it in the mod.rs file.
```bash
$ ls runtime/picasso/src/weights/
frame_system.rs
indices.rs
membership.rs
mod.rs
```
In our case, we want to add the scheduler pallets weight file, which
we can easily find in the [pallets repository](https://github.com/paritytech/substrate/tree/master/frame/scheduler).

Let's download the weights.rs file and place it into our weights folder and name it after the pallet:
```
$ curl "https://raw.githubusercontent.com/paritytech/substrate/v3.0.0/frame/scheduler/src/weights.rs" -o composable/runtime/picasso/src/weights/scheduler.rs
```


Lets make the file more lightweight by change WeightInfo into a struct instead of the default trait and rename SubstrateWeight to WeightInfo:

```rust
47 pub trait WeightInfo {               // remove
48         fn schedule(s: u32, ) -> Weight; // remove
49         fn cancel(s: u32, ) -> Weight;    // remove
50         fn schedule_named(s: u32, ) -> Weight;  // remove
51         fn cancel_named(s: u32, ) -> Weight;   // remove
52 }   // remove
53
55 pub struct SubstrateWeight<T>(PhantomData<T>);  // Rename SubstrateWeight to WeightInfo everywhere
56 impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> { //same

...

// lets also remove this
95 impl WeightInfo for () {
96         // Storage: Scheduler Agenda (r:1 w:1)
97         fn schedule(s: u32, ) -> Weight {
98                 (24_730_000 as Weight)

```

What we are left with is a more lightweight file:

```rust
//  cat scheduler.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> scheduler::WeightInfo for WeightInfo<T> {
	fn schedule(s: u32, ) -> Weight {
		(43_832_000 as Weight) // These weights will change depending on the results from the benchmarking
			// Standard Error: 181_000
			.saturating_add((7_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn cancel(s: u32, ) -> Weight {
		(33_112_000 as Weight)
			// Standard Error: 234_000
			.saturating_add((4_865_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn schedule_named(s: u32, ) -> Weight {
		(45_294_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((221_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn cancel_named(s: u32, ) -> Weight {
		(38_311_000 as Weight)
			// Standard Error: 95_000
			.saturating_add((4_575_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

```

Append our new weight file to mod.rs
```bash
echo "pub mod scheduler;" >> mod.rs
```
and now we can simply call our weight file with:
*weights::scheduler*

And use our WeightInfo functionality in the code base by defining a variable just like this:
```rust

impl scheduler::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type PalletsOrigin = OriginCaller;
	type Call = Call;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type WeightInfo = weights::scheduler::WeightInfo<Runtime>;     // updated
}
```



## Adding benchmarking to the runtime
So now when we can easily include and use our weight file, we can move forward and add it to our runtime.
In order to add a pallets benchmarking functions to a runtime we first
need to go into the directory folder:

```bash
cd runtime/picasso/
```

Enable the pallet's benchmarking features in Cargo.toml:

```toml
...
runtime-benchmarks = [
        "benchmarking",
        "frame-support/runtime-benchmarks",
        "frame-system-benchmarking",
        "frame-system/runtime-benchmarks",
        "scheduler/runtime-benchmarks",
        "collective/runtime-benchmarks",
        "democracy/runtime-benchmarks",
]

```

In our runtime we need to tell the benchmark library to execute our benchmarks, so we can easily use it.
This is done with the [dispatch_benchmark](https://github.com/paritytech/substrate/blob/polkadot-v0.9.8/frame/benchmarking/src/utils.rs#L93) function. All benchmarks
we want to enable needs to be added to the Vec list we are creating in this function.
So simply add it with the [add_benchmark](https://docs.rs/frame-benchmarking/3.0.0/frame_benchmarking/macro.add_benchmark.html) macro function:

```rust
...
add_benchmark!(params, batches, <>)
...
```

Read more about the benchmarking library here:
https://github.com/paritytech/substrate/blob/master/frame/benchmarking/src/lib.rs#L1325
https://crates.io/crates/frame-benchmarking

Extra:
If we are running the latest version of the frame-benchmarking dependency we can also add our pallet to the [benchmark_metadata](https://github.com/paritytech/substrate/blob/polkadot-v0.9.12/frame/benchmarking/src/utils.rs#L150) function.



## Compile and run
Now when we have everything we need, we want to build it and enable
the runtime-benchmarks features, which are optional.
```shell
$ cargo build --release --features runtime-benchmarks
```

## Run the benchmarks

```shell
$ ./target/release/composable benchmark --chain=picasso-dev --execution=wasm --wasm-execution=compiled --pallet=scheduler --extrinsic='*' --steps=10 --repeat=5 --raw --output=./runtime/picasso/src/weights

```

Tips:
In the script folder you can find the benchmarks.sh, use that to easily
run benchmarks on all pallets synchronously.



### Read more:
https://github.com/shawntabrizi/substrate-benchmark-genesis
https://github.com/paritytech/substrate/blob/master/frame/benchmarking/src/lib.rs
https://www.shawntabrizi.com/substrate-graph-benchmarks/docs/#/
https://substrate.dev/docs/en/knowledgebase/runtime/benchmarking
https://crates.io/crates/frame-benchmarking
https://github.com/paritytech/substrate/tree/polkadot-v0.9.8/frame/benchmarking

