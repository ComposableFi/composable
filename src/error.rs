pub enum BeefyClientError {
    /// Failed to read a value from storage
    StorageReadError,
    /// Failed to write a value to storage
    StorageWriteError,
    /// Error deriving [`sp_core::H256`] from `Vec<u8>`
    HashDecodeError,
}
