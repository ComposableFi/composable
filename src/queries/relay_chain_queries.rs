use beefy_primitives::{SignedCommitment, VersionedFinalityProof};
use codec::{Decode, Encode};
use pallet_mmr_rpc::{LeafBatchProof, LeafProof};
use sp_core::H256;
use sp_runtime::traits::Header as HeaderT;
use std::collections::BTreeMap;
use subxt::{
    rpc::{rpc_params, ClientT},
    Client, Config,
};

use crate::{error::BeefyClientError, get_leaf_index_for_block_number};

use super::runtime;

pub struct FinalizedParaHeads {
    pub leaf_indices: Vec<u32>,
    pub raw_finalized_heads: BTreeMap<u64, BTreeMap<u32, Vec<u8>>>,
}

/// Get the raw parachain heads finalized in the provided block
pub async fn fetch_finalized_parachain_heads<T: Config>(
    client: &Client<T>,
    commitment_block_number: u32,
    latest_beefy_height: u32,
    para_id: u32,
    beefy_activation_block: u32,
) -> Result<FinalizedParaHeads, BeefyClientError>
where
    u32: From<<T as subxt::Config>::BlockNumber>,
{
    let subxt_block_number: subxt::BlockNumber = commitment_block_number.into();
    let block_hash = client.rpc().block_hash(Some(subxt_block_number)).await?;

    let api = client
        .clone()
        .to_runtime_api::<runtime::api::RuntimeApi<T, subxt::PolkadotExtrinsicParams<_>>>();

    let para_ids = api.storage().paras().parachains(block_hash).await?;
    let storage_prefix = frame_support::storage::storage_prefix(b"Paras", b"Heads");
    let mut para_header_keys = Vec::new();

    for para_id in para_ids {
        let encoded_para_id = para_id.encode();

        let mut full_key = storage_prefix.clone().to_vec();
        full_key.extend_from_slice(sp_core::hashing::twox_64(&encoded_para_id).as_slice());
        full_key.extend_from_slice(&encoded_para_id);
        para_header_keys.push(subxt::sp_core::storage::StorageKey(full_key));
    }

    let previous_finalized_block_number: subxt::BlockNumber = (latest_beefy_height + 1).into();
    let previous_finalized_hash = client
        .rpc()
        .block_hash(Some(previous_finalized_block_number))
        .await?
        .ok_or_else(|| {
            BeefyClientError::Custom(
                "Failed to get previous finalized beefy block hash from block number".to_string(),
            )
        })?;

    let change_set = client
        .storage()
        .query_storage(para_header_keys, previous_finalized_hash, block_hash)
        .await?;
    let mut finalized_blocks = BTreeMap::new();
    let mut leaf_indices = vec![];
    for changes in change_set {
        let header = client
            .rpc()
            .header(Some(changes.block))
            .await?
            .ok_or_else(|| {
                BeefyClientError::Custom(format!(
                    "[get_parachain_headers] block not found {:?}",
                    changes.block
                ))
            })?;

        let mut heads = BTreeMap::new();

        for (key, value) in changes.changes {
            if let Some(storage_data) = value {
                let key = key.0.to_vec();
                let para_id = u32::decode(&mut &key[40..])?;
                let head_data: runtime::api::runtime_types::polkadot_parachain::primitives::HeadData =
                        Decode::decode(&mut &*storage_data.0)?;
                heads.insert(para_id, head_data.0);
            }
        }

        if !heads.contains_key(&para_id) {
            continue;
        }
        let block_number = u32::from(*header.number());
        finalized_blocks.insert(block_number as u64, heads);
        leaf_indices.push(get_leaf_index_for_block_number(
            beefy_activation_block,
            block_number,
        ));
    }

    Ok(FinalizedParaHeads {
        raw_finalized_heads: finalized_blocks,
        leaf_indices,
    })
}

/// Get beefy justification for latest finalized beefy block
pub async fn fetch_beefy_justification<T: Config>(
    client: &Client<T>,
) -> Result<
    (
        SignedCommitment<u32, beefy_primitives::crypto::Signature>,
        T::Hash,
    ),
    BeefyClientError,
> {
    let latest_beefy_finalized: <T as Config>::Hash = client
        .rpc()
        .client
        .request("beefy_getFinalizedHead", rpc_params!())
        .await?;
    let block = client
        .rpc()
        .block(Some(latest_beefy_finalized))
        .await
        .ok()
        .flatten()
        .expect("Should find a valid block");

    let justifications = block
        .justifications
        .expect("Block should have valid justifications");

    let beefy_justification = justifications
        .into_justification(beefy_primitives::BEEFY_ENGINE_ID)
        .expect("Should have valid beefy justification");
    let VersionedFinalityProof::V1(signed_commitment) = VersionedFinalityProof::<
        u32,
        beefy_primitives::crypto::Signature,
    >::decode(&mut &*beefy_justification)
    .expect("Beefy justification should decode correctly");

    Ok((signed_commitment, latest_beefy_finalized))
}

/// Query a batch leaf proof
pub async fn fetch_mmr_batch_proof<T: Config>(
    client: &Client<T>,
    leaf_indices: Vec<u32>,
    block_hash: Option<T::Hash>,
) -> Result<LeafBatchProof<H256>, BeefyClientError> {
    let proof: LeafBatchProof<H256> = client
        .rpc()
        .client
        .request(
            "mmr_generateBatchProof",
            rpc_params!(leaf_indices, block_hash),
        )
        .await?;
    Ok(proof)
}

/// Query a single leaf proof
pub async fn fetch_mmr_leaf_proof<T: Config>(
    client: &Client<T>,
    leaf_index: u64,
    block_hash: Option<T::Hash>,
) -> Result<LeafProof<H256>, BeefyClientError> {
    let proof: LeafProof<H256> = client
        .rpc()
        .client
        .request("mmr_generateProof", rpc_params!(leaf_index, block_hash))
        .await?;

    Ok(proof)
}
