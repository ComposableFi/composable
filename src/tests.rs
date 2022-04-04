use crate::primitives::{ParachainHeader, PartialMmrLeaf, SignedCommitment};
use crate::{
    runtime, BeefyLightClient, KeccakHasher, MmrUpdateProof, ParachainsUpdateProof,
    SignatureWithAuthorityIndex,
};
use crate::{AuthoritySet, BeefyClientError, MmrState, StorageRead, StorageWrite, H256};
use beefy_primitives::known_payload_ids::MMR_ROOT_ID;
use beefy_primitives::mmr::{BeefyNextAuthoritySet, MmrLeaf};
use beefy_primitives::Payload;
use codec::{Decode, Encode};
use frame_support::assert_ok;
use hex_literal::hex;
use pallet_mmr_primitives::Proof;
use sp_core::bytes::to_hex;
use sp_runtime::traits::Convert;
use std::collections::BTreeMap;
use subxt::rpc::ClientT;
use subxt::rpc::{rpc_params, JsonValue, Subscription, SubscriptionClientT};
use subxt::sp_core::keccak_256;

pub const PARA_ID: u32 = 2000;
pub struct StorageMock {
    mmr_state: MmrState,
    authority_set: AuthoritySet,
}

impl StorageMock {
    fn new() -> Self {
        Self {
            mmr_state: MmrState {
                latest_beefy_height: 0,
                mmr_root_hash: Default::default(),
            },
            authority_set: AuthoritySet {
                current_authorities: BeefyNextAuthoritySet {
                    id: 0,
                    len: 5,
                    root: H256::from(hex!(
                        "baa93c7834125ee3120bac6e3342bd3f28611110ad21ab6075367abdffefeb09"
                    )),
                },
                next_authorities: BeefyNextAuthoritySet {
                    id: 1,
                    len: 5,
                    root: H256::from(hex!(
                        "baa93c7834125ee3120bac6e3342bd3f28611110ad21ab6075367abdffefeb09"
                    )),
                },
            },
        }
    }
}

impl StorageRead for StorageMock {
    fn mmr_state(&self) -> Result<MmrState, BeefyClientError> {
        Ok(self.mmr_state.clone())
    }

    fn authority_set(&self) -> Result<AuthoritySet, BeefyClientError> {
        Ok(self.authority_set.clone())
    }
}

impl StorageWrite for StorageMock {
    fn set_mmr_state(&mut self, mmr_state: MmrState) -> Result<(), BeefyClientError> {
        self.mmr_state = mmr_state;
        Ok(())
    }

    fn set_authority_set(&mut self, set: AuthoritySet) -> Result<(), BeefyClientError> {
        self.authority_set = set;
        Ok(())
    }
}

