//! Runtime API definition for the Reward Module.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use frame_support::dispatch::DispatchError;

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Eq, PartialEq, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
/// a wrapper around a balance, used in RPC to workaround a bug where using u128
/// in runtime-apis fails. See <https://github.com/paritytech/substrate/issues/4641>
pub struct BalanceWrapper<T> {
    #[cfg_attr(feature = "std", serde(bound(serialize = "T: std::fmt::Display")))]
    #[cfg_attr(feature = "std", serde(serialize_with = "serialize_as_string"))]
    #[cfg_attr(feature = "std", serde(bound(deserialize = "T: std::str::FromStr")))]
    #[cfg_attr(feature = "std", serde(deserialize_with = "deserialize_from_string"))]
    pub amount: T,
}

#[cfg(feature = "std")]
fn serialize_as_string<S: Serializer, T: std::fmt::Display>(t: &T, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&t.to_string())
}

#[cfg(feature = "std")]
fn deserialize_from_string<'de, D: Deserializer<'de>, T: std::str::FromStr>(deserializer: D) -> Result<T, D::Error> {
    let s = String::deserialize(deserializer)?;
    s.parse::<T>()
        .map_err(|_| serde::de::Error::custom("Parse from string failed"))
}

sp_api::decl_runtime_apis! {
    pub trait RewardApi<AccountId, VaultId, CurrencyId, Balance, BlockNumber, UnsignedFixedPoint> where
        AccountId: Codec,
        VaultId: Codec,
        CurrencyId: Codec,
        Balance: Codec,
        BlockNumber: Codec,
        UnsignedFixedPoint: Codec,
    {
        /// Calculate the number of escrow rewards accrued
        fn compute_escrow_reward(account_id: AccountId, currency_id: CurrencyId) -> Result<BalanceWrapper<Balance>, DispatchError>;

        /// Calculate the number of farming rewards accrued
        fn compute_farming_reward(account_id: AccountId, pool_currency_id: CurrencyId, reward_currency_id: CurrencyId) -> Result<BalanceWrapper<Balance>, DispatchError>;

        /// Calculate the number of vault rewards accrued
        fn compute_vault_reward(vault_id: VaultId, currency_id: CurrencyId) -> Result<BalanceWrapper<Balance>, DispatchError>;

        /// Estimate staking reward rate for a one year period
        fn estimate_escrow_reward_rate(account_id: AccountId, amount: Option<Balance>, lock_time: Option<BlockNumber>) -> Result<UnsignedFixedPoint, DispatchError>;

        /// Estimate farming rewards for remaining incentives
        fn estimate_farming_reward(account_id: AccountId, pool_currency_id: CurrencyId, reward_currency_id: CurrencyId) -> Result<BalanceWrapper<Balance>, DispatchError>;

        /// Estimate vault reward rate for a one year period
        fn estimate_vault_reward_rate(vault_id: VaultId) -> Result<UnsignedFixedPoint, DispatchError>;
    }
}
