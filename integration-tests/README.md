# Overview

Runs transfers from some Composable based parachain to Composable parachain. And other parachains integrations.


## Building issue

During benchmarks build next issue happens

```
error[E0046]: not all trait items implemented, missing: `successful_origin`
    --> /home/dz/.cargo/git/checkouts/substrate-7e08433d4c370a21/d76f399/frame/society/src/lib.rs:1264:1
     |
1264 | impl<T: Config> EnsureOrigin<T::Origin> for EnsureFounder<T> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `successful_origin` in implementation
     |
     = help: implement the missing item: `fn successful_origin() -> OuterOrigin { todo!() }`
```

But code exists in repo of that version https://github.com/paritytech/substrate/blob/d76f39995315ec36980908e4b99709bd14927044/frame/society/src/lib.rs#L1273

Fixes like

`![cfg(not(feature = "runtime-benchmarks"))]` or

```
runtime-benchmarks = [
	"pallet-society",
]
```

did no help.

So please exclude this crate from `runtime-benchmarks` enabled runs