async fn get_mmr_update(
    client: &subxt::Client<subxt::DefaultConfig>,
    signed_commitment: beefy_primitives::SignedCommitment<u32, beefy_primitives::crypto::Signature>,
) -> MmrUpdateProof {
    let api =
        client.clone().to_runtime_api::<runtime::api::RuntimeApi<
            subxt::DefaultConfig,
            subxt::DefaultExtra<subxt::DefaultConfig>,
        >>();
    let subxt_block_number: subxt::BlockNumber = signed_commitment.commitment.block_number.into();
    let block_hash = client
        .rpc()
        .block_hash(Some(subxt_block_number))
        .await
        .unwrap();

    let current_authorities = api.storage().beefy().authorities(block_hash).await.unwrap();

    // Current LeafIndex
    let block_number = signed_commitment.commitment.block_number;
    let leaf_index = (block_number - 1) as u64;
    let leaf_proof: pallet_mmr_rpc::LeafProof<H256> = client
        .rpc()
        .client
        .request("mmr_generateProof", rpc_params!(leaf_index, block_hash))
        .await
        .unwrap();

    let opaque_leaf: Vec<u8> = codec::Decode::decode(&mut &*leaf_proof.leaf.0).unwrap();
    let latest_leaf: MmrLeaf<u32, H256, H256> = codec::Decode::decode(&mut &*opaque_leaf).unwrap();
    let mmr_proof: pallet_mmr_primitives::Proof<H256> =
        codec::Decode::decode(&mut &*leaf_proof.proof.0).unwrap();

    let authority_address_hashes = current_authorities
        .into_iter()
        .map(|x| {
            let id: beefy_primitives::crypto::AuthorityId =
                codec::Decode::decode(&mut &*x.encode()).unwrap();
            keccak_256(&beefy_mmr::BeefyEcdsaToEthereum::convert(id))
        })
        .collect::<Vec<_>>();

    let signatures = signed_commitment
        .signatures
        .into_iter()
        .enumerate()
        .map(|(index, x)| {
            if let Some(sig) = x {
                let mut temp = [0u8; 65];
                if sig.len() == 65 {
                    temp.copy_from_slice(&*sig.encode());
                    Some(SignatureWithAuthorityIndex {
                        index: index as u32,
                        signature: temp,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .filter_map(|x| x)
        .collect::<Vec<_>>();

    let signature_indices = signatures
        .iter()
        .map(|x| x.index as usize)
        .collect::<Vec<_>>();

    let tree = rs_merkle::MerkleTree::<KeccakHasher>::from_leaves(&authority_address_hashes);

    let authority_proof = tree.proof(&signature_indices);

    MmrUpdateProof {
        signed_commitment: SignedCommitment {
            commitment: signed_commitment.commitment.clone(),
            signatures,
        },
        latest_mmr_leaf: latest_leaf.clone(),
        mmr_proof,
        authority_proof: authority_proof.proof_hashes().to_vec(),
    }
}

#[tokio::test]
async fn test_ingest_mmr_with_proof() {
    let store = StorageMock::new();
    let mut beef_light_client = BeefyLightClient::new(store);
    let client = subxt::ClientBuilder::new()
        .set_url("ws://127.0.0.1:9944")
        .build::<subxt::DefaultConfig>()
        .await
        .unwrap();

    let mut subscription: Subscription<String> = client
        .rpc()
        .client
        .subscribe(
            "beefy_subscribeJustifications",
            rpc_params![],
            "beefy_unsubscribeJustifications",
        )
        .await
        .unwrap();

    while let Some(Ok(commitment)) = subscription.next().await {
        let recv_commitment: sp_core::Bytes =
            serde_json::from_value(JsonValue::String(commitment)).unwrap();
        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = codec::Decode::decode(&mut &*recv_commitment).unwrap();

        println!(
            "Received signed commitmment for: {:?}",
            signed_commitment.commitment.block_number
        );

        let mmr_update = get_mmr_update(&client, signed_commitment.clone()).await;

        assert_eq!(
            beef_light_client.ingest_mmr_root_with_proof(mmr_update.clone()),
            Ok(())
        );

        let mmr_state = beef_light_client.store_ref().mmr_state().unwrap();
        let authority_set = beef_light_client.store_ref().authority_set().unwrap();

        let mmr_root_hash = signed_commitment
            .commitment
            .payload
            .get_raw(&MMR_ROOT_ID)
            .unwrap();

        assert_eq!(mmr_state.mmr_root_hash.as_bytes(), &mmr_root_hash[..]);

        assert_eq!(
            mmr_state.latest_beefy_height,
            signed_commitment.commitment.block_number
        );

        assert_eq!(
            authority_set.next_authorities,
            mmr_update.latest_mmr_leaf.beefy_next_authority_set
        );

        println!(
            "\nSuccessfully ingested mmr for block number: {}\nmmr_root_hash: {}\n",
            mmr_state.latest_beefy_height,
            to_hex(&mmr_state.mmr_root_hash[..], false)
        )
    }
}

#[test]
fn should_fail_with_incomplete_signature_threshold() {
    let store = StorageMock::new();
    let mut beef_light_client = BeefyLightClient::new(store);
    let mmr_update = MmrUpdateProof {
        signed_commitment: SignedCommitment {
            commitment: beefy_primitives::Commitment {
                payload: Payload::new(MMR_ROOT_ID, vec![0u8; 32]),
                block_number: Default::default(),
                validator_set_id: 3,
            },
            signatures: vec![
                SignatureWithAuthorityIndex {
                    index: 0,
                    signature: [0u8; 65]
                };
                2
            ],
        },
        latest_mmr_leaf: MmrLeaf {
            version: Default::default(),
            parent_number_and_hash: (Default::default(), Default::default()),
            beefy_next_authority_set: BeefyNextAuthoritySet {
                id: 0,
                len: 0,
                root: Default::default(),
            },
            parachain_heads: Default::default(),
        },
        mmr_proof: Proof {
            leaf_index: 0,
            leaf_count: 0,
            items: vec![],
        },
        authority_proof: vec![],
    };

    assert_eq!(
        beef_light_client.ingest_mmr_root_with_proof(mmr_update),
        Err(BeefyClientError::IncompleteSignatureThreshold)
    );
}

#[test]
fn should_fail_with_invalid_validator_set_id() {
    let store = StorageMock::new();
    let mut beef_light_client = BeefyLightClient::new(store);

    let mmr_update = MmrUpdateProof {
        signed_commitment: SignedCommitment {
            commitment: beefy_primitives::Commitment {
                payload: Payload::new(MMR_ROOT_ID, vec![0u8; 32]),
                block_number: Default::default(),
                validator_set_id: 3,
            },
            signatures: vec![
                SignatureWithAuthorityIndex {
                    index: 0,
                    signature: [0u8; 65]
                };
                5
            ],
        },
        latest_mmr_leaf: MmrLeaf {
            version: Default::default(),
            parent_number_and_hash: (Default::default(), Default::default()),
            beefy_next_authority_set: BeefyNextAuthoritySet {
                id: 0,
                len: 0,
                root: Default::default(),
            },
            parachain_heads: Default::default(),
        },
        mmr_proof: Proof {
            leaf_index: 0,
            leaf_count: 0,
            items: vec![],
        },
        authority_proof: vec![],
    };

    assert_eq!(
        beef_light_client.ingest_mmr_root_with_proof(mmr_update),
        Err(BeefyClientError::InvalidMmrUpdate)
    );
}

#[tokio::test]
async fn verify_parachain_headers() {
    let store = StorageMock::new();
    let mut beef_light_client = BeefyLightClient::new(store);
    let client = subxt::ClientBuilder::new()
        .set_url("ws://127.0.0.1:9944")
        .build::<subxt::DefaultConfig>()
        .await
        .unwrap();
    let api =
        client.clone().to_runtime_api::<runtime::api::RuntimeApi<
            subxt::DefaultConfig,
            subxt::DefaultExtra<subxt::DefaultConfig>,
        >>();
    let mut subscription: Subscription<String> = client
        .rpc()
        .client
        .subscribe(
            "beefy_subscribeJustifications",
            rpc_params![],
            "beefy_unsubscribeJustifications",
        )
        .await
        .unwrap();

    while let Some(Ok(commitment)) = subscription.next().await {
        let recv_commitment: sp_core::Bytes =
            serde_json::from_value(JsonValue::String(commitment)).unwrap();
        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = codec::Decode::decode(&mut &*recv_commitment).unwrap();

        println!(
            "Received signed commitmment for: {:?}",
            signed_commitment.commitment.block_number
        );

        let block_number = signed_commitment.commitment.block_number;
        let subxt_block_number: subxt::BlockNumber = block_number.into();
        let block_hash = client
            .rpc()
            .block_hash(Some(subxt_block_number))
            .await
            .unwrap();

        let para_ids = api.storage().paras().parachains(block_hash).await.unwrap();
        let storage_prefix = frame_support::storage::storage_prefix(b"Paras", b"Heads");
        let mut para_header_keys = Vec::new();

        for para_id in para_ids {
            let encoded_para_id = para_id.encode();

            let mut full_key = storage_prefix.clone().to_vec();
            full_key.extend_from_slice(sp_core::hashing::twox_64(&encoded_para_id).as_slice());
            full_key.extend_from_slice(&encoded_para_id);
            para_header_keys.push(subxt::sp_core::storage::StorageKey(full_key));
        }

        let mmr_state = beef_light_client.store_ref().mmr_state().unwrap();
        let previous_finalized_block_number: subxt::BlockNumber =
            (mmr_state.latest_beefy_height + 1).into();
        let previous_finalized_hash = client
            .rpc()
            .block_hash(Some(previous_finalized_block_number))
            .await
            .unwrap()
            .unwrap();

        let change_set = client
            .storage()
            .query_storage(para_header_keys, previous_finalized_hash, block_hash)
            .await
            .unwrap();
        let mut finalized_blocks = BTreeMap::new();
        let mut leaf_indices = vec![];
        for changes in change_set {
            let header = client
                .rpc()
                .header(Some(changes.block))
                .await
                .unwrap()
                .unwrap();

            let mut heads = BTreeMap::new();

            for (key, value) in changes.changes {
                if let Some(storage_data) = value {
                    let key = key.0.to_vec();
                    let para_id = u32::decode(&mut &key[40..]).unwrap();
                    let head_data: runtime::api::runtime_types::polkadot_parachain::primitives::HeadData = Decode::decode(&mut &*storage_data.0).unwrap();
                    heads.insert(para_id, head_data.0);
                }
            }

            if !heads.contains_key(&PARA_ID) {
                continue;
            }
            finalized_blocks.insert(header.number as u64, heads);
            leaf_indices.push(header.number - 1);
        }

        let batch_proof: pallet_mmr_rpc::LeafBatchProof<H256> = client
            .rpc()
            .client
            .request(
                "mmr_generateBatchProof",
                rpc_params!(leaf_indices.clone(), block_hash),
            )
            .await
            .unwrap();

        let mut mmr_leaves_test = vec![];
        let leaves: Vec<(Vec<u8>, pallet_mmr_primitives::LeafIndex)> =
            Decode::decode(&mut &*batch_proof.leaves.to_vec()).unwrap();

        let mut parachain_headers = vec![];
        for (leaf_bytes, leaf_index) in leaves {
            let leaf: MmrLeaf<u32, H256, H256> = Decode::decode(&mut &*leaf_bytes).unwrap();
            mmr_leaves_test.push(pallet_mmr_primitives::DataOrHash::Data::<
                sp_runtime::traits::Keccak256,
                _,
            >(leaf.clone()));
            let leaf_block_number = leaf_index + 1;
            let para_headers = finalized_blocks.get(&leaf_block_number).unwrap();

            let mut index = None;
            let mut parachain_leaves = vec![];
            // Values are already sorted by key which is the para_id
            for (idx, (key, header)) in para_headers.iter().enumerate() {
                let pair = (*key, header.clone());
                let leaf_hash = keccak_256(pair.encode().as_slice());
                parachain_leaves.push(leaf_hash);
                if key == &PARA_ID {
                    index = Some(idx);
                }
            }

            let tree = rs_merkle::MerkleTree::<KeccakHasher>::from_leaves(&parachain_leaves);

            let proof = if let Some(index) = index {
                tree.proof(&[index])
                    .proof_hashes()
                    .into_iter()
                    .map(|item| item.clone())
                    .collect::<Vec<_>>()
            } else {
                vec![]
            };

            let header = ParachainHeader {
                parachain_header: para_headers.get(&PARA_ID).unwrap().clone(),
                partial_mmr_leaf: PartialMmrLeaf {
                    version: leaf.version,
                    parent_number_and_hash: leaf.parent_number_and_hash,
                    beefy_next_authority_set: leaf.beefy_next_authority_set.clone(),
                },
                para_id: PARA_ID,
                parachain_heads_proof: proof,
                heads_leaf_index: index.unwrap() as u32,
                heads_total_count: parachain_leaves.len() as u32,
                extrinsic_proof: vec![],
            };

            parachain_headers.push(header);
        }

        let batch_proof: pallet_mmr_primitives::BatchProof<H256> =
            Decode::decode(&mut batch_proof.proof.0.as_slice()).unwrap();
        let parachain_update_proof = ParachainsUpdateProof {
            parachain_headers,
            mmr_proof: batch_proof,
        };

        let mmr_update = get_mmr_update(&client, signed_commitment).await;
        assert_ok!(beef_light_client.ingest_mmr_root_with_proof(mmr_update));
        assert_ok!(beef_light_client.verify_parachain_headers(parachain_update_proof));

        let mmr_state = beef_light_client.store_ref().mmr_state().unwrap();

        println!(
            "\nSuccessfully verified parachain headers for block number: {}\nmmr_root_hash: {}\n",
            mmr_state.latest_beefy_height,
            to_hex(&mmr_state.mmr_root_hash[..], false)
        );
    }
}
