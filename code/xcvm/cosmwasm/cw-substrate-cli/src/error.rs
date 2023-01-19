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

    #[error("Shell command failure.")]
    ShellCommandFailure,

    #[error("{0}")]
    ToolNotInstalled(String),
}
