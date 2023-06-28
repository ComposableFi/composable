/// Errors from integration of CosmwWasm <-> Substrate (types, coversions, encoding, etc)
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
}
