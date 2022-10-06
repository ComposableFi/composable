// Copyright (C) 2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use alloc::collections::{BTreeMap, BTreeSet};
use anyhow::anyhow;
use codec::{Decode, Encode};
use finality_grandpa::voter_set::VoterSet;
use primitives::{error, Commit, HostFunctions};
use sp_finality_grandpa::{
	AuthorityId, AuthorityList, AuthoritySignature, ConsensusLog, Equivocation, RoundNumber,
	ScheduledChange, SetId, GRANDPA_ENGINE_ID,
};
use sp_runtime::{generic::OpaqueDigestItemId, traits::Header as HeaderT};
use sp_std::prelude::*;

/// A GRANDPA justification for block finality, it includes a commit message and
/// an ancestry proof including all headers routing all precommit target blocks
/// to the commit target block. Due to the current voting strategy the precommit
/// targets should be the same as the commit target, since honest voters don't
/// vote past authority set change blocks.
///
/// This is meant to be stored in the db and passed around the network to other
/// nodes, and are used by syncing nodes to prove authority set handoffs.
#[cfg_attr(any(feature = "std", test), derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq, Eq)]
pub struct GrandpaJustification<H: HeaderT> {
	pub round: u64,
	pub commit: Commit<H>,
	pub votes_ancestries: Vec<H>,
}

impl<H> GrandpaJustification<H>
where
	H: HeaderT,
	H::Number: finality_grandpa::BlockNumberOps,
{
	/// Validate the commit and the votes' ancestry proofs.
	pub fn verify<Host>(&self, set_id: u64, authorities: &AuthorityList) -> Result<(), error::Error>
	where
		Host: HostFunctions,
	{
		let voters =
			VoterSet::new(authorities.iter().cloned()).ok_or(anyhow!("Invalid AuthoritiesSet"))?;

		self.verify_with_voter_set::<Host>(set_id, &voters)
	}

	/// Validate the commit and the votes' ancestry proofs.
	pub(crate) fn verify_with_voter_set<Host>(
		&self,
		set_id: u64,
		voters: &VoterSet<AuthorityId>,
	) -> Result<(), error::Error>
	where
		Host: HostFunctions,
	{
		use finality_grandpa::Chain;

		let ancestry_chain = AncestryChain::<H>::new(&self.votes_ancestries);

		match finality_grandpa::validate_commit(&self.commit, voters, &ancestry_chain) {
			Ok(ref result) if result.is_valid() => {},
			err => {
				let result = err?;
				Err(anyhow!("invalid commit in grandpa justification: {result:?}"))?
			},
		}

		// we pick the precommit for the lowest block as the base that
		// should serve as the root block for populating ancestry (i.e.
		// collect all headers from all precommit blocks to the base)
		let base_hash = self
			.commit
			.precommits
			.iter()
			.map(|signed| &signed.precommit)
			.min_by_key(|precommit| precommit.target_number)
			.map(|precommit| precommit.target_hash.clone())
			.expect(
				"can only fail if precommits is empty; \
				 commit has been validated above; \
				 valid commits must include precommits; \
				 qed.",
			);

		let mut visited_hashes = BTreeSet::new();
		for signed in self.commit.precommits.iter() {
			let message = finality_grandpa::Message::Precommit(signed.precommit.clone());
			check_message_signature::<Host, _, _>(
				&message,
				&signed.id,
				&signed.signature,
				self.round,
				set_id,
			)?;

			if base_hash == signed.precommit.target_hash {
				continue
			}

			let route = ancestry_chain.ancestry(base_hash, signed.precommit.target_hash)?;
			// ancestry starts from parent hash but the precommit target hash has been
			// visited
			visited_hashes.insert(signed.precommit.target_hash);
			for hash in route {
				visited_hashes.insert(hash);
			}
		}

		let ancestry_hashes: BTreeSet<_> =
			self.votes_ancestries.iter().map(|h: &H| h.hash()).collect();

		if visited_hashes != ancestry_hashes {
			Err(anyhow!(
				"invalid precommit ancestries in grandpa justification with unused headers",
			))?
		}

		Ok(())
	}

	/// The target block number and hash that this justifications proves finality for.
	pub fn target(&self) -> (H::Number, H::Hash) {
		(self.commit.target_number, self.commit.target_hash)
	}
}

/// A utility trait implementing `finality_grandpa::Chain` using a given set of headers.
/// This is useful when validating commits, using the given set of headers to
/// verify a valid ancestry route to the target commit block.
pub struct AncestryChain<H: HeaderT> {
	ancestry: BTreeMap<H::Hash, H>,
}

