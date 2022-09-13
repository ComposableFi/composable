use super::*;
use crate::{AccountIdOf, Pallet as Cosmwasm};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_system::{Pallet as System, RawOrigin};

benchmarks! {
	upload {
		let origin = account::<<T as Config>::AccountIdExtended>("signer", 0, 0xCAFEBABE);
		let code = include_bytes!("../../../../../target/wasm32-unknown-unknown/cosmwasm-contracts/xcvm_router.wasm").to_vec().try_into().unwrap();
	}: _(RawOrigin::Signed(origin), code)
}

impl_benchmark_test_suite!(Cosmwasm, crate::mock::new_test_ext(), crate::mock::Test,);
