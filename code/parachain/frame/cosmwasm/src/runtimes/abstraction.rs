use crate::{types::AccountIdOf, Config, Pallet};
use alloc::{string::String, vec::Vec};
use codec::Encode;
use core::marker::PhantomData;
use cosmwasm_std::{Addr, CanonicalAddr};
use cosmwasm_vm::vm::VmGasCheckpoint;
use scale_info::TypeInfo;
use vec1::Vec1;

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
	checkpoints: Vec1<u64>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GasOutcome {
	Halt,
	Continue,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TooManyCheckpoints;
#[derive(Debug, PartialEq, Eq)]
pub struct NoCheckpointToPop;

impl Gas {
	pub fn new(max_frames: u8, initial_value: u64) -> Self {
		let max_frames = usize::from(max_frames).max(1);
		Gas { checkpoints: Vec1::with_capacity(initial_value, max_frames) }
	}

	fn current_mut(&mut self) -> &mut u64 {
		self.checkpoints.last_mut()
	}

	/// Pushes a new gas checkpoint.
	///
	/// If `max_frames` number of checkpoints have been reached, returns
	/// [`TooManyCheckpoints`] error.  Otherwise, checks if there’s enough gas
	/// at the current checkpoint and if so creates a new checkpoint with
	/// requested limit.
	pub fn push(&mut self, checkpoint: VmGasCheckpoint) -> Result<GasOutcome, TooManyCheckpoints> {
		if self.checkpoints.len() >= self.checkpoints.capacity() {
			return Err(TooManyCheckpoints)
		}
		let parent = self.current_mut();
		Ok(match checkpoint {
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
		})
	}
	/// Pops the last gas checkpoint.
	///
	/// Any gas limit remaining in the checkpoint is added back to the parent
	/// checkpoint.  Returns an error if function tries to pop the final
	/// checkpoint.
	pub fn pop(&mut self) -> Result<(), NoCheckpointToPop> {
		self.checkpoints
			.pop()
			.map(|child| {
				*self.current_mut() += child;
			})
			.or(Err(NoCheckpointToPop))
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
		// always less than `initial_value`, it will not panic
		self.checkpoints.iter().sum()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_checkpoint_gas_limits() {
		let total_gas = 100_000u64;
		let max_frames = 100;
		let mut gas = Gas::new(max_frames, total_gas);
		assert_eq!(gas.push(VmGasCheckpoint::Limited(50_000)), Ok(GasOutcome::Continue));
		assert_eq!(gas.push(VmGasCheckpoint::Limited(30_000)), Ok(GasOutcome::Continue));
		assert_eq!(gas.push(VmGasCheckpoint::Limited(20_000)), Ok(GasOutcome::Continue));
		assert_eq!(gas.push(VmGasCheckpoint::Limited(10_000)), Ok(GasOutcome::Continue));
		assert_eq!(gas.push(VmGasCheckpoint::Limited(5_000)), Ok(GasOutcome::Continue));
		assert_eq!(gas.push(VmGasCheckpoint::Limited(5_001)), Ok(GasOutcome::Halt));
		assert_eq!(gas.checkpoints, [50000, 20000, 10000, 10000, 5000, 5000]);
		assert_eq!(gas.pop(), Ok(()));
		assert_eq!(gas.checkpoints, [50000, 20000, 10000, 10000, 10000]);

		assert_eq!(gas.remaining(), total_gas);

		assert_eq!(gas.charge(5000), GasOutcome::Continue);
		assert_eq!(gas.charge(10000), GasOutcome::Halt);
		assert_eq!(gas.checkpoints, [50000, 20000, 10000, 10000, 0]);
		assert_eq!(gas.pop(), Ok(()));
		assert_eq!(gas.charge(10000), GasOutcome::Continue);
		assert_eq!(gas.checkpoints, [50000, 20000, 10000, 0]);

		assert_eq!(gas.remaining(), total_gas - 20000);
	}

	#[test]
	fn test_invalid_checkpoints() {
		const TOTAL_GAS: u64 = 100;
		let mut gas = Gas::new(3, TOTAL_GAS);
		assert_eq!(gas.pop(), Err(NoCheckpointToPop));

		for _ in 0..2 {
			assert_eq!(gas.push(VmGasCheckpoint::Limited(50)), Ok(GasOutcome::Continue));
			assert_eq!(gas.push(VmGasCheckpoint::Limited(50)), Ok(GasOutcome::Continue));
			assert_eq!(gas.push(VmGasCheckpoint::Limited(50)), Err(TooManyCheckpoints));

			assert_eq!(gas.pop(), Ok(()));
			assert_eq!(gas.pop(), Ok(()));
			assert_eq!(gas.pop(), Err(NoCheckpointToPop));

			assert_eq!(gas.remaining(), TOTAL_GAS);
		}

		assert_eq!(gas.push(VmGasCheckpoint::Limited(50)), Ok(GasOutcome::Continue));
		assert_eq!(gas.push(VmGasCheckpoint::Limited(60)), Ok(GasOutcome::Halt));
	}
}
