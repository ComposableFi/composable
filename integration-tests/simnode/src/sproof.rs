use codec::Encode;
use common::OpaqueBlock;
use parachain_inherent::ParachainInherentData;
use polkadot_primitives::v1::PersistedValidationData;
use sp_blockchain::HeaderBackend;
use sp_runtime::generic::BlockId;
use sproof_builder::RelayStateSproofBuilder;
use std::sync::Arc;

pub struct ParachainInherentSproof<C> {
	client: Arc<C>,
}

impl<C> ParachainInherentSproof<C>
where
	C: HeaderBackend<OpaqueBlock>,
{
	pub fn new(client: Arc<C>) -> Self {
		ParachainInherentSproof { client }
	}

	pub fn create_inherent(&mut self, slot: u64) -> ParachainInherentData {
		let mut sproof = RelayStateSproofBuilder::default();
		sproof.current_slot = slot.into();
		sproof.para_id = 2087.into();
		sproof.host_config.validation_upgrade_delay = 1;

		let info = self.client.info();
		let header = self.client.header(BlockId::Hash(info.best_hash)).unwrap().unwrap().encode();

		let (state_root, proof) = sproof.into_state_root_and_proof();

		ParachainInherentData {
			validation_data: PersistedValidationData {
				parent_head: header.into(),
				relay_parent_number: info.best_number * 100,
				relay_parent_storage_root: state_root,
				max_pov_size: 15 * 1024 * 1024,
			},
			relay_chain_state: proof,
			downward_messages: Default::default(),
			horizontal_messages: Default::default(),
		}
	}
}
