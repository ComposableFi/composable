use itertools::Itertools;
use primitives::currency::CurrencyId;
use sp_core::{blake2_256, crypto::Ss58Codec, Decode, Encode};
use sp_runtime::{traits::TrailingZeroInput, AccountId32};
use ss58_registry::Ss58AddressFormat;
use std::{marker::PhantomData, ops::Add};
use substrate_api_client::ApiResult;

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
pub struct Batch<Call> {
	pub calls: Vec<Call>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Raw;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Canonical;

#[derive(Copy, Clone, Debug)]
pub struct Amount<T>(u128, PhantomData<T>);

impl<T> const PartialEq for Amount<T> {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl<T> const Add for Amount<T> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Amount(self.0 + rhs.0, PhantomData)
	}
}

impl<T> Add<&Self> for Amount<T> {
	type Output = Self;
	fn add(self, rhs: &Self) -> Self::Output {
		Amount(self.0 + rhs.0, PhantomData)
	}
}

impl From<Amount<Canonical>> for u128 {
	fn from(Amount(x, _): Amount<Canonical>) -> Self {
		x
	}
}

impl From<Amount<Raw>> for Amount<Canonical> {
	fn from(Amount(x, _): Amount<Raw>) -> Self {
		Amount(x.checked_mul(CurrencyId::unit::<u128>()).expect("impossible"), PhantomData)
	}
}

impl Amount<Raw> {
	pub const fn new(x: u128) -> Self {
		Self(x, PhantomData)
	}
}

#[derive(Debug)]
pub enum CommonError {
	InvalidAccount(String),
}

pub fn extract_account(account_src: &str) -> Result<AccountId32, CommonError> {
	let account = AccountId32::from_ss58check(account_src)
		.map_err(|_| CommonError::InvalidAccount(account_src.to_string()))?;
	Ok(account)
}

pub fn multi_account_id(
	format: impl Into<Ss58AddressFormat> + Copy,
	composite_accounts: &[&str],
	threshold: u16,
) -> Result<AccountId32, CommonError> {
	let verified_composite = composite_accounts
		.iter()
		.map(|x| extract_account(x))
		.collect::<Result<Vec<_>, _>>()?;

	let sorted_verified_composite = verified_composite
		.into_iter()
		.sorted_by(|x, y| {
			Ord::cmp(
				&x.to_ss58check_with_version(format.into()),
				&y.to_ss58check_with_version(format.into()),
			)
		})
		.collect::<Vec<_>>();

	let entropy =
		(b"modlpy/utilisuba", &sorted_verified_composite, threshold).using_encoded(blake2_256);
	Ok(Decode::decode(&mut TrailingZeroInput::new(entropy.as_ref()))
		.expect("infinite length input; no invalid inputs for type; qed"))
}

pub fn api_wrap<C, E: From<substrate_api_client::ApiClientError>>(x: ApiResult<C>) -> Result<C, E> {
	x.map_err(Into::into)
}
