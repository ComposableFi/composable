use crate::{NetworkId, UserOrigin};

use cosmwasm_std::Addr;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

/// Protocol used to bridge call/funds.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(
	Clone, PartialEq, Eq, PartialOrd, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize,
)]
pub enum BridgeProtocol { IBC }

/// The Origin that executed the XCVM operation.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum CallOrigin {
	Remote { protocol: BridgeProtocol, relayer: Addr, user_origin: UserOrigin },
	Local { user: Addr },
}

impl CallOrigin {
	/// Extract the user from a [`CallOrigin`].
	/// No distinction is done for local or remote user in this case.
	pub fn user(&self, current_network: NetworkId) -> UserOrigin {
		match self {
			CallOrigin::Remote { user_origin, .. } => user_origin.clone(),
			CallOrigin::Local { user } =>
				UserOrigin { network_id: current_network, user_id: user.as_bytes().to_vec().into() },
		}
	}

	/// The relayer.
	pub fn relayer(&self) -> &Addr {
		match self {
			CallOrigin::Remote { relayer, .. } => relayer,
			CallOrigin::Local { user } => user,
		}
	}
}
