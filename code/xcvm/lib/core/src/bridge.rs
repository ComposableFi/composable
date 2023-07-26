use crate::prelude::*;

use crate::{NetworkId, UserOrigin};

/// The Origin that executed the XCVM operation.
/// Origin was verified to satisfy security semantics for execution.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum CallOrigin {
	Remote { user_origin: UserOrigin },
	Local { user: Addr },
}

impl CallOrigin {
	/// Extract the user from a [`CallOrigin`].
	/// No distinction is done for local or remote user in this case.
	pub fn user(&self, current_network: NetworkId) -> UserOrigin {
		match self {
			CallOrigin::Remote { user_origin } => user_origin.clone(),
			CallOrigin::Local { user } =>
				UserOrigin { network_id: current_network, user_id: user.as_bytes().to_vec().into() },
		}
	}
}
