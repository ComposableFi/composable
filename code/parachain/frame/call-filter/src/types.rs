use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

use crate::prelude::*;

/// An object that is able to tell us whether an entry can or cannot be disabled.
pub trait CallFilterHook<S: Get<u32>> {
	fn enable_hook(entry: &CallFilterEntry<S>) -> DispatchResult;
	fn disable_hook(entry: &CallFilterEntry<S>) -> DispatchResult;
}

impl<S: Get<u32>> CallFilterHook<S> for () {
	#[inline(always)]
	fn enable_hook(_: &CallFilterEntry<S>) -> DispatchResult {
		Ok(())
	}
	#[inline(always)]
	fn disable_hook(_: &CallFilterEntry<S>) -> DispatchResult {
		Ok(())
	}
}

/// An object that is able to pause/unpause extrinsics.
pub trait CallFilter<S: Get<u32>> {
	fn disabled(entry: &CallFilterEntry<S>) -> bool;
	fn enable(entry: &CallFilterEntry<S>) -> DispatchResult;
	fn disable(entry: &CallFilterEntry<S>) -> DispatchResult;
}

/// A call filter entry, product of the pallet name and the extrinsic name.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct CallFilterEntry<S: Get<u32>> {
	pub pallet_name: BoundedVec<u8, S>,
	pub function_name: BoundedVec<u8, S>,
}

impl<S: Get<u32>> CallFilterEntry<S> {
	pub fn valid(&self) -> bool {
		sp_std::str::from_utf8(&self.pallet_name).is_ok() &&
			sp_std::str::from_utf8(&self.function_name).is_ok()
	}
}
