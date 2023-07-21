//! so we need just some code which helps to convert substrate and CW primitives back and forth

/// Errors from integration of CosmwWasm <-> Substrate (types, conversions, encoding, host
/// functions, etc)
#[derive(thiserror::Error, Debug)]
pub enum CosmwasmSubstrateError {
	#[error("")]
	AssetConversion,
	#[error("")]
	AccountConvert,
	#[error("")]
	DispatchError,
	#[error("")]
	QuerySerialize,
	#[error("")]
	ExecuteSerialize,
	#[error("")]
	Ibc,
}
