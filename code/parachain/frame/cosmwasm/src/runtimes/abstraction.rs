use crate::{types::AccountIdOf, Config, Pallet};
use alloc::{string::String, vec::Vec};
use codec::Encode;
use core::marker::PhantomData;
use cosmwasm_std::{Addr, CanonicalAddr};
use cosmwasm_vm::vm::VmGasCheckpoint;
use scale_info::TypeInfo;

pub trait VMPallet {
	type VmError;
}

#[derive(Clone, Debug)]
pub struct CanonicalCosmwasmAccount<T: Config>(pub CosmwasmAccount<T>);

impl<T: Config> From<CosmwasmAccount<T>> for CanonicalCosmwasmAccount<T> {
	fn from(from: CosmwasmAccount<T>) -> Self {
		CanonicalCosmwasmAccount(from)
	}
}

impl<T: Config + VMPallet> TryFrom<Vec<u8>> for CanonicalCosmwasmAccount<T> {
	type Error = T::VmError;
	fn try_from(source: Vec<u8>) -> Result<Self, Self::Error> {
		Ok(CanonicalCosmwasmAccount(CosmwasmAccount::new(Pallet::<T>::canonical_addr_to_account(
			source,
		)?)))
	}
}

#[allow(clippy::from_over_into)]
impl<T: Config> Into<CanonicalAddr> for CanonicalCosmwasmAccount<T> {
	fn into(self) -> CanonicalAddr {
		let cosmwasm_account = &self.0;
		CanonicalAddr::from(cosmwasm_account.1.as_ref())
	}
}

#[derive(Clone, Debug, Encode, TypeInfo, PartialEq)]
pub struct CosmwasmAccount<T: Config>(PhantomData<T>, AccountIdOf<T>);

impl<T: Config> CosmwasmAccount<T> {
	pub fn into_inner(self) -> AccountIdOf<T> {
		self.1
	}
}

impl<T: Config> AsRef<AccountIdOf<T>> for CosmwasmAccount<T> {
	fn as_ref(&self) -> &AccountIdOf<T> {
		&self.1
	}
}

impl<T: Config> CosmwasmAccount<T> {
	pub const fn new(x: AccountIdOf<T>) -> Self {
		CosmwasmAccount(PhantomData, x)
	}
}

#[allow(clippy::from_over_into)]
impl<T: Config> From<CosmwasmAccount<T>> for String {
	fn from(account: CosmwasmAccount<T>) -> Self {
		Pallet::<T>::account_to_cosmwasm_addr(account.1)
	}
}

impl<T: Config + VMPallet> TryFrom<String> for CosmwasmAccount<T> {
	type Error = T::VmError;
	fn try_from(value: String) -> Result<Self, Self::Error> {
		Pallet::<T>::cosmwasm_addr_to_account(value).map(CosmwasmAccount::new)
	}
}

#[allow(clippy::from_over_into)]
impl<T: Config> From<CosmwasmAccount<T>> for Addr {
	fn from(account: CosmwasmAccount<T>) -> Self {
		Self::unchecked(String::from(account))
	}
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Gas {
	/// Stack of checkpoints.
	///
	/// Always non-empty with last entry corresponding to the latest checkpoint.
	/// When a new checkpoint is created, gas from the current one is moved to
	/// the new one (see [`Self::push`]) such that total remaining gas is the
	/// sum of gas on all checkpoints.
	checkpoints: Vec<u64>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GasOutcome {
	Halt,
	Continue,
}

impl Gas {
	pub fn new(max_frames: u16, initial_value: u64) -> Self {
		let mut checkpoints = Vec::with_capacity(max_frames.into());
		checkpoints.push(initial_value);
		Gas { checkpoints }
	}

	fn current_mut(&mut self) -> &mut u64 {
		self.checkpoints.last_mut().expect("unbalanced gas checkpoints")
	}

	pub fn push(&mut self, checkpoint: VmGasCheckpoint) -> GasOutcome {
		let parent = self.current_mut();
		match checkpoint {
			VmGasCheckpoint::Unlimited => {
				let value = *parent;
				*parent = 0;
				self.checkpoints.push(value);
				GasOutcome::Continue
			},
			VmGasCheckpoint::Limited(limit) if limit <= *parent => {
				*parent -= limit;
				self.checkpoints.push(limit);
				GasOutcome::Continue
			},
			_ => GasOutcome::Halt,
		}
	}

	pub fn pop(&mut self) {
		let child = self.checkpoints.pop().unwrap();
		let parent = self.current_mut();
		*parent += child;
	}

	pub fn charge(&mut self, value: u64) -> GasOutcome {
		let current = self.current_mut();
		if let Some(left) = current.checked_sub(value) {
			*current = left;
			GasOutcome::Continue
		} else {
			*current = 0;
			GasOutcome::Halt
		}
	}

	pub fn remaining(&self) -> u64 {
		self.checkpoints.iter().sum()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let total_gas = 100_000u64;
		let max_frames = 100;
		let mut gas = Gas::new(max_frames, total_gas);
		assert_eq!(gas.push(VmGasCheckpoint::Limited(50_000)), GasOutcome::Continue);
		assert_eq!(gas.push(VmGasCheckpoint::Limited(30_000)), GasOutcome::Continue);
		assert_eq!(gas.push(VmGasCheckpoint::Limited(20_000)), GasOutcome::Continue);
		assert_eq!(gas.push(VmGasCheckpoint::Limited(10_000)), GasOutcome::Continue);
		assert_eq!(gas.push(VmGasCheckpoint::Limited(5_000)), GasOutcome::Continue);
		assert_eq!(gas.push(VmGasCheckpoint::Limited(5_001)), GasOutcome::Halt);
		assert_eq!(gas.checkpoints, [50000, 20000, 10000, 10000, 5000, 5000]);
		gas.pop();
		assert_eq!(gas.checkpoints, [50000, 20000, 10000, 10000, 10000]);

		assert_eq!(gas.remaining(), total_gas);

		assert_eq!(gas.charge(5000), GasOutcome::Continue);
		assert_eq!(gas.charge(10000), GasOutcome::Halt);
		assert_eq!(gas.checkpoints, [50000, 20000, 10000, 10000, 0]);
		gas.pop();
		assert_eq!(gas.charge(10000), GasOutcome::Continue);
		assert_eq!(gas.checkpoints, [50000, 20000, 10000, 0]);

		assert_eq!(gas.remaining(), total_gas - 20000);
	}
}
