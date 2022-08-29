use super::runtime;
use crate::error::Error;
use beefy_client_primitives::get_leaf_index_for_block_number;
use beefy_primitives::{SignedCommitment, VersionedFinalityProof};
use codec::{Decode, Encode};
use pallet_mmr_rpc::{LeafBatchProof, LeafProof};
use sp_core::{storage::StorageKey, H256};
use sp_runtime::traits::Header;
use sp_runtime::traits::Zero;
use std::collections::{BTreeMap, BTreeSet};
use subxt::{
    rpc::{rpc_params, ClientT},
    Client, Config,
};

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
    header_numbers: &BTreeSet<T::BlockNumber>,
) -> Result<FinalizedParaHeads, Error>
where
    u32: From<<T as subxt::Config>::BlockNumber>,
    T::BlockNumber: Ord + Zero,
{
    let subxt_block_number: subxt::BlockNumber = commitment_block_number.into();
    let block_hash = client.rpc().block_hash(Some(subxt_block_number)).await?;

    let api = client
        .clone()
        .to_runtime_api::<runtime::api::RuntimeApi<T, subxt::PolkadotExtrinsicParams<_>>>();

    let para_ids = api.storage().paras().parachains(block_hash).await?;
    let previous_finalized_block_number: subxt::BlockNumber = (latest_beefy_height + 1).into();
    let previous_finalized_hash = client
        .rpc()
        .block_hash(Some(previous_finalized_block_number))
        .await?
        .ok_or_else(|| {
            Error::Custom(
                "Failed to get previous finalized beefy block hash from block number".to_string(),
            )
        })?;

    let change_set = client
        .storage()
        .query_storage(
            // we are interested only in the blocks where our parachain header changes.
            vec![parachain_header_storage_key(para_id)],
            previous_finalized_hash,
            block_hash,
        )
        .await?;
    let mut finalized_blocks = BTreeMap::new();
    let mut leaf_indices = vec![];

    for changes in change_set {
        let header = client
            .rpc()
            .header(Some(changes.block))
            .await?
            .ok_or_else(|| {
                Error::Custom(format!(
                    "[get_parachain_headers] block not found {:?}",
                    changes.block
                ))
            })?;

        let mut heads = BTreeMap::new();
        for id in para_ids.iter() {
            if let Some(head) = api
                .storage()
                .paras()
                .heads(id, Some(header.hash()))
                .await?
            {
                heads.insert(id.0, head.0);
            }
        }

        let para_header: T::Header = Decode::decode(&mut &heads[&para_id][..])
            .map_err(|_| Error::Custom(format!("Failed to decode header for {para_id}")))?;
        let para_block_number = *para_header.number();
        // skip genesis header or any unknown headers
        if para_block_number == Zero::zero() || !header_numbers.contains(&para_block_number) {
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
    Error,
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
) -> Result<LeafBatchProof<H256>, Error> {
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
) -> Result<LeafProof<H256>, Error> {
    let proof: LeafProof<H256> = client
        .rpc()
        .client
        .request("mmr_generateProof", rpc_params!(leaf_index, block_hash))
        .await?;

    Ok(proof)
}

pub fn parachain_header_storage_key(para_id: u32) -> StorageKey {
    let mut storage_key = frame_support::storage::storage_prefix(b"Paras", b"Heads").to_vec();
    let encoded_para_id = para_id.encode();
    storage_key.extend_from_slice(sp_core::hashing::twox_64(&encoded_para_id).as_slice());
    storage_key.extend_from_slice(&encoded_para_id);
    StorageKey(storage_key)
}
