use std::ops::Deref;

use cosmwasm_schema::cw_serde;

pub struct AnyClientMessage;
pub struct AnyClientState;
pub struct AnyConsensusState;
pub struct AnyClient;

#[cw_serde]
pub struct Height {
	/// Previously known as "epoch"
	pub revision_number: u64,

	/// The height of a block
	pub revision_height: u64,
}

impl Deref for Height {
	type Target = ibc::Height;
	fn deref(&self) -> &Self::Target {
		&ibc::Height {
			revision_number: self.revision_number,
			revision_height: self.revision_height,
		}
	}
}
