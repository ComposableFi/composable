use super::*;

use crate::{
	ics23::{client_states::ClientStates, clients::Clients, consensus_states::ConsensusStates},
	impls::host_height,
	light_clients::{AnyClient, AnyClientMessage, AnyClientState, AnyConsensusState},
	routing::Context,
};

use alloc::string::{String, ToString};
use frame_support::traits::Get;
use ibc::{
	core::{
		ics02_client::{
			client_consensus::ConsensusState,
			client_state::{ClientState, ClientType},
			context::{ClientKeeper, ClientReader, ClientTypes},
			error::Error as ICS02Error,
		},
		ics24_host::identifier::ClientId,
	},
	timestamp::Timestamp,
	Height,
};
use sp_runtime::SaturatedConversion;
use tendermint_proto::Protobuf;

#[derive(Encode, Decode)]
pub struct HostConsensusProof {
	pub header: Vec<u8>,
	pub extrinsic: Vec<u8>,
	pub extrinsic_proof: Vec<Vec<u8>>,
}

impl<T: Config + Send + Sync> ClientReader for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
	<T as frame_system::Config>::BlockNumber: From<u32>,
{
	fn client_type(&self, client_id: &ClientId) -> Result<ClientType, ICS02Error> {
		log::trace!(target: "pallet_ibc", "in client : [client_type] >> client_id = {:?}", client_id);

		if <Clients<T>>::contains_key(client_id) {
			let data = <Clients<T>>::get(client_id)
				.ok_or_else(|| ICS02Error::client_not_found(client_id.clone()))?;
			let data = String::from_utf8(data).map_err(|e| {
				ICS02Error::implementation_specific(format!(
					"[client_type]: error decoding client type bytes to string {}",
					e
				))
			})?;
			// todo: &'static str was a bad idea
			Ok(data)
		} else {
			log::trace!(target: "pallet_ibc", "in client : [client_type] >> read client_type is None");
			Err(ICS02Error::client_not_found(client_id.clone()))
		}
	}

	fn client_state(&self, client_id: &ClientId) -> Result<AnyClientState, ICS02Error> {
		log::trace!(target: "pallet_ibc", "in client : [client_state] >> client_id = {:?}", client_id);
		let data = <ClientStates<T>>::get(client_id)
			.ok_or_else(|| ICS02Error::client_not_found(client_id.clone()))?;
		let state = AnyClientState::decode_vec(&data)
			.map_err(|_| ICS02Error::client_not_found(client_id.clone()))?;
		log::trace!(target: "pallet_ibc", "in client : [client_state] >> any client_state: {:?}", state);
		Ok(state)
	}

	fn consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<AnyConsensusState, ICS02Error> {
		log::trace!(target: "pallet_ibc",
			"in client : [consensus_state] >> client_id = {:?}, height = {:?}",
			client_id,
			height
		);

		let native_height = height;
		let value = <ConsensusStates<T>>::get(client_id.clone(), height)
			.ok_or_else(|| ICS02Error::consensus_state_not_found(client_id.clone(), height))?;

		let any_consensus_state = AnyConsensusState::decode_vec(&value)
			.map_err(|_| ICS02Error::consensus_state_not_found(client_id.clone(), native_height))?;
		log::trace!(target: "pallet_ibc",
			"in client : [consensus_state] >> any consensus state = {:?}",
			any_consensus_state
		);
		Ok(any_consensus_state)
	}

	fn host_client_type(&self) -> String {
		if cfg!(any(test, feature = "runtime-benchmarks")) {
			"tendermint".to_string()
		} else {
			"beefy".to_string()
		}
	}

	fn next_consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<Option<AnyConsensusState>, ICS02Error> {
		let mut cs_states = ConsensusStates::<T>::iter_key_prefix(client_id)
			.map(|(height, cs_state)| {
				let cs = AnyConsensusState::decode_vec(&cs_state).map_err(|e| {
					ICS02Error::implementation_specific(format!(
						"[next_consensus_state]: error decoding consensus state from bytes {}",
						e
					))
				})?;
				Ok((height, cs))
			})
			.collect::<Result<Vec<_>, ICS02Error>>()?;

		cs_states.sort_by(|a, b| a.0.cmp(&b.0));

		for cs in cs_states {
			if cs.0 > height {
				return Ok(Some(cs.1))
			}
		}

		Ok(None)
	}

	fn prev_consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<Option<AnyConsensusState>, ICS02Error> {
		let mut cs_states = ConsensusStates::<T>::iter_key_prefix(client_id)
			.map(|(height, cs_state)| {
				let cs = AnyConsensusState::decode_vec(&cs_state).map_err(|e| {
					ICS02Error::implementation_specific(format!(
						"[next_consensus_state]: error decoding consensus state from bytes {}",
						e
					))
				})?;
				Ok((height, cs))
			})
			.collect::<Result<Vec<_>, ICS02Error>>()?;

		cs_states.sort_by(|a, b| b.0.cmp(&a.0));

		for cs in cs_states {
			if cs.0 < height {
				return Ok(Some(cs.1))
			}
		}

		Ok(None)
	}

	fn host_height(&self) -> Height {
		log::trace!(target: "pallet_ibc", "in client: [host_height]");
		let current_height = host_height::<T>();
		let para_id: u32 = parachain_info::Pallet::<T>::get().into();
		Height::new(para_id.into(), current_height)
	}

	#[allow(clippy::disallowed_methods)]
	fn host_timestamp(&self) -> Timestamp {
		use frame_support::traits::UnixTime;
		let time = T::TimeProvider::now();
		let ts = Timestamp::from_nanoseconds(time.as_nanos().saturated_into::<u64>())
			.map_err(|e| panic!("{:?}, caused by {:?} from pallet timestamp_pallet", e, time));
		// Should not panic, if timestamp is invalid after the genesis block then there's a major
		// error in pallet timestamp
		ts.unwrap()
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn host_consensus_state(
		&self,
		_height: Height,
		_proof: Option<Vec<u8>>,
	) -> Result<AnyConsensusState, ICS02Error> {
		let timestamp = Timestamp::from_nanoseconds(1).unwrap();
		let timestamp = timestamp.into_tm_time().unwrap();
		return Ok(AnyConsensusState::Beefy(ics11_beefy::consensus_state::ConsensusState {
			timestamp,
			root: vec![].into(),
		}))
	}

	#[cfg(not(feature = "runtime-benchmarks"))]
	fn host_consensus_state(
		&self,
		height: Height,
		proof: Option<Vec<u8>>,
	) -> Result<AnyConsensusState, ICS02Error> {
		use codec::Compact;
		use sp_core::H256;
		use sp_runtime::traits::{BlakeTwo256, Header};
		use sp_trie::LayoutV0;
		let proof = proof.ok_or_else(|| {
			ICS02Error::implementation_specific(format!("No host proof supplied"))
		})?;

		// unfortunately we can't access headers on-chain, but we can verify them using
		// frame_system's cache of header hashes
		let height = u32::try_from(height.revision_height).map_err(|_| {
			ICS02Error::implementation_specific(format!(
				"[host_consensus_state]: Can't fit height: {} in u32",
				height
			))
		})?;
		let header_hash = frame_system::BlockHash::<T>::get(T::BlockNumber::from(height));
		// we don't even have the hash for this height (anymore?)
		if header_hash == T::Hash::default() {
			Err(ICS02Error::implementation_specific(format!(
				"[host_consensus_state]: Unknown height {}",
				height
			)))?
		}

		let connection_proof: HostConsensusProof =
			codec::Decode::decode(&mut &proof[..]).map_err(|e| {
				ICS02Error::implementation_specific(format!(
					"[host_consensus_state]: Failed to decode proof: {}",
					e
				))
			})?;
		let header = T::Header::decode(&mut &connection_proof.header[..]).map_err(|e| {
			ICS02Error::implementation_specific(format!(
				"[host_consensus_state]: Failed to decode header: {}",
				e
			))
		})?;
		if header.hash() != header_hash {
			Err(ICS02Error::implementation_specific(format!(
				"[host_consensus_state]: Incorrect host consensus state for height {}",
				height
			)))?
		}
		let timestamp = {
			// verify timestamp extrinsic
			let ext = &*connection_proof.extrinsic;
			let proof = &*connection_proof.extrinsic_proof;
			let extrinsic_root = <[u8; 32]>::try_from(header.extrinsics_root().as_ref())
				.expect("header has been verified; qed");
			// Timestamp extrinsic should be the first inherent and hence the first extrinsic
			// https://github.com/paritytech/substrate/blob/d602397a0bbb24b5d627795b797259a44a5e29e9/primitives/trie/src/lib.rs#L99-L101
			let key = codec::Compact(0u32).encode();
			sp_trie::verify_trie_proof::<LayoutV0<BlakeTwo256>, _, _, _>(
				&H256::from(extrinsic_root),
				proof,
				&vec![(key, Some(ext))],
			)
			.map_err(|_| ICS02Error::implementation_specific(format!("Invalid extrinsic proof")))?;

			let (_, _, timestamp): (u8, u8, Compact<u64>) = codec::Decode::decode(&mut &ext[2..])
				.map_err(|err| {
				ICS02Error::implementation_specific(format!("Failed to decode extrinsic: {err}"))
			})?;

			let duration = core::time::Duration::from_millis(timestamp.into());
			Timestamp::from_nanoseconds(duration.as_nanos().saturated_into::<u64>())
				.map_err(|e| {
					ICS02Error::implementation_specific(format!(
						"[host_consensus_state]: error decoding timestamp{}",
						e
					))
				})?
				.into_tm_time()
				.ok_or_else(|| {
					ICS02Error::implementation_specific(
						"[host_consensus_state]: Could not convert timestamp into tendermint time"
							.to_string(),
					)
				})?
		};

		// now this header can be trusted
		let consensus_state = match <T as Config>::LIGHT_CLIENT_PROTOCOL {
			crate::LightClientProtocol::Beefy => {
				let cs_state = ics11_beefy::consensus_state::ConsensusState {
					timestamp,
					root: header.state_root().as_ref().to_vec().into(),
				};
				AnyConsensusState::Beefy(cs_state)
			},
			crate::LightClientProtocol::Grandpa => {
				let cs_state = ics10_grandpa::consensus_state::ConsensusState {
					timestamp,
					root: header.state_root().as_ref().to_vec().into(),
				};
				AnyConsensusState::Grandpa(cs_state)
			},
		};
		Ok(consensus_state)
	}

	fn client_counter(&self) -> Result<u64, ICS02Error> {
		let count = ClientCounter::<T>::get();
		log::trace!(target: "pallet_ibc", "in client : [client_counter] >> client_counter: {:?}", count);

		Ok(count as u64)
	}
}

impl<T: Config> ClientTypes for Context<T> {
	type AnyClientMessage = AnyClientMessage;
	type AnyClientState = AnyClientState;
	type AnyConsensusState = AnyConsensusState;
	type ClientDef = AnyClient;
}

impl<T: Config + Send + Sync> ClientKeeper for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	fn store_client_type(
		&mut self,
		client_id: ClientId,
		client_type: ClientType,
	) -> Result<(), ICS02Error> {
		log::trace!(target: "pallet_ibc",
			"in client : [store_client_type] >> client id = {:?}, client_type = {:?}",
			client_id,
			client_type
		);

		let client_type = client_type.as_bytes().to_vec();
		<Clients<T>>::insert(&client_id, client_type);
		Ok(())
	}

	fn store_client_state(
		&mut self,
		client_id: ClientId,
		client_state: AnyClientState,
	) -> Result<(), ICS02Error> {
		log::trace!(target: "pallet_ibc",
			"in client : [store_client_state] >> client_id: {:?}, client_state = {:?}",
			client_id,
			client_state
		);

		let data = client_state.encode_to_vec();
		// store client states key-value
		<ClientStates<T>>::insert(&client_id, data);

		Ok(())
	}

	fn store_consensus_state(
		&mut self,
		client_id: ClientId,
		height: Height,
		consensus_state: AnyConsensusState,
	) -> Result<(), ICS02Error> {
		log::trace!(target: "pallet_ibc", "in client : [store_consensus_state] >> client_id: {:?}, height = {:?}, consensus_state = {:?}",
			client_id, height, consensus_state);

		let data = consensus_state.encode_to_vec();
		// todo: pruning
		ConsensusStates::<T>::insert(client_id, height, data);
		Ok(())
	}

	fn increase_client_counter(&mut self) {
		log::trace!(target: "pallet_ibc", "in client : [increase_client_counter]");
		// increment counter
		if let Some(val) = <ClientCounter<T>>::get().checked_add(1) {
			<ClientCounter<T>>::put(val);
		}
	}

	fn store_update_time(
		&mut self,
		client_id: ClientId,
		height: Height,
		timestamp: Timestamp,
	) -> Result<(), ICS02Error> {
		log::trace!(target: "pallet_ibc", "in client: [store_update_time] >> Client Height {:?}, Host Timestamp {:?} ", height, timestamp);
		let height = height.encode_vec();
		let timestamp = timestamp.nanoseconds();
		let client_id = client_id.as_bytes().to_vec();
		ClientUpdateTime::<T>::insert(client_id, height, timestamp);
		Ok(())
	}

	fn store_update_height(
		&mut self,
		client_id: ClientId,
		height: Height,
		host_height: Height,
	) -> Result<(), ICS02Error> {
		log::trace!(target: "pallet_ibc", "in client: [store_update_height] >> Client Height {:?}, Host Height {:?} ", height, host_height);
		let height = height.encode_vec();
		let host_height = host_height.encode_vec();
		let client_id = client_id.as_bytes().to_vec();
		ClientUpdateHeight::<T>::insert(client_id, height, host_height);
		Ok(())
	}

	fn validate_self_client(&self, client_state: &AnyClientState) -> Result<(), ICS02Error> {
		let (relay_chain, para_id, latest_para_height) = match client_state {
			AnyClientState::Beefy(client_state) => {
				if client_state.is_frozen() {
					Err(ICS02Error::implementation_specific(format!("client state is frozen")))?
				}

				(client_state.relay_chain, client_state.para_id, client_state.latest_para_height)
			},
			AnyClientState::Grandpa(client_state) => {
				if client_state.is_frozen() {
					Err(ICS02Error::implementation_specific(format!("client state is frozen")))?
				}

				(client_state.relay_chain, client_state.para_id, client_state.latest_para_height)
			},
			client => Err(ICS02Error::unknown_client_type(format!("{}", client.client_type())))?,
		};

		if relay_chain != T::RelayChain::get() {
			Err(ICS02Error::implementation_specific(format!("relay chain mis-match")))?
		}

		let self_para_id: u32 = T::ParaId::get().into();
		if para_id != self_para_id {
			Err(ICS02Error::implementation_specific(format!("para-id mis-match")))?
		}

		let block_number: u32 = <frame_system::Pallet<T>>::block_number().into();

		// this really shouldn't be possible
		if latest_para_height >= block_number {
			Err(ICS02Error::implementation_specific(format!(
				"client has latest_para_height {} greater than or equal to chain height {block_number}",
				latest_para_height
			)))?
		}

		Ok(())
	}
}
