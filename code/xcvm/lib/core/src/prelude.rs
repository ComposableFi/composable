pub use alloc::{
	boxed::Box,
	collections::VecDeque,
	string::{String, ToString},
	vec,
	vec::Vec,
};
pub use core::str::FromStr;
pub use cosmwasm_std::{Addr, Binary, Coin, Uint128};
pub use serde::{Deserialize, Serialize};

pub use parity_scale_codec::{Decode, Encode};

#[cfg(feature = "std")]
pub use cosmwasm_schema::{cw_serde, QueryResponses};

#[cfg(feature = "std")]
pub use schemars::JsonSchema;

use core::fmt::{Display, Error as FmtError, Formatter};

// https://github.com/cosmos/ibc-rs/issues/800#issuecomment-1659494043
// pub use ibc_rs_scale::applications::transfer::PrefixedDenom;

use ibc_rs_scale::{
	applications::transfer::{error::TokenTransferError, BaseDenom, TracePath, TracePrefix},
	core::ics24_host::identifier::{ChannelId, PortId},
};

/// A type that contains the base denomination for ICS20 and the source tracing information path.
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
	parity_scale_codec::Encode,
	parity_scale_codec::Decode,
	scale_info::TypeInfo,
	Clone,
	Debug,
	Eq,
	PartialEq,
	PartialOrd,
	Ord,
)]
pub struct PrefixedDenom {
	/// A series of `{port-id}/{channel-id}`s for tracing the source of the token.
	#[cfg_attr(feature = "serde", serde(with = "serde_string"))]
	#[cfg_attr(feature = "schema", schemars(with = "String"))]
	pub trace_path: TracePath,
	/// Base denomination of the relayed fungible token.
	pub base_denom: BaseDenom,
}

impl PrefixedDenom {
	/// Removes the specified prefix from the trace path if there is a match, otherwise does
	/// nothing.
	pub fn remove_trace_prefix(&mut self, prefix: &TracePrefix) {
		self.trace_path.remove_prefix(prefix)
	}

	/// Adds the specified prefix to the trace path.
	pub fn add_trace_prefix(&mut self, prefix: TracePrefix) {
		self.trace_path.add_prefix(prefix)
	}
}

/// Returns true if the denomination originally came from the sender chain and
/// false otherwise.
///
/// Note: It is better to think of the "source" chain as the chain that
/// escrows/unescrows the token, while the other chain mints/burns the tokens,
/// respectively. A chain being the "source" of a token does NOT mean it is the
/// original creator of the token (e.g. "uatom"), as "source" might suggest.
///
/// This means that in any given transfer, a chain can very well be the source
/// of a token of which it is not the creator. For example, let
///
/// A: sender chain in this transfer, port "transfer" and channel "c2b" (to B)
/// B: receiver chain in this transfer, port "transfer" and channel "c2a" (to A)
/// token denom: "transfer/someOtherChannel/someDenom"
///
/// A, initiator of the transfer, needs to figure out if it should escrow the
/// tokens, or burn them. If B had originally sent the token to A in a previous
/// transfer, then A would have stored the token as "transfer/c2b/someDenom".
/// Now, A is sending to B, so to check if B is the source of the token, we need
/// to check if the token starts with "transfer/c2b". In this example, it
/// doesn't, so the token doesn't originate from B. A is considered the source,
/// even though it is not the creator of the token. Specifically, the token was
/// created by the chain at the other end of A's port "transfer" and channel
/// "someOtherChannel".
pub fn is_sender_chain_source(
	source_port: PortId,
	source_channel: ChannelId,
	denom: &PrefixedDenom,
) -> bool {
	!is_receiver_chain_source(source_port, source_channel, denom)
}

/// Returns true if the denomination originally came from the receiving chain and false otherwise.
pub fn is_receiver_chain_source(
	source_port: PortId,
	source_channel: ChannelId,
	denom: &PrefixedDenom,
) -> bool {
	// For example, let
	// A: sender chain in this transfer, port "transfer" and channel "c2b" (to B)
	// B: receiver chain in this transfer, port "transfer" and channel "c2a" (to A)
	//
	// If B had originally sent the token in a previous transfer, then A would have stored the token
	// as "transfer/c2b/{token_denom}". Now, A is sending to B, so to check if B is the source of
	// the token, we need to check if the token starts with "transfer/c2b".
	let prefix = TracePrefix::new(source_port, source_channel);
	denom.trace_path.starts_with(&prefix)
}

impl FromStr for PrefixedDenom {
	type Err = TokenTransferError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts: Vec<&str> = s.split('/').collect();
		let last_part = parts.pop().expect("split() returned an empty iterator");

		let (base_denom, trace_path) = {
			if last_part == s {
				(BaseDenom::from_str(s)?, TracePath::default())
			} else {
				let base_denom = BaseDenom::from_str(last_part)?;
				let trace_path = TracePath::try_from(parts)?;
				(base_denom, trace_path)
			}
		};

		Ok(Self { trace_path, base_denom })
	}
}

impl From<BaseDenom> for PrefixedDenom {
	fn from(denom: BaseDenom) -> Self {
		Self { trace_path: Default::default(), base_denom: denom }
	}
}

impl Display for PrefixedDenom {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
		if self.trace_path.is_empty() {
			write!(f, "{}", self.base_denom)
		} else {
			write!(f, "{}/{}", self.trace_path, self.base_denom)
		}
	}
}
