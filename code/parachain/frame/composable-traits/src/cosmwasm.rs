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
	#[error("")]
	Xcm,
}
