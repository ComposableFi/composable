use crate::{
    AuthoritySet, BeefyClientError, KeccakHasher, MmrState, StorageRead, StorageWrite, H256,
};
use beefy_primitives::mmr::BeefyNextAuthoritySet;
use codec::Encode;
use hex_literal::hex;
use jsonrpc_core::{types::Params, Notification};
use rs_merkle::{MerkleProof, MerkleTree};
use sp_runtime::traits::Convert;
use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};
use subxt::sp_core::keccak_256;
use subxt::{
    rpc::{rpc_params, JsonValue, Subscription, SubscriptionClientT},
    sp_core::storage::{StorageData, StorageKey},
};

pub struct StorageMock {
    mmr_state: MmrState,
    authority_set: AuthoritySet,
}
struct Storage;

const PARA_ID: u32 = 2000;

std::thread_local! {
    pub static MOCK_STORAGE: Arc<Mutex<StorageMock>> = Arc::new(Mutex::new(StorageMock{
        mmr_state: MmrState { latest_beefy_height: 0, mmr_root_hash: Default::default() },
        authority_set: AuthoritySet { current_authorities: BeefyNextAuthoritySet{
            id: 0,
            len: 5,
            root: H256::from(hex!("baa93c7834125ee3120bac6e3342bd3f28611110ad21ab6075367abdffefeb09"))
        }, next_authorities: BeefyNextAuthoritySet{
            id: 0,
            len: 5,
            root: H256::from(hex!("baa93c7834125ee3120bac6e3342bd3f28611110ad21ab6075367abdffefeb09"))
        } }
    }));
}

impl StorageRead for Storage {
    fn mmr_state() -> Result<MmrState, BeefyClientError> {
        Ok(MOCK_STORAGE.with(|mock| mock.lock().unwrap().mmr_state.clone()))
    }

    fn authority_set() -> Result<AuthoritySet, BeefyClientError> {
        Ok(MOCK_STORAGE.with(|mock| mock.lock().unwrap().authority_set.clone()))
    }
}

impl StorageWrite for Storage {
    fn set_mmr_state(mmr_state: MmrState) -> Result<(), BeefyClientError> {
        Ok(MOCK_STORAGE.with(move |mock| {
            let mut inner = mock.lock().unwrap();
            inner.mmr_state = mmr_state;
        }))
    }

    fn set_authority_set(set: AuthoritySet) -> Result<(), BeefyClientError> {
        Ok(MOCK_STORAGE.with(move |mock| {
            let mut inner = mock.lock().unwrap();
            inner.authority_set = set;
        }))
    }
}

#[tokio::test]
async fn test_ingest_mmr_with_proof() {
    let client = subxt::ClientBuilder::new()
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

    for res in subscription.next().await {
        if let Ok(recv) = res {
            let recv_commitment: sp_core::Bytes =
                serde_json::from_value(JsonValue::String(recv)).unwrap();
            let signed_commitment: beefy_primitives::SignedCommitment<
                u32,
                beefy_primitives::crypto::Signature,
            > = codec::Decode::decode(&mut &*recv_commitment).unwrap();

            let block_number: subxt::BlockNumber = signed_commitment.commitment.block_number.into();
            let block_hash = client.rpc().block_hash(Some(block_number)).await.unwrap();
            let current_authorities_key = StorageKey(
                hex!("08c41974a97dbf15cfbec28365bea2da5e0621c4869aa60c02be9adcc98a0d1d").to_vec(),
            );
            let next_authorities_key = StorageKey(
                hex!("08c41974a97dbf15cfbec28365bea2daaacf00b9b41fda7a9268821c2a2b3e4c").to_vec(),
            );
            let validator_set_id_key = StorageKey(
                hex!("08c41974a97dbf15cfbec28365bea2da8f05bccc2f70ec66a32999c5761156be").to_vec(),
            );
            let parachain_ids_key = StorageKey(
                hex!("cd710b30bd2eab0352ddcc26417aa1940b76934f4cc08dee01012d059e1b83ee").to_vec(),
            );
            let current_authorities = client
                .rpc()
                .storage(&current_authorities_key, block_hash)
                .await
                .unwrap()
                .unwrap();
            let next_authorities = client
                .rpc()
                .storage(&next_authorities_key, block_hash)
                .await
                .unwrap()
                .unwrap();
            let validator_set_id = client
                .rpc()
                .storage(&validator_set_id_key, block_hash)
                .await
                .unwrap()
                .unwrap();
            let parachain_ids = client
                .rpc()
                .storage(&parachain_ids_key, block_hash)
                .await
                .unwrap()
                .unwrap();

            let validator_set_id: u64 = codec::Decode::decode(&mut &*validator_set_id.0).unwrap();

            let parachain_ids: Vec<u32> = codec::Decode::decode(&mut &*parachain_ids.0).unwrap();

            let authority_ids: Vec<beefy_primitives::crypto::AuthorityId> =
                codec::Decode::decode(&mut &*current_authorities.0).unwrap();
            let authority_indices = authority_ids
                .iter()
                .enumerate()
                .map(|x| x.0)
                .collect::<Vec<_>>();
            let next_authority_ids: Vec<beefy_primitives::crypto::AuthorityId> =
                codec::Decode::decode(&mut &*next_authorities.0).unwrap();

            let authority_address_hashes = authority_ids
                .into_iter()
                .map(|x| keccak_256(&beefy_mmr::BeefyEcdsaToEthereum::convert(x)))
                .collect::<Vec<_>>();

            let next_authority_hashes = next_authority_ids
                .into_iter()
                .map(|x| keccak_256(&beefy_mmr::BeefyEcdsaToEthereum::convert(x)))
                .collect::<Vec<_>>();
            let tree = MerkleTree::<KeccakHasher>::from_leaves(&authority_address_hashes);

            let authority_proof = tree.proof(&authority_indices);

            let next_authorities_root =
                MerkleTree::<KeccakHasher>::from_leaves(&next_authority_hashes)
                    .root()
                    .unwrap();

            // for para_id in parachain_ids {
            //
            // }

            // let leaf = beefy_primitives::mmr::MmrLeaf {
            //     version: beefy_primitives::mmr::MmrLeafVersion::new(0,0),
            //     parent_number_and_hash: ((), ()),
            //     beefy_next_authority_set: BeefyNextAuthoritySet {
            //         id: validator_set_id + 1,
            //         len: next_authority_ids.len() as u32,
            //         root: H256::from(next_authorities_root.as_slice())
            //     },
            //     parachain_heads: ()
            // };
        }
    }
}
