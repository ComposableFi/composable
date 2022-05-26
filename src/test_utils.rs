use crate::primitives::{ParachainHeader, PartialMmrLeaf, SignedCommitment};
use crate::traits::{ClientState, HostFunctions};
use crate::{runtime, MerkleHasher, MmrUpdateProof, SignatureWithAuthorityIndex};
use beefy_primitives::mmr::{BeefyNextAuthoritySet, MmrLeaf};
use codec::{Decode, Encode};
use hex_literal::hex;
use pallet_mmr_primitives::BatchProof;
use sp_core::H256;
use sp_io::crypto;
use sp_runtime::generic::Header;
use sp_runtime::traits::{BlakeTwo256, Convert};
use sp_trie::{generate_trie_proof, TrieDBMut, TrieMut};
use std::collections::BTreeMap;
use subxt::rpc::rpc_params;
use subxt::rpc::ClientT;
use subxt::sp_core::keccak_256;

pub const PARA_ID: u32 = 2000;

#[derive(Clone)]
pub struct Crypto;

impl HostFunctions for Crypto {
    fn keccak_256(input: &[u8]) -> [u8; 32] {
        keccak_256(input)
    }

    fn secp256k1_ecdsa_recover_compressed(
        signature: &[u8; 65],
        value: &[u8; 32],
    ) -> Option<Vec<u8>> {
        crypto::secp256k1_ecdsa_recover_compressed(signature, value)
            .ok()
            .map(|val| val.to_vec())
    }
}

pub async fn get_initial_client_state(
    api: Option<
        &runtime::api::RuntimeApi<
            subxt::DefaultConfig,
            subxt::PolkadotExtrinsicParams<subxt::DefaultConfig>,
        >,
    >,
) -> ClientState {
    if api.is_none() {
        return ClientState {
            latest_beefy_height: 0,
            mmr_root_hash: Default::default(),
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
            beefy_activation_block: 0,
        };
    }
    // Get initial validator set
    // In development mode validators are the same for all sessions only validator set_id changes
    let api = api.unwrap();
    let validator_set_id = api.storage().beefy().validator_set_id(None).await.unwrap();
    let next_val_set = api
        .storage()
        .mmr_leaf()
        .beefy_next_authorities(None)
        .await
        .unwrap();
    ClientState {
        latest_beefy_height: 0,
        mmr_root_hash: Default::default(),
        current_authorities: BeefyNextAuthoritySet {
            id: validator_set_id,
            len: next_val_set.len,
            root: next_val_set.root,
        },
        next_authorities: BeefyNextAuthoritySet {
            id: validator_set_id + 1,
            len: next_val_set.len,
            root: next_val_set.root,
        },
        beefy_activation_block: 0,
    }
}

pub async fn get_mmr_update(
    client: &subxt::Client<subxt::DefaultConfig>,
    signed_commitment: beefy_primitives::SignedCommitment<u32, beefy_primitives::crypto::Signature>,
) -> MmrUpdateProof {
    let api =
        client.clone().to_runtime_api::<runtime::api::RuntimeApi<
            subxt::DefaultConfig,
            subxt::PolkadotExtrinsicParams<_>,
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
    let latest_leaf: MmrLeaf<u32, H256, H256, H256> =
        codec::Decode::decode(&mut &*opaque_leaf).unwrap();
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

    let tree =
        rs_merkle::MerkleTree::<MerkleHasher<Crypto>>::from_leaves(&authority_address_hashes);

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

pub async fn get_parachain_headers(
    client: &subxt::Client<subxt::DefaultConfig>,
    para_client: &subxt::Client<subxt::DefaultConfig>,
    commitment_block_number: u32,
    latest_beefy_height: u32,
) -> (Vec<ParachainHeader>, BatchProof<H256>) {
    let subxt_block_number: subxt::BlockNumber = commitment_block_number.into();
    let block_hash = client
        .rpc()
        .block_hash(Some(subxt_block_number))
        .await
        .unwrap();

    let api =
        client.clone().to_runtime_api::<runtime::api::RuntimeApi<
            subxt::DefaultConfig,
            subxt::PolkadotExtrinsicParams<_>,
        >>();

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

    let previous_finalized_block_number: subxt::BlockNumber = (latest_beefy_height + 1).into();
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

    let leaves: Vec<Vec<u8>> = Decode::decode(&mut &*batch_proof.leaves.to_vec()).unwrap();

    let mut parachain_headers = vec![];
    for leaf_bytes in leaves {
        let leaf: MmrLeaf<u32, H256, H256, H256> = Decode::decode(&mut &*leaf_bytes).unwrap();
        let leaf_block_number = (leaf.parent_number_and_hash.0 + 1) as u64;
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

        let tree = rs_merkle::MerkleTree::<MerkleHasher<Crypto>>::from_leaves(&parachain_leaves);

        let proof = if let Some(index) = index {
            tree.proof(&[index])
                .proof_hashes()
                .into_iter()
                .map(|item| item.clone())
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

        let para_head = para_headers.get(&PARA_ID).unwrap().clone();
        let decoded_para_head = Header::<u32, BlakeTwo256>::decode(&mut &*para_head).unwrap();

        let block_number = decoded_para_head.number;
        let subxt_block_number: subxt::BlockNumber = block_number.into();
        let block_hash = para_client
            .rpc()
            .block_hash(Some(subxt_block_number))
            .await
            .unwrap();

        let block = para_client.rpc().block(block_hash).await.unwrap().unwrap();
        let extrinsics = block
            .block
            .extrinsics
            .into_iter()
            .map(|e| e.encode())
            .collect::<Vec<_>>();

        let (timestamp_extrinsic, extrinsic_proof) = {
            if extrinsics.is_empty() {
                (vec![], vec![])
            } else {
                let timestamp_ext = extrinsics[0].clone();

                let mut db = sp_trie::MemoryDB::<BlakeTwo256>::default();

                let root = {
                    let mut root = Default::default();
                    let mut trie =
                        <TrieDBMut<sp_trie::LayoutV0<BlakeTwo256>>>::new(&mut db, &mut root);

                    for (i, ext) in extrinsics.into_iter().enumerate() {
                        let key = codec::Compact(i as u32).encode();
                        trie.insert(&key, &ext).unwrap();
                    }
                    *trie.root()
                };

                let key = codec::Compact::<u32>(0u32).encode();
                let extrinsic_proof =
                    generate_trie_proof::<sp_trie::LayoutV0<BlakeTwo256>, _, _, _>(
                        &db,
                        root,
                        vec![&key],
                    )
                    .unwrap();
                (timestamp_ext, extrinsic_proof)
            }
        };

        let header = ParachainHeader {
            parachain_header: para_head,
            partial_mmr_leaf: PartialMmrLeaf {
                version: leaf.version,
                parent_number_and_hash: leaf.parent_number_and_hash,
                beefy_next_authority_set: leaf.beefy_next_authority_set.clone(),
            },
            para_id: PARA_ID,
            parachain_heads_proof: proof,
            heads_leaf_index: index.unwrap() as u32,
            heads_total_count: parachain_leaves.len() as u32,
            extrinsic_proof,
            timestamp_extrinsic,
        };

        parachain_headers.push(header);
    }

    let batch_proof: pallet_mmr_primitives::BatchProof<H256> =
        Decode::decode(&mut batch_proof.proof.0.as_slice()).unwrap();
    (parachain_headers, batch_proof)
}
