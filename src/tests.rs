use crate::primitives::{MmrLeafWithIndex, SignedCommitment};
use crate::{runtime, BeefyLightClient, KeccakHasher, MmrUpdateProof};
use crate::{AuthoritySet, BeefyClientError, MmrState, StorageRead, StorageWrite, H256};
use beefy_primitives::mmr::{BeefyNextAuthoritySet, MmrLeaf};
use codec::Encode;
use hex_literal::hex;
use sp_runtime::traits::Convert;
use subxt::rpc::ClientT;
use subxt::rpc::{rpc_params, JsonValue, Subscription, SubscriptionClientT};
use subxt::sp_core::keccak_256;

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
// Run test against rococo-local chain of polkadot release-v0.9.16
// If a different node is used, rebuild the runtime types using subxt cli codegen
#[tokio::test]
async fn test_ingest_mmr_with_proof() {
    let store = StorageMock::new();
    let mut beef_light_client = BeefyLightClient::new(store);
    let client = subxt::ClientBuilder::new()
        .set_url("ws://127.0.0.1:9978")
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

    for res in subscription.next().await {
        if let Ok(recv) = res {
            println!("Received signed commitmment");
            let recv_commitment: sp_core::Bytes =
                serde_json::from_value(JsonValue::String(recv)).unwrap();
            let signed_commitment: beefy_primitives::SignedCommitment<
                u32,
                beefy_primitives::crypto::Signature,
            > = codec::Decode::decode(&mut &*recv_commitment).unwrap();

            let subxt_block_number: subxt::BlockNumber =
                signed_commitment.commitment.block_number.into();
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
            let latest_leaf: MmrLeaf<u32, H256, H256> =
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
                .map(|x| {
                    if let Some(sig) = x {
                        let mut temp = [0u8; 65];
                        if sig.len() == 65 {
                            temp.copy_from_slice(&*sig.encode());
                            Some(temp)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let signature_indices = signatures
                .iter()
                .enumerate()
                .filter_map(|x| if x.1.is_some() { Some(x.0) } else { None })
                .collect::<Vec<_>>();

            let tree =
                rs_merkle::MerkleTree::<KeccakHasher>::from_leaves(&authority_address_hashes);

            let authority_proof = tree.proof(&signature_indices);

            let mmr_update = MmrUpdateProof {
                signed_commitment: SignedCommitment {
                    commitment: signed_commitment.commitment,
                    signatures,
                },
                latest_mmr_leaf_with_index: MmrLeafWithIndex {
                    leaf: latest_leaf,
                    index: leaf_index,
                },
                mmr_proof,
                authority_proof: authority_proof.proof_hashes().to_vec(),
            };

            assert_eq!(
                beef_light_client.ingest_mmr_root_with_proof(mmr_update),
                Ok(())
            );
        }
    }
}
