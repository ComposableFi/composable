use crate::{types::AccountIdOf, Config, Pallet};
use alloc::{collections::VecDeque, string::String, vec::Vec};
use codec::Encode;
use core::marker::PhantomData;
use cosmwasm_vm::{
	cosmwasm_std::{Addr, CanonicalAddr},
	vm::VmGasCheckpoint,
};
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
impl<T: Config> Into<String> for CosmwasmAccount<T> {
	fn into(self) -> String {
		Pallet::<T>::account_to_cosmwasm_addr(self.1)
	}
}

impl<T: Config + VMPallet> TryFrom<String> for CosmwasmAccount<T> {
	type Error = T::VmError;
	fn try_from(value: String) -> Result<Self, Self::Error> {
		Pallet::<T>::cosmwasm_addr_to_account(value).map(CosmwasmAccount::new)
	}
}

#[allow(clippy::from_over_into)]
impl<T: Config> Into<Addr> for CosmwasmAccount<T> {
	fn into(self) -> Addr {
		Addr::unchecked(Into::<String>::into(self))
	}
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Gas {
	checkpoints: VecDeque<u64>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GasOutcome {
	Halt,
	Continue,
}

impl Gas {
	pub fn new(max_frames: u32, initial_value: u64) -> Self {
		let mut checkpoints = VecDeque::with_capacity(max_frames as _);
		checkpoints.push_front(initial_value);
		Gas { checkpoints }
	}
	fn current(&self) -> u64 {
		*self.checkpoints.front().expect("at least one frame must be running; qed;")
	}
	fn current_mut(&mut self) -> &mut u64 {
		self.checkpoints.front_mut().expect("at least one frame must be running; qed;")
	}
	pub fn push(&mut self, checkpoint: VmGasCheckpoint) -> GasOutcome {
		match checkpoint {
			VmGasCheckpoint::Unlimited => {
				let parent = self.current_mut();
				let value = *parent;
				*parent = 0;
				self.checkpoints.push_front(value);
				GasOutcome::Continue
			},
			VmGasCheckpoint::Limited(limit) if limit <= self.current() => {
				*self.current_mut() -= limit;
				self.checkpoints.push_front(limit);
				GasOutcome::Continue
			},
			_ => GasOutcome::Halt,
		}
	}
	pub fn pop(&mut self) {
		let child = self.checkpoints.pop_front().expect("impossible");
		let parent = self.current_mut();
		*parent += child;
	}
	pub fn charge(&mut self, value: u64) -> GasOutcome {
		let current = self.current_mut();
		if *current >= value {
			*current -= value;
			GasOutcome::Continue
		} else {
			*current = current.saturating_sub(value);
			GasOutcome::Halt
		}
	}
	pub fn remaining(&self) -> u64 {
		self.checkpoints.iter().sum::<u64>()
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
		assert_eq!(gas.checkpoints, [5000, 5000, 10000, 10000, 20000, 50000]);

		// TODO: PROPTEST
		// Property: on push/pop, gas is always distributed, never created/destroyed
		assert_eq!(gas.remaining(), total_gas);
	}
}
