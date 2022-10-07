use super::*;
use crate::{AccountIdOf, Pallet as Cosmwasm};
use cosmwasm_vm_wasmi::code_gen::{self, WasmModule};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::traits::Get;
use frame_system::{Pallet as System, RawOrigin};
use sp_runtime::traits::Convert;

benchmarks! {
	where_clause { where T::Balance: From<u128>, T::AssetId: From<u128> }
	upload {
		let n in 0 .. T::MaxCodeSize::get() - 3000;
		let origin = account::<<T as Config>::AccountIdExtended>("signer", 0, 0xCAFEBABE);
		let asset = T::AssetToDenom::convert(alloc::string::String::from("1")).unwrap();
		let wasm_module: WasmModule = code_gen::ModuleDefinition::new(n as usize).unwrap().into();
	}: _(RawOrigin::Signed(origin), wasm_module.code.try_into().unwrap())
}

impl_benchmark_test_suite!(Cosmwasm, crate::mock::new_test_ext(), crate::mock::Test,);
