use super::*;

use crate::Pallet as Vault;
use codec::{Decode, Encode, MaxEncodedLen};
use composable_support::validation::Validated;
use composable_traits::vault::{CapabilityVault, Deposit, Vault as VaultTrait, VaultConfig};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	assert_ok,
	traits::{fungible::Mutate as FungibleMutate, fungibles::Mutate as FungiblesMutate, Get},
};
use frame_system::{EventRecord, Pallet as System, RawOrigin};
use sp_runtime::Perquintill;
use sp_std::prelude::*;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

const DEFAULT_STRATEGY_SHARE: Perquintill = Perquintill::from_percent(90);
const DEFAULT_RESERVE: Perquintill = Perquintill::from_percent(10);

// NOTE(hussein-aitlahcen): postfix underscore on `deposit_` to avoid clashing with benchmarking
// macro
fn create_vault_extended<T: Config>(
	asset_id: u128,
	strategy_account_id: T::AccountId,
	strategy_share: Perquintill,
	reserved: Perquintill,
	deposit_: Deposit<BalanceOf<T>, BlockNumberOf<T>>,
) -> (T::VaultId, VaultInfo<T>) {
	let config = VaultConfig {
		asset_id: recode_unwrap_u128(asset_id),
		manager: whitelisted_caller(),
		reserved,
		strategies: [(strategy_account_id, strategy_share)].iter().cloned().collect(),
	};
	let v = Vault::<T>::do_create_vault(deposit_, Validated::new(config).unwrap());
	assert_ok!(&v);
	v.expect("unreachable; qed;")
}

fn create_vault<T: Config>(
	asset_id: u128,
	strategy_account_id: T::AccountId,
) -> (T::VaultId, VaultInfo<T>) {
	create_vault_extended::<T>(
		asset_id,
		strategy_account_id,
		DEFAULT_STRATEGY_SHARE,
		DEFAULT_RESERVE,
		Deposit::Existential,
	)
}

const A: u128 = 2;
// if to make it generic, and pass u128, it will pass HasCompact, and u128 will be 5 bits, not 16...
pub fn recode_unwrap_u128<
	O: Decode + MaxEncodedLen + Encode,
	I: Decode + MaxEncodedLen + Encode,
>(
	raw: I,
) -> O {
	// next does not holds, because in wasm it is 16 and 8, in native 16 and 5. But that works fine
	// overall assert_eq!(I::max_encoded_len(), O::max_encoded_len(), "<I as
	// MaxEncodedLen>::max_encoded_len() must be equal <O as MaxEncodedLen>::max_encoded_len()");
	O::decode(&mut &raw.encode()[..]).unwrap()
}

