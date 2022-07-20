use crate::{Config, Pallet};
use alloc::string::String;
use core::marker::PhantomData;

pub trait VMPallet {
	type VmError;
}

#[derive(Clone, Debug)]
pub struct CosmwasmAccount<T, U>(PhantomData<T>, U);

impl<T, U> AsRef<U> for CosmwasmAccount<T, U> {
	fn as_ref(&self) -> &U {
		&self.1
	}
}

impl<T, U> CosmwasmAccount<T, U> {
	pub fn new(x: U) -> Self {
		CosmwasmAccount(PhantomData, x)
	}
}

impl<T: Config> Into<String> for CosmwasmAccount<T, <T as frame_system::Config>::AccountId> {
	fn into(self) -> String {
		Pallet::<T>::account_to_cosmwasm_addr(self.1)
	}
}

impl<T: Config + VMPallet> TryFrom<String>
	for CosmwasmAccount<T, <T as frame_system::Config>::AccountId>
{
	type Error = T::VmError;
	fn try_from(value: String) -> Result<Self, Self::Error> {
		Pallet::<T>::cosmwasm_addr_to_account(value).map(CosmwasmAccount::new)
	}
}
