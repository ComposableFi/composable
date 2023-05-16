pub mod prelude;

use parity_scale_codec::Encode;

use crate::prelude::*;

pub fn request_system_account<AccountId32: AsRef<[u8; 32]>>(
    sender: &UnboundedSender<StorageKey>,
    to: AccountId32,
    prefix: &str,
) {
    let key = format!(
        "{}{}{}",
        prefix,
        hex::encode(sp_core_hashing::blake2_128(to.as_ref())),
        hex::encode(to.as_ref())
    );
    sender
        .unbounded_send(StorageKey(hex::decode(key).unwrap()))
        .unwrap();
}

pub fn request_tokens_account<AccountId32: AsRef<[u8; 32]>>(
    sender: &UnboundedSender<StorageKey>,
    to: AccountId32,
    prefix: &str,
    asset_id: u128,
) {
    let key = format!(
        "{}{}{}{}{}",
        prefix,
        hex::encode(sp_core_hashing::blake2_128(to.as_ref())),
        hex::encode(to.as_ref()),
        hex::encode(sp_core_hashing::twox_64(&asset_id.encode())),
        hex::encode(asset_id.encode()),
    );
    sender
        .unbounded_send(StorageKey(hex::decode(key).unwrap()))
        .unwrap();
}

pub fn request_tokens_total_issuance(
    sender: &UnboundedSender<StorageKey>,
    mut root: StorageKey,
    asset_id: u128,
) {
    root.0
        .append(&mut sp_core_hashing::twox_64(&asset_id.encode()).to_vec());
    root.0.append(&mut asset_id.encode());

    sender.unbounded_send(root).unwrap();
}
