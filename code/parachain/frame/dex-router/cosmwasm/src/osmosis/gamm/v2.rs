use osmosis_std_derive::CosmwasmExt;
/// Deprecated: please use alternate in x/poolmanager
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v2.QuerySpotPriceRequest")]
#[proto_query(
    path = "/osmosis.gamm.v2.Query/SpotPrice",
    response_type = QuerySpotPriceResponse
)]
#[deprecated]
pub struct QuerySpotPriceRequest {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset_denom: ::prost::alloc::string::String,
}
/// Depreacted: please use alternate in x/poolmanager
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.v2.QuerySpotPriceResponse")]
#[deprecated]
pub struct QuerySpotPriceResponse {
    /// String of the Dec. Ex) 10.203uatom
    #[prost(string, tag = "1")]
    pub spot_price: ::prost::alloc::string::String,
}
pub struct GammQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> GammQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    #[deprecated]
    pub fn spot_price(
        &self,
        pool_id: u64,
        base_asset_denom: ::prost::alloc::string::String,
        quote_asset_denom: ::prost::alloc::string::String,
    ) -> Result<QuerySpotPriceResponse, cosmwasm_std::StdError> {
        QuerySpotPriceRequest {
            pool_id,
            base_asset_denom,
            quote_asset_denom,
        }
        .query(self.querier)
    }
}
