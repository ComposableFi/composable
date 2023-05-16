pub mod parachain;

pub const ibc_escrow_addresses: &str =
    "65ccf20369c0dddad82d1003523ac48e3a3d850485c41d5ea37fe8f1a09a0e53";
pub const system_account_prefix: &str =
    "26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9";
pub const tokens_accounts_prefix: &str =
    "99971b5749ac43e0235e41b0d37869188ee7418a6531173d60d1f6a82d8f4d51";
pub const system_events: &str = "26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7";

pub fn tokens_total_issuance() -> subxt::storage::StorageKey {
    let storage = parachain::api::tokens::storage::StorageApi {};
    let key = storage.total_issuance_root().to_root_bytes();
    subxt::storage::StorageKey(key)
}