benchmarks! {
	create {
		let caller: T::AccountId = whitelisted_caller();
		let asset_id = recode_unwrap_u128(A);
		let reserved = Perquintill::from_percent(100);
		let manager = whitelisted_caller();
		let strategies = Default::default();
		let amount = T::CreationDeposit::get() * 10u32.into();
		T::Currency::mint_into(recode_unwrap_u128(A), &caller, amount * 2u32.into())?;
		T::NativeCurrency::mint_into(&caller, amount * 2u32.into())?;
		let vault_id = recode_unwrap_u128(1u128);
	}: _(
		RawOrigin::Signed(caller.clone()),
		VaultConfig {
			asset_id,
			reserved,
			manager,
			strategies
		},
		amount
	)
	verify {
		assert_last_event::<T>(Event::VaultCreated {
			id: vault_id
		}.into())
	}

	deposit {
		let caller: T::AccountId = whitelisted_caller();
		let amount = T::CreationDeposit::get() * 10u32.into();
		let (vault, _) = create_vault::<T>(A, whitelisted_caller());
		T::Currency::mint_into(recode_unwrap_u128(A), &caller, amount * 2u32.into())?;
		T::NativeCurrency::mint_into(&caller, amount * 2u32.into())?;
	}: _(RawOrigin::Signed(caller.clone()), vault, amount)
	verify {
		assert_last_event::<T>(Event::Deposited {
			account: caller,
			asset_amount: amount,
			lp_amount: amount
		}.into())
	}

	withdraw {
		let caller: T::AccountId = whitelisted_caller();
		let amount = T::CreationDeposit::get() * 10u32.into();
		let (vault, _) = create_vault::<T>(A, caller.clone());
		T::Currency::mint_into(recode_unwrap_u128(A), &caller, amount * 2u32.into())?;
		T::NativeCurrency::mint_into(&caller, amount * 2u32.into())?;
		<Vault<T> as VaultTrait>::deposit(&vault, &caller, amount)?;
	}: _(RawOrigin::Signed(caller.clone()), vault, amount)
	verify {
		assert_last_event::<T>(Event::Withdrawn {
			account: caller,
			asset_amount: amount,
			lp_amount: amount
		}.into())
	}

	emergency_shutdown {
		let caller: T::AccountId = whitelisted_caller();
		let (vault, _) = create_vault::<T>(A, caller);
	}: _(RawOrigin::Root, vault)
	verify {
		assert_last_event::<T>(Event::EmergencyShutdown {
			vault
		}.into())
	}

	// NOTE(hussein-aitlahcen): underscore postfix to avoid clashing with the `start` function of the benchmarking macro.
	start_ {
		let caller: T::AccountId = whitelisted_caller();
		let (vault, _) = create_vault::<T>(A, caller);
		<Vault<T> as CapabilityVault>::stop(&vault)?;
	}: start(RawOrigin::Root, vault)
	verify {
		assert_last_event::<T>(Event::VaultStarted {
			vault
		}.into())
	}

	add_surcharge {
		let caller: T::AccountId = whitelisted_caller();
		let amount = T::CreationDeposit::get() * 10u32.into();
		let add_amount = Validated::new(T::CreationDeposit::get()).unwrap();
		let block = System::<T>::block_number();
		let deposit_ = Deposit::Rent { amount, at: block };
		T::Currency::mint_into(recode_unwrap_u128(A), &caller, amount * 2u32.into())?;
		T::NativeCurrency::mint_into(&caller, amount * 2u32.into())?;
		let (vault, _) = create_vault_extended::<T>(A, caller.clone(), DEFAULT_STRATEGY_SHARE, DEFAULT_RESERVE, deposit_);
		System::<T>::set_block_number(10_000_000u32.into());
	}: _(RawOrigin::Signed(caller), vault, add_amount)

	claim_surcharge {
		let vault = recode_unwrap_u128(1u128);
		let caller: T::AccountId = whitelisted_caller();
		let asset_id = recode_unwrap_u128(A);
		let reserved = Perquintill::from_percent(100);
		let strategies = Default::default();
		let amount = T::CreationDeposit::get() * 10u32.into();
		let block = System::<T>::block_number();
		T::Currency::mint_into(recode_unwrap_u128(A), &caller, amount * 2u32.into())?;
		T::NativeCurrency::mint_into(&caller, amount * 2u32.into())?;
		Vault::<T>::create(
			RawOrigin::Signed(caller.clone()).into(),
			VaultConfig {
				asset_id,
				reserved,
				manager: caller.clone(),
				strategies
			},
			amount
		)?;
		System::<T>::set_block_number(10_000_000u32.into());
	}: _(RawOrigin::Signed(caller), vault, None)

	delete_tombstoned {
		let caller: T::AccountId = whitelisted_caller();
		let amount = T::CreationDeposit::get() * 10u32.into();
		let block = System::<T>::block_number();
		let deposit_ = Deposit::Rent { amount, at: block };
		T::Currency::mint_into(recode_unwrap_u128(A), &caller, amount * 2u32.into())?;
		T::NativeCurrency::mint_into(&caller, amount * 2u32.into())?;
		let (vault, _) = create_vault_extended::<T>(A, caller.clone(), DEFAULT_STRATEGY_SHARE, DEFAULT_RESERVE, deposit_);
		System::<T>::set_block_number(10_000_000u32.into());
		Vault::<T>::claim_surcharge(RawOrigin::Signed(caller.clone()).into(), vault, None).expect("goo");
		// Wait until the vault is deletable.
		System::<T>::set_block_number(System::<T>::block_number() + T::TombstoneDuration::get());
	}: _(RawOrigin::Signed(caller), vault, None)
}

impl_benchmark_test_suite!(
	Vault,
	crate::mocks::tests::ExtBuilder::default().build(),
	crate::mocks::tests::Test,
);
