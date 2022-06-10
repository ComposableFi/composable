// This file is part of Substrate.

// Copyright (C) 2018-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{exec::ExecError, Config, Error};
use frame_support::{
	dispatch::{
		DispatchError, DispatchErrorWithPostInfo, DispatchResultWithPostInfo, PostDispatchInfo,
	},
	weights::Weight,
	DefaultNoBound,
};
use sp_core::crypto::UncheckedFrom;
use sp_runtime::traits::Zero;
use sp_std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq)]
pub struct ChargedAmount(Weight);

impl ChargedAmount {
	pub fn amount(&self) -> Weight {
		self.0
	}
}

/// This trait represents a token that can be used for charging `GasMeter`.
/// There is no other way of charging it.
///
/// Implementing type is expected to be super lightweight hence `Copy` (`Clone` is added
/// for consistency). If inlined there should be no observable difference compared
/// to a hand-written code.
pub trait Token<T: Config>: Copy + Clone {
	/// Return the amount of gas that should be taken by this token.
	///
	/// This function should be really lightweight and must not fail. It is not
	/// expected that implementors will query the storage or do any kinds of heavy operations.
	///
	/// That said, implementors of this function still can run into overflows
	/// while calculating the amount. In this case it is ok to use saturating operations
	/// since on overflow they will return `max_value` which should consume all gas.
	fn weight(&self) -> Weight;
}

#[derive(DefaultNoBound)]
pub struct GasMeter<T: Config> {
	gas_limit: Weight,
	/// Amount of gas left from initial gas limit. Can reach zero.
	gas_left: Weight,
	/// Due to `adjust_gas` and `nested` the `gas_left` can temporarily dip below its final value.
	gas_left_lowest: Weight,
	_phantom: PhantomData<T>,
}

impl<T: Config> GasMeter<T>
where
	T::AccountId: UncheckedFrom<<T as frame_system::Config>::Hash> + AsRef<[u8]>,
{
	pub fn new(gas_limit: Weight) -> Self {
		GasMeter {
			gas_limit,
			gas_left: gas_limit,
			gas_left_lowest: gas_limit,
			_phantom: PhantomData,
		}
	}

	/// Create a new gas meter by removing gas from the current meter.
	///
	/// # Note
	///
	/// Passing `0` as amount is interpreted as "all remaining gas".
	pub fn nested(&mut self, amount: Weight) -> Result<Self, DispatchError> {
		let amount = if amount == 0 { self.gas_left } else { amount };

		// NOTE that it is ok to allocate all available gas since it still ensured
		// by `charge` that it doesn't reach zero.
		if self.gas_left < amount {
			Err(<Error<T>>::OutOfGas.into())
		} else {
			self.gas_left = self.gas_left - amount;
			Ok(GasMeter::new(amount))
		}
	}

	/// Absorb the remaining gas of a nested meter after we are done using it.
	pub fn absorb_nested(&mut self, nested: Self) {
		if self.gas_left == 0 {
			// All of the remaining gas was inherited by the nested gas meter. When absorbing
			// we can therefore safely inherit the lowest gas that the nested gas meter experienced
			// as long as it is lower than the lowest gas that was experienced by the parent.
			// We cannot call `self.gas_left_lowest()` here because in the state that this
			// code is run the parent gas meter has `0` gas left.
			self.gas_left_lowest = nested.gas_left_lowest().min(self.gas_left_lowest);
		} else {
			// The nested gas meter was created with a fixed amount that did not consume all of the
			// parents (self) gas. The lowest gas that self will experience is when the nested
			// gas was pre charged with the fixed amount.
			self.gas_left_lowest = self.gas_left_lowest();
		}
		self.gas_left += nested.gas_left;
	}

	/// Account for used gas.
	///
	/// Amount is calculated by the given `token`.
	///
	/// Returns `OutOfGas` if there is not enough gas or addition of the specified
	/// amount of gas has lead to overflow. On success returns `Proceed`.
	///
	/// NOTE that amount is always consumed, i.e. if there is not enough gas
	/// then the counter will be set to zero.
	#[inline]
	pub fn charge<Tok: Token<T>>(&mut self, token: Tok) -> Result<ChargedAmount, DispatchError> {
		let amount = token.weight();
		let new_value = self.gas_left.checked_sub(amount);

		// We always consume the gas even if there is not enough gas.
		self.gas_left = new_value.unwrap_or_else(Zero::zero);

		match new_value {
			Some(_) => Ok(ChargedAmount(amount)),
			None => Err(Error::<T>::OutOfGas.into()),
		}
	}

	/// Adjust a previously charged amount down to its actual amount.
	///
	/// This is when a maximum a priori amount was charged and then should be partially
	/// refunded to match the actual amount.
	pub fn adjust_gas<Tok: Token<T>>(&mut self, charged_amount: ChargedAmount, token: Tok) {
		self.gas_left_lowest = self.gas_left_lowest();
		let adjustment = charged_amount.0.saturating_sub(token.weight());
		self.gas_left = self.gas_left.saturating_add(adjustment).min(self.gas_limit);
	}

	/// Returns the amount of gas that is required to run the same call.
	///
	/// This can be different from `gas_spent` because due to `adjust_gas` the amount of
	/// spent gas can temporarily drop and be refunded later.
	pub fn gas_required(&self) -> Weight {
		self.gas_limit - self.gas_left_lowest()
	}

	/// Returns how much gas was spent
	pub fn gas_consumed(&self) -> Weight {
		self.gas_limit - self.gas_left
	}

	/// Returns how much gas left from the initial budget.
	pub fn gas_left(&self) -> Weight {
		self.gas_left
	}

	/// Turn this GasMeter into a DispatchResult that contains the actually used gas.
	pub fn into_dispatch_result<R, E>(
		self,
		result: Result<R, E>,
		base_weight: Weight,
	) -> DispatchResultWithPostInfo
	where
		E: Into<ExecError>,
	{
		let post_info = PostDispatchInfo {
			actual_weight: Some(self.gas_consumed().saturating_add(base_weight)),
			pays_fee: Default::default(),
		};

		result
			.map(|_| post_info)
			.map_err(|e| DispatchErrorWithPostInfo { post_info, error: e.into().error })
	}

	fn gas_left_lowest(&self) -> Weight {
		self.gas_left_lowest.min(self.gas_left)
	}
}
