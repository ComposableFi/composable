//they do not support no_std
// so cp paste from https://github.com/osmosis-labs/osmosis-rust/blob/main/packages/osmosis-std/src/types/osmosis/cosmwasmpool/v1beta1.rs

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.Params")]
pub struct Params {
    /// code_ide_whitelist contains the list of code ids that are allowed to be
    /// instantiated.
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    #[serde(alias = "codeID_whitelist")]
    pub code_id_whitelist: ::prost::alloc::vec::Vec<u64>,
    /// pool_migration_limit is the maximum number of pools that can be migrated
    /// at once via governance proposal. This is to have a constant bound on the
    /// number of pools that can be migrated at once and remove the possibility
    /// of an unlikely scenario of causing a chain halt due to a large migration.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_migration_limit: u64,
}
/// GenesisState defines the cosmwasmpool module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params is the container of cosmwasmpool parameters.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub pools: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// UploadCosmWasmPoolCodeAndWhiteListProposal is a gov Content type for
/// uploading coswasm pool code and adding it to internal whitelist. Only the
/// code ids created by this message are eligible for being x/cosmwasmpool pools.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(
    type_url = "/osmosis.cosmwasmpool.v1beta1.UploadCosmWasmPoolCodeAndWhiteListProposal"
)]
pub struct UploadCosmWasmPoolCodeAndWhiteListProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// WASMByteCode can be raw or gzip compressed
    #[prost(bytes = "vec", tag = "3")]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
}
/// MigratePoolContractsProposal is a gov Content type for
/// migrating  given pools to the new contract code and adding to internal
/// whitelist if needed. It has two options to perform the migration:
///
/// 1. If the codeID is non-zero, it will migrate the pool contracts to a given
/// codeID assuming that it has already been uploaded. uploadByteCode must be
/// empty in such a case. Fails if codeID does not exist. Fails if uploadByteCode
/// is not empty.
///
/// 2. If the codeID is zero, it will upload the given uploadByteCode and use the
/// new resulting code id to migrate the pool to. Errors if uploadByteCode is
/// empty or invalid.
///
/// In both cases, if one of the pools specified by the given poolID does not
/// exist, the proposal fails.
///
/// The reason for having poolIDs be a slice of ids is to account for the
/// potential need for emergency migration of all old code ids associated with
/// particular pools to new code ids, or simply having the flexibility of
/// migrating multiple older pool contracts to a new one at once when there is a
/// release.
///
/// poolD count to be submitted at once is gated by a governance paramets (20 at
/// launch). The proposal fails if more. Note that 20 was chosen arbitrarily to
/// have a constant bound on the number of pools migrated at once. This size will
/// be configured by a module parameter so it can be changed by a constant.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.MigratePoolContractsProposal")]
pub struct MigratePoolContractsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// pool_ids are the pool ids of the contracts to be migrated
    /// either to the new_code_id that is already uploaded to chain or to
    /// the given wasm_byte_code.
    #[prost(uint64, repeated, tag = "3")]
    #[serde(alias = "poolIDs")]
    pub pool_ids: ::prost::alloc::vec::Vec<u64>,
    /// new_code_id is the code id of the contract code to migrate to.
    /// Assumes that the code is already uploaded to chain. Only one of
    /// new_code_id and wasm_byte_code should be set.
    #[prost(uint64, tag = "4")]
    #[serde(alias = "new_codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub new_code_id: u64,
    /// WASMByteCode can be raw or gzip compressed. Assumes that the code id
    /// has not been uploaded yet so uploads the given code and migrates to it.
    /// Only one of new_code_id and wasm_byte_code should be set.
    #[prost(bytes = "vec", tag = "5")]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
    /// MigrateMsg migrate message to be used for migrating the pool contracts.
    #[prost(bytes = "vec", tag = "6")]
    pub migrate_msg: ::prost::alloc::vec::Vec<u8>,
}
/// ===================== InstantiateMsg
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.InstantiateMsg")]
pub struct InstantiateMsg {
    /// pool_asset_denoms is the list of asset denoms that are initialized
    /// at pool creation time.
    #[prost(string, repeated, tag = "1")]
    pub pool_asset_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ===================== CalcOutAmtGivenIn
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.CalcOutAmtGivenIn")]
pub struct CalcOutAmtGivenIn {
    /// token_in is the token to be sent to the pool.
    #[prost(message, optional, tag = "1")]
    pub token_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// token_out_denom is the token denom to be received from the pool.
    #[prost(string, tag = "2")]
    pub token_out_denom: ::prost::alloc::string::String,
    /// swap_fee is the swap fee for this swap estimate.
    #[prost(string, tag = "3")]
    pub swap_fee: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.CalcOutAmtGivenInRequest")]
pub struct CalcOutAmtGivenInRequest {
    /// calc_out_amt_given_in is the structure containing all the request
    /// information for this query.
    #[prost(message, optional, tag = "1")]
    pub calc_out_amt_given_in: ::core::option::Option<CalcOutAmtGivenIn>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.CalcOutAmtGivenInResponse")]
pub struct CalcOutAmtGivenInResponse {
    /// token_out is the token out computed from this swap estimate call.
    #[prost(message, optional, tag = "1")]
    pub token_out: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// ===================== CalcInAmtGivenOut
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.CalcInAmtGivenOut")]
pub struct CalcInAmtGivenOut {
    /// token_out is the token out to be receoved from the pool.
    #[prost(message, optional, tag = "1")]
    pub token_out: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// token_in_denom is the token denom to be sentt to the pool.
    #[prost(string, tag = "2")]
    pub token_in_denom: ::prost::alloc::string::String,
    /// swap_fee is the swap fee for this swap estimate.
    #[prost(string, tag = "3")]
    pub swap_fee: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.CalcInAmtGivenOutRequest")]
pub struct CalcInAmtGivenOutRequest {
    /// calc_in_amt_given_out is the structure containing all the request
    /// information for this query.
    #[prost(message, optional, tag = "1")]
    pub calc_in_amt_given_out: ::core::option::Option<CalcInAmtGivenOut>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.CalcInAmtGivenOutResponse")]
pub struct CalcInAmtGivenOutResponse {
    /// token_in is the token in computed from this swap estimate call.
    #[prost(message, optional, tag = "1")]
    pub token_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// ===================== SwapExactAmountIn
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.SwapExactAmountIn")]
pub struct SwapExactAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// token_in is the token to be sent to the pool.
    #[prost(message, optional, tag = "2")]
    pub token_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// token_out_denom is the token denom to be received from the pool.
    #[prost(string, tag = "3")]
    pub token_out_denom: ::prost::alloc::string::String,
    /// token_out_min_amount is the minimum amount of token_out to be received from
    /// the pool.
    #[prost(string, tag = "4")]
    pub token_out_min_amount: ::prost::alloc::string::String,
    /// swap_fee is the swap fee for this swap estimate.
    #[prost(string, tag = "5")]
    pub swap_fee: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.SwapExactAmountInSudoMsg")]
pub struct SwapExactAmountInSudoMsg {
    /// swap_exact_amount_in is the structure containing all the request
    /// information for this message.
    #[prost(message, optional, tag = "1")]
    pub swap_exact_amount_in: ::core::option::Option<SwapExactAmountIn>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.SwapExactAmountInSudoMsgResponse")]
pub struct SwapExactAmountInSudoMsgResponse {
    /// token_out_amount is the token out computed from this swap estimate call.
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== SwapExactAmountOut
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.SwapExactAmountOut")]
pub struct SwapExactAmountOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// token_out is the token to be sent out of the pool.
    #[prost(message, optional, tag = "2")]
    pub token_out: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// token_in_denom is the token denom to be sent too the pool.
    #[prost(string, tag = "3")]
    pub token_in_denom: ::prost::alloc::string::String,
    /// token_in_max_amount is the maximum amount of token_in to be sent to the
    /// pool.
    #[prost(string, tag = "4")]
    pub token_in_max_amount: ::prost::alloc::string::String,
    /// swap_fee is the swap fee for this swap estimate.
    #[prost(string, tag = "5")]
    pub swap_fee: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.SwapExactAmountOutSudoMsg")]
pub struct SwapExactAmountOutSudoMsg {
    /// swap_exact_amount_out is the structure containing all the request
    /// information for this message.
    #[prost(message, optional, tag = "1")]
    pub swap_exact_amount_out: ::core::option::Option<SwapExactAmountOut>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.SwapExactAmountOutSudoMsgResponse")]
pub struct SwapExactAmountOutSudoMsgResponse {
    /// token_in_amount is the token in computed from this swap estimate call.
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.CosmWasmPool")]
pub struct CosmWasmPool {
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(uint64, tag = "3")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    #[prost(bytes = "vec", tag = "4")]
    pub instantiate_msg: ::prost::alloc::vec::Vec<u8>,
}
/// ===================== GetSwapFeeQueryMsg
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.GetSwapFeeQueryMsg")]
pub struct GetSwapFeeQueryMsg {
    /// get_swap_fee is the query strcuture to get swap fee.
    #[prost(message, optional, tag = "1")]
    pub get_swap_fee: ::core::option::Option<EmptyStruct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.GetSwapFeeQueryMsgResponse")]
pub struct GetSwapFeeQueryMsgResponse {
    /// swap_fee is the swap fee for this swap estimate.
    #[prost(string, tag = "3")]
    pub swap_fee: ::prost::alloc::string::String,
}
/// ===================== SpotPriceQueryMsg
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.SpotPrice")]
pub struct SpotPrice {
    /// quote_asset_denom is the quote asset of the spot query.
    #[prost(string, tag = "1")]
    pub quote_asset_denom: ::prost::alloc::string::String,
    /// base_asset_denom is the base asset of the spot query.
    #[prost(string, tag = "2")]
    pub base_asset_denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.SpotPriceQueryMsg")]
pub struct SpotPriceQueryMsg {
    /// spot_price is the structure containing request field of the spot price
    /// query message.
    #[prost(message, optional, tag = "1")]
    pub spot_price: ::core::option::Option<SpotPrice>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.SpotPriceQueryMsgResponse")]
pub struct SpotPriceQueryMsgResponse {
    /// spot_price is the spot price returned.
    #[prost(string, tag = "1")]
    pub spot_price: ::prost::alloc::string::String,
}
/// ===================== GetTotalPoolLiquidityQueryMsg
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.EmptyStruct")]
pub struct EmptyStruct {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.GetTotalPoolLiquidityQueryMsg")]
pub struct GetTotalPoolLiquidityQueryMsg {
    /// get_total_pool_liquidity is the structure containing request field of the
    /// total pool liquidity query message.
    #[prost(message, optional, tag = "1")]
    pub get_total_pool_liquidity: ::core::option::Option<EmptyStruct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.GetTotalPoolLiquidityQueryMsgResponse")]
pub struct GetTotalPoolLiquidityQueryMsgResponse {
    ///   total_pool_liquidity is the total liquidity in the pool denominated in
    ///   coins.
    #[prost(message, repeated, tag = "1")]
    pub total_pool_liquidity:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// ===================== GetTotalSharesQueryMsg
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.GetTotalSharesQueryMsg")]
pub struct GetTotalSharesQueryMsg {
    /// get_total_shares is the structure containing request field of the
    /// total shares query message.
    #[prost(message, optional, tag = "1")]
    pub get_total_shares: ::core::option::Option<EmptyStruct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.GetTotalSharesQueryMsgResponse")]
pub struct GetTotalSharesQueryMsgResponse {
    /// total_shares is the amount of shares returned.
    #[prost(string, tag = "1")]
    pub total_shares: ::prost::alloc::string::String,
}
/// ===================== JoinPoolExecuteMsg
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.EmptyRequest")]
pub struct EmptyRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.JoinPoolExecuteMsgRequest")]
pub struct JoinPoolExecuteMsgRequest {
    /// join_pool is the structure containing all request fields of the join pool
    /// execute message.
    #[prost(message, optional, tag = "1")]
    pub join_pool: ::core::option::Option<EmptyRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.JoinPoolExecuteMsgResponse")]
pub struct JoinPoolExecuteMsgResponse {}
/// ===================== ExitPoolExecuteMsg
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.ExitPoolExecuteMsgRequest")]
pub struct ExitPoolExecuteMsgRequest {
    /// exit_pool is the structure containing all request fields of the exit pool
    /// execute message.
    #[prost(message, optional, tag = "1")]
    pub exit_pool: ::core::option::Option<EmptyRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.ExitPoolExecuteMsgResponse")]
pub struct ExitPoolExecuteMsgResponse {}
/// ===================== MsgCreateCosmwasmPool
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.MsgCreateCosmWasmPool")]
pub struct MsgCreateCosmWasmPool {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub instantiate_msg: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
}
/// Returns a unique poolID to identify the pool with.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.MsgCreateCosmWasmPoolResponse")]
pub struct MsgCreateCosmWasmPoolResponse {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
}
/// =============================== ContractInfoByPoolId
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.ParamsRequest")]
#[proto_query(
    path = "/osmosis.cosmwasmpool.v1beta1.Query/Params",
    response_type = ParamsResponse
)]
pub struct ParamsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.ParamsResponse")]
pub struct ParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// =============================== Pools
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.PoolsRequest")]
#[proto_query(
    path = "/osmosis.cosmwasmpool.v1beta1.Query/Pools",
    response_type = PoolsResponse
)]
pub struct PoolsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.PoolsResponse")]
pub struct PoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<crate::shim::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// =============================== ContractInfoByPoolId
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.ContractInfoByPoolIdRequest")]
#[proto_query(
    path = "/osmosis.cosmwasmpool.v1beta1.Query/ContractInfoByPoolId",
    response_type = ContractInfoByPoolIdResponse
)]
pub struct ContractInfoByPoolIdRequest {
    /// pool_id is the pool id of the requested pool.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.ContractInfoByPoolIdResponse")]
pub struct ContractInfoByPoolIdResponse {
    /// contract_address is the pool address and contract address
    /// of the requested pool id.
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// code_id is the code id of the requested pool id.
    #[prost(uint64, tag = "2")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
}