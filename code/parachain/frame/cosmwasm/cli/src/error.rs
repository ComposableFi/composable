#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Operation needs to be signed.")]
	OperationNeedsToBeSigned,

	#[error("Invalid funds. Format should be `asset_id-1:amount-1,asset_id-2:amount-2`")]
	InvalidFundsFormat,

	#[error("Invalid address.")]
	InvalidAddress,

	#[error("Invalid seed.")]
	InvalidSeed,

	#[error("Invalid phrase.")]
	InvalidPhrase,

	/// The overall format was invalid (e.g. the seed phrase contained symbols).
	#[error("Invalid format")]
	SecretStringInvalidFormat,
	/// The seed phrase provided is not a valid BIP39 phrase.
	#[error("Invalid phrase")]
	SecretStringInvalidPhrase,
	/// The supplied password was invalid.
	#[error("Invalid password")]
	SecretStringInvalidPassword,
	/// The seed is invalid (bad content).
	#[error("Invalid seed")]
	SecretStringInvalidSeed,
	/// The seed has an invalid length.
	#[error("Invalid seed length")]
	SecretStringInvalidSeedLength,
	/// The derivation path was invalid (e.g. contains soft junctions when they are not supported).
	#[error("Invalid path")]
	SecretStringInvalidPath,
	#[error("{0}")]
	SerdeJson(#[from] serde_json::Error),
	#[error("{0}")]
	Reqwest(#[from] reqwest::Error),
	#[error("{0}")]
	StdIo(#[from] std::io::Error),
	#[error("{0}")]
	Jsonrpc(#[from] jsonrpc::Error),
	#[error("{0}")]
	Subxt(subxt::Error),
	#[error("{0:?}")]
	SubxtRuntime(subxt::error::DispatchError),
	#[error("{0}")]
	SubxtModule(String),
}

impl From<subxt::Error> for Error {
	fn from(value: subxt::Error) -> Self {
		match value {
			subxt::Error::Runtime(inner) => match &inner {
				subxt::error::DispatchError::Module(inner) => {
					let details = inner.details().expect("correct metadata used");
					Self::SubxtModule(format!("{:?}", details.variant))
				},
				_ => Self::SubxtRuntime(inner),
			},
			x => Self::Subxt(x),
		}
	}
}

impl From<sp_core::crypto::SecretStringError> for Error {
	fn from(value: sp_core::crypto::SecretStringError) -> Self {
		match value {
			sp_core::crypto::SecretStringError::InvalidFormat => Self::SecretStringInvalidFormat,
			sp_core::crypto::SecretStringError::InvalidPhrase => Self::SecretStringInvalidPhrase,
			sp_core::crypto::SecretStringError::InvalidPassword =>
				Self::SecretStringInvalidPassword,
			sp_core::crypto::SecretStringError::InvalidSeed => Self::SecretStringInvalidSeed,
			sp_core::crypto::SecretStringError::InvalidSeedLength =>
				Self::SecretStringInvalidSeedLength,
			sp_core::crypto::SecretStringError::InvalidPath => Self::SecretStringInvalidPath,
		}
	}
}
