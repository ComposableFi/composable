use subxt::Error as SubstrateError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Operation needs to be signed.")]
    OperationNeedsToBeSigned,

    #[error("{0}")]
    Substrate(SubstrateError),

    #[error("Invalid funds. Format should be `asset_id-1:amount-1,asset_id-2:amount-2`")]
    InvalidFundsFormat,

    #[error("Invalid address.")]
    InvalidAddress,

    #[error("Invalid seed.")]
    InvalidSeed,

    #[error("Invalid phrase.")]
    InvalidPhrase,

    #[error("Shell command failure.")]
    ShellCommandFailure,

    #[error("{0}")]
    ToolNotInstalled(String),

    #[error("Internal error: {0}")]
    Internal(Box<dyn std::error::Error>),

    #[error("Codec error occured. `pallet-cosmwasm` versions might be different.")]
    Codec,

    #[error("Extrinsic didn't return an error but it also didn't emit the expected event.")]
    ExpectedEventNotEmitted,

    #[error("Network error: {0}")]
    Network(String),

    #[error("Serialization: {0}")]
    Serialization(String),

    #[error("Rpc error: {0}")]
    Rpc(String),
}
