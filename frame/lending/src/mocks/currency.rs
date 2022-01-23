use crate::{self as pallet_lending, *};
use composable_traits::{
	currency::DynamicCurrencyId,
	defi::DeFiComposableConfig,
	governance::{GovernanceRegistry, SignedRawOrigin},
};
use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{Everything, OnFinalize, OnInitialize},
	PalletId,
};
use frame_system::EnsureSignedBy;
use hex_literal::hex;
use once_cell::sync::Lazy;
use orml_traits::{parameter_type_with_key, GetByKey};
use scale_info::TypeInfo;
use sp_arithmetic::traits::Zero;
use sp_core::{sr25519::Signature, H256};
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{
		BlakeTwo256, ConvertInto, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify,
	},
	ArithmeticError, DispatchError,
};


#[derive(
	PartialOrd,
	Ord,
	PartialEq,
	Eq,
	Debug,
	Copy,
	Clone,
	codec::Encode,
	codec::Decode,
	serde::Serialize,
	serde::Deserialize,
	TypeInfo,
)]
#[allow(clippy::upper_case_acronyms)] // currencies should be CONSTANT_CASE
pub enum CurrencyId {
	PICA,
	BTC,
	ETH,
	LTC,
	USDT,
	LpToken(u128),
}

impl From<u128> for CurrencyId {
	fn from(id: u128) -> Self {
		match id {
			0 => CurrencyId::PICA,
			1 => CurrencyId::BTC,
			2 => CurrencyId::ETH,
			3 => CurrencyId::LTC,
			4 => CurrencyId::USDT,
			5 => CurrencyId::LpToken(0),
			_ => unreachable!(),
		}
	}
}

impl Default for CurrencyId {
	fn default() -> Self {
		CurrencyId::PICA
	}
}

impl DynamicCurrencyId for CurrencyId {
	fn next(self) -> Result<Self, DispatchError> {
		match self {
			CurrencyId::LpToken(x) => Ok(CurrencyId::LpToken(
				x.checked_add(1).ok_or(DispatchError::Arithmetic(ArithmeticError::Overflow))?,
			)),
			_ => unreachable!(),
		}
	}
}