impl<H: HeaderT> AncestryChain<H> {
	pub fn new(ancestry: &[H]) -> AncestryChain<H> {
		let ancestry: BTreeMap<_, _> = ancestry.iter().cloned().map(|h: H| (h.hash(), h)).collect();

		AncestryChain { ancestry }
	}

	pub fn header(&self, hash: &H::Hash) -> Option<&H> {
		self.ancestry.get(hash)
	}
}

impl<H: HeaderT> finality_grandpa::Chain<H::Hash, H::Number> for AncestryChain<H>
where
	H::Number: finality_grandpa::BlockNumberOps,
{
	fn ancestry(
		&self,
		base: H::Hash,
		block: H::Hash,
	) -> Result<Vec<H::Hash>, finality_grandpa::Error> {
		let mut route = Vec::new();
		let mut current_hash = block;
		loop {
			if current_hash == base {
				break
			}
			match self.ancestry.get(&current_hash) {
				Some(current_header) => {
					current_hash = *current_header.parent_hash();
					route.push(current_hash);
				},
				_ => return Err(finality_grandpa::Error::NotDescendent),
			}
		}
		route.pop(); // remove the base

		Ok(route)
	}
}

/// Checks the given header for a consensus digest signalling a **standard** scheduled change and
/// extracts it.
pub fn find_scheduled_change<H: HeaderT>(header: &H) -> Option<ScheduledChange<H::Number>> {
	let id = OpaqueDigestItemId::Consensus(&GRANDPA_ENGINE_ID);

	let filter_log = |log: ConsensusLog<H::Number>| match log {
		ConsensusLog::ScheduledChange(change) => Some(change),
		_ => None,
	};

	// find the first consensus digest with the right ID which converts to
	// the right kind of consensus log.
	header.digest().convert_first(|l| l.try_to(id).and_then(filter_log))
}

/// Checks the given header for a consensus digest signalling a **forced** scheduled change and
/// extracts it.
pub fn find_forced_change<H: HeaderT>(
	header: &H,
) -> Option<(H::Number, ScheduledChange<H::Number>)> {
	let id = OpaqueDigestItemId::Consensus(&GRANDPA_ENGINE_ID);

	let filter_log = |log: ConsensusLog<H::Number>| match log {
		ConsensusLog::ForcedChange(delay, change) => Some((delay, change)),
		_ => None,
	};

	// find the first consensus digest with the right ID which converts to
	// the right kind of consensus log.
	header.digest().convert_first(|l| l.try_to(id).and_then(filter_log))
}

/// Check a message signature by encoding the message and verifying the provided signature using the
/// expected authority id.
pub fn check_message_signature<Host, H, N>(
	message: &finality_grandpa::Message<H, N>,
	id: &AuthorityId,
	signature: &AuthoritySignature,
	round: RoundNumber,
	set_id: SetId,
) -> Result<(), anyhow::Error>
where
	Host: HostFunctions,
	H: Encode,
	N: Encode,
{
	let buf = (message, round, set_id).encode();

	if !Host::ed25519_verify(signature.as_ref(), &buf, id.as_ref()) {
		Err(anyhow!("invalid signature for precommit in grandpa justification"))?
	}

	Ok(())
}

/// Verifies the equivocation proof by making sure that both votes target
/// different blocks and that its signatures are valid.
pub fn check_equivocation_proof<Host, H, N>(
	set_id: u64,
	equivocation: Equivocation<H, N>,
) -> Result<(), anyhow::Error>
where
	Host: HostFunctions,
	H: Clone + Encode + PartialEq,
	N: Clone + Encode + PartialEq,
{
	// NOTE: the bare `Prevote` and `Precommit` types don't share any trait,
	// this is implemented as a macro to avoid duplication.
	macro_rules! check {
		( $equivocation:expr, $message:expr ) => {
			// if both votes have the same target the equivocation is invalid.
			if $equivocation.first.0.target_hash == $equivocation.second.0.target_hash &&
				$equivocation.first.0.target_number == $equivocation.second.0.target_number
			{
				return Err(anyhow!("both votes have the same target!"))
			}

			// check signatures on both votes are valid
			check_message_signature::<Host, _, _>(
				&$message($equivocation.first.0),
				&$equivocation.identity,
				&$equivocation.first.1,
				$equivocation.round_number,
				set_id,
			)?;

			check_message_signature::<Host, _, _>(
				&$message($equivocation.second.0),
				&$equivocation.identity,
				&$equivocation.second.1,
				$equivocation.round_number,
				set_id,
			)?;

			return Ok(())
		};
	}

	match equivocation {
		Equivocation::Prevote(equivocation) => {
			check!(equivocation, finality_grandpa::Message::Prevote);
		},
		Equivocation::Precommit(equivocation) => {
			check!(equivocation, finality_grandpa::Message::Precommit);
		},
	}
}
