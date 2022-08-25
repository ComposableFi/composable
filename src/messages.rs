use ibc::{
	core::{
		ics02_client::{header::AnyHeader, msgs::update_client::MsgUpdateAnyClient},
		ics03_connection::{
			connection::{ConnectionEnd, Counterparty},
			msgs::{
				conn_open_ack::MsgConnectionOpenAck, conn_open_confirm::MsgConnectionOpenConfirm,
				conn_open_try::MsgConnectionOpenTry,
			},
		},
		ics04_channel::{
			channel::{ChannelEnd, Counterparty as ChannelCounterparty},
			msgs::{
				acknowledgement::MsgAcknowledgement, chan_close_confirm::MsgChannelCloseConfirm,
				chan_open_ack::MsgChannelOpenAck, chan_open_confirm::MsgChannelOpenConfirm,
				chan_open_try::MsgChannelOpenTry, recv_packet::MsgRecvPacket,
			},
		},
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes},
	},
	events::IbcEvent,
	proofs::Proofs,
	tx_msg::Msg,
	Height,
};
use ibc_proto::google::protobuf::Any;
use primitives::{error::Error, Chain};
use tendermint_proto::Protobuf;

/// Provides an interface for decoding messages from [`IbcEvent`]
pub struct Messages;

impl Messages {
	/// This converts events coming from a source chain to messages that should be delivered to a
	/// counterparty chain.
	pub async fn from(
		chain_a: &mut impl Chain,
		chain_b: &mut impl Chain,
		events: Vec<IbcEvent>,
		header: AnyHeader,
	) -> Result<Vec<Any>, anyhow::Error> {
		let mut messages = vec![];

		// 1. translate events to messages
		for event in events {
			match event {
				IbcEvent::OpenInitConnection(open_init) => {
					if let Some(connection_id) = open_init.connection_id() {
						let connection_id = connection_id.clone();
						// Get connection end with proof
						let connection_response = chain_a
							.query_connection_end(open_init.height(), connection_id.clone())
							.await?;
						let connection_end = ConnectionEnd::try_from(
							connection_response.connection.ok_or_else(|| {
								Error::Custom(format!(
								"[get_messages_for_events - open_conn_init] Connection end not found for {:?}",
								open_init.attributes().connection_id
							))
							})?,
						)?;
						let counterparty = connection_end.counterparty();

						let connection_proof =
							CommitmentProofBytes::try_from(connection_response.proof)?;
						let prefix: CommitmentPrefix = chain_a.connection_prefix();
						// Querying client state because in ibc-rs, the client state proof is
						// required when decoiding the message on the counterparty even if, client
						// state will not be validated
						let client_state_response = chain_a
							.query_client_state(
								open_init.height(),
								open_init.attributes().client_id.clone(),
							)
							.await?;

						let proof_height = connection_response.proof_height.ok_or_else(|| {
						Error::Custom(format!("[get_messages_for_events - open_conn_init] Proof height not found in response"))
					})?;
						let proof_height =
							Height::new(proof_height.revision_number, proof_height.revision_height);
						let client_state_proof =
							CommitmentProofBytes::try_from(client_state_response.proof).ok();

						// Construct OpenTry
						let msg = MsgConnectionOpenTry {
							previous_connection_id: counterparty.connection_id.clone(),
							client_id: counterparty.client_id().clone(),
							// we don't want to prove client state
							client_state: None,
							counterparty: Counterparty::new(
								open_init.attributes().client_id.clone(),
								Some(connection_id),
								prefix,
							),
							counterparty_versions: connection_end.versions().to_vec(),
							proofs: Proofs::new(
								connection_proof,
								client_state_proof,
								None,
								None,
								proof_height,
							)?,
							delay_period: connection_end.delay_period(),

							signer: chain_b.account_id(),
						};

						let value = msg.encode_vec();
						let msg = Any { value, type_url: msg.type_url() };
						messages.push(msg)
					}
				},
				IbcEvent::OpenTryConnection(open_try) => {
					if let Some(connection_id) = open_try.connection_id() {
						let connection_id = connection_id.clone();
						// Get connection end with proof
						let connection_response = chain_a
							.query_connection_end(open_try.height(), connection_id.clone())
							.await?;
						let connection_end = ConnectionEnd::try_from(
							connection_response.connection.ok_or_else(|| {
								Error::Custom(format!(
								"[get_messages_for_events - open_conn_try] Connection end not found for {:?}",
								open_try.attributes().connection_id
							))
							})?,
						)?;
						let counterparty = connection_end.counterparty();

						let connection_proof =
							CommitmentProofBytes::try_from(connection_response.proof)?;
						// Querying client state because in ibc-rs, the client state proof is
						// required when decoiding the message on the counterparty even if, client
						// state will not be validated
						let client_state_response = chain_a
							.query_client_state(
								open_try.height(),
								open_try.attributes().client_id.clone(),
							)
							.await?;

						let proof_height = connection_response.proof_height.ok_or_else(|| {
						Error::Custom(format!("[get_messages_for_events - open_conn_try] Proof height not found in response"))
					})?;
						let proof_height =
							Height::new(proof_height.revision_number, proof_height.revision_height);
						let client_state_proof =
							CommitmentProofBytes::try_from(client_state_response.proof).ok();
						// Construct OpenAck
						let msg = MsgConnectionOpenAck {
							connection_id: counterparty
								.connection_id()
								.ok_or_else(|| {
									Error::Custom(format!("[get_messages_for_events - open_conn_try] Connection Id not found"))
								})?
								.clone(),
							counterparty_connection_id: connection_id,
							client_state: None,
							proofs: Proofs::new(
								connection_proof,
								client_state_proof,
								None,
								None,
								proof_height,
							)?,
							version: connection_end
								.versions()
								.get(0)
								.ok_or_else(|| {
									Error::Custom(format!(
									"[get_messages_for_events - open_conn_try] Connection version is missing for  {:?}",
									open_try.attributes().connection_id
								))
								})?
								.clone(),

							signer: chain_b.account_id(),
						};

						let value = msg.encode_vec();
						let msg = Any { value, type_url: msg.type_url() };
						messages.push(msg)
					}
				},
				IbcEvent::OpenAckConnection(open_ack) => {
					if let Some(connection_id) = open_ack.connection_id() {
						let connection_id = connection_id.clone();
						// Get connection end with proof
						let connection_response = chain_a
							.query_connection_end(open_ack.height(), connection_id.clone())
							.await?;
						let connection_end = ConnectionEnd::try_from(
							connection_response.connection.ok_or_else(|| {
								Error::Custom(format!(
								"[get_messages_for_events - open_conn_ack] Connection end not found for {:?}",
								open_ack.attributes().connection_id
							))
							})?,
						)?;
						let counterparty = connection_end.counterparty();

						let connection_proof =
							CommitmentProofBytes::try_from(connection_response.proof)?;
						// Querying client state because in ibc-rs, the client state proof is
						// required when decoiding the message on the counterparty even if, client
						// state will not be validated
						let client_state_response = chain_a
							.query_client_state(
								open_ack.height(),
								open_ack.attributes().client_id.clone(),
							)
							.await?;

						let proof_height = connection_response.proof_height.ok_or_else(|| {
						Error::Custom(format!("[get_messages_for_events - open_conn_ack] Proof height not found in response"))
					})?;
						let proof_height =
							Height::new(proof_height.revision_number, proof_height.revision_height);
						let client_state_proof =
							CommitmentProofBytes::try_from(client_state_response.proof).ok();

						// Construct OpenAck
						let msg = MsgConnectionOpenConfirm {
							connection_id: counterparty
								.connection_id()
								.ok_or_else(|| {
									Error::Custom(format!("[get_messages_for_events - open_conn_ack] Connection Id not found"))
								})?
								.clone(),
							proofs: Proofs::new(
								connection_proof,
								client_state_proof,
								None,
								None,
								proof_height,
							)?,

							signer: chain_b.account_id(),
						};

						let value = msg.encode_vec();
						let msg = Any { value, type_url: msg.type_url() };
						messages.push(msg)
					}
				},
				IbcEvent::OpenInitChannel(open_init) => {
					if let Some(channel_id) = open_init.channel_id {
						let channel_response = chain_a
							.query_channel_end(
								open_init.height(),
								channel_id,
								open_init.port_id.clone(),
							)
							.await?;
						let channel_end =
							ChannelEnd::try_from(channel_response.channel.ok_or_else(|| {
								Error::Custom(format!(
								"[get_messages_for_events - open_chan_init] ChannelEnd not found for {:?}/{:?}",
								channel_id,
								open_init.port_id.clone()
							))
							})?)
							.expect("Channel end decoding should not fail");
						let counterparty = channel_end.counterparty();
						// Construct the channel end as we expect it to be constructed on the
						// receiving chain
						let channel = ChannelEnd::new(
							channel_end.state,
							channel_end.ordering,
							ChannelCounterparty::new(open_init.port_id, Some(channel_id)),
							channel_end.connection_hops.clone(),
							channel_end.version.clone(),
						);

						let channel_proof = CommitmentProofBytes::try_from(channel_response.proof)?;

						let proof_height = channel_response.proof_height.expect(
						"[get_messages_for_events - open_chan_init]Proof height should be present",
					);
						let proof_height =
							Height::new(proof_height.revision_number, proof_height.revision_height);

						let msg = MsgChannelOpenTry {
							port_id: counterparty.port_id.clone(),
							previous_channel_id: counterparty.channel_id.clone(),
							channel,
							counterparty_version: channel_end.version,
							proofs: Proofs::new(channel_proof, None, None, None, proof_height)?,

							signer: chain_b.account_id(),
						};

						let value = msg.encode_vec();
						let msg = Any { value, type_url: msg.type_url() };
						messages.push(msg)
					}
				},
				IbcEvent::OpenTryChannel(open_try) => {
					if let Some(channel_id) = open_try.channel_id {
						let channel_response = chain_a
							.query_channel_end(
								open_try.height(),
								channel_id,
								open_try.port_id.clone(),
							)
							.await?;
						let channel_end =
							ChannelEnd::try_from(channel_response.channel.ok_or_else(|| {
								Error::Custom(format!(
								"[get_messages_for_events - open_chan_try] ChannelEnd not found for {:?}/{:?}",
								channel_id, open_try.port_id
							))
							})?)
							.expect("Channel end decoding should not fail");
						let counterparty = channel_end.counterparty();
						let channel_proof = CommitmentProofBytes::try_from(channel_response.proof)?;

						let proof_height = channel_response.proof_height.expect(
						"[get_messages_for_events - open_chan_try] Proof height should be present",
					);
						let proof_height =
							Height::new(proof_height.revision_number, proof_height.revision_height);

						let msg = MsgChannelOpenAck {
							port_id: counterparty.port_id.clone(),
							counterparty_version: channel_end.version.clone(),
							proofs: Proofs::new(channel_proof, None, None, None, proof_height)?,
							channel_id: counterparty
								.channel_id
								.expect("Expect channel id to be set"),
							counterparty_channel_id: channel_id,

							signer: chain_b.account_id(),
						};

						let value = msg.encode_vec();
						let msg = Any { value, type_url: msg.type_url() };
						messages.push(msg)
					}
				},
				IbcEvent::OpenAckChannel(open_ack) => {
					if let Some(channel_id) = open_ack.channel_id {
						let channel_response = chain_a
							.query_channel_end(
								open_ack.height(),
								channel_id,
								open_ack.port_id.clone(),
							)
							.await?;
						let channel_end =
							ChannelEnd::try_from(channel_response.channel.ok_or_else(|| {
								Error::Custom(format!(
								"[get_messages_for_events - open_chan_ack] ChannelEnd not found for {:?}/{:?}",
								channel_id, open_ack.port_id
							))
							})?)?;
						let counterparty = channel_end.counterparty();
						let channel_proof = CommitmentProofBytes::try_from(channel_response.proof)?;

						let proof_height =
							channel_response.proof_height.expect("Proof height should be present");
						let proof_height =
							Height::new(proof_height.revision_number, proof_height.revision_height);

						let msg = MsgChannelOpenConfirm {
							port_id: counterparty.port_id.clone(),
							proofs: Proofs::new(channel_proof, None, None, None, proof_height)?,
							channel_id: counterparty
								.channel_id
								.expect("Expect channel id to be set"),

							signer: chain_b.account_id(),
						};

						let value = msg.encode_vec();
						let msg = Any { value, type_url: msg.type_url() };
						messages.push(msg)
					}
				},
				IbcEvent::CloseInitChannel(close_init) => {
					let channel_id = close_init.channel_id;
					let channel_response = chain_a
						.query_channel_end(
							close_init.height(),
							channel_id,
							close_init.port_id.clone(),
						)
						.await?;
					let channel_end =
						ChannelEnd::try_from(channel_response.channel.ok_or_else(|| {
							Error::Custom(format!(
							"[get_messages_for_events - close_chan_init] ChannelEnd not found for {:?}/{:?}",
							channel_id, close_init.port_id
						))
						})?)?;
					let counterparty = channel_end.counterparty();
					let channel_proof = CommitmentProofBytes::try_from(channel_response.proof)?;

					let proof_height =
						channel_response.proof_height.expect("Proof height should be present");
					let proof_height =
						Height::new(proof_height.revision_number, proof_height.revision_height);

					let msg = MsgChannelCloseConfirm {
						port_id: counterparty.port_id.clone(),
						proofs: Proofs::new(channel_proof, None, None, None, proof_height)?,
						channel_id: counterparty.channel_id.expect("Expect channel id to be set"),

						signer: chain_b.account_id(),
					};

					let value = msg.encode_vec();
					let msg = Any { value, type_url: msg.type_url() };
					messages.push(msg)
				},
				IbcEvent::SendPacket(send_packet) => {
					let port_id = send_packet.packet.source_port.clone();
					let channel_id = send_packet.packet.source_channel.clone();
					let seq = u64::from(send_packet.packet.sequence);
					let packet = send_packet.packet;
					let packet_commitment_response = chain_a
						.query_packet_commitment(send_packet.height, &port_id, &channel_id, seq)
						.await?;
					let commitment_proof =
						CommitmentProofBytes::try_from(packet_commitment_response.proof)?;

					let proof_height = packet_commitment_response
						.proof_height
						.expect("Proof height should be present");
					let proof_height =
						Height::new(proof_height.revision_number, proof_height.revision_height);
					let msg = MsgRecvPacket {
						packet: packet.clone(),
						proofs: Proofs::new(commitment_proof, None, None, None, proof_height)?,

						signer: chain_b.account_id(),
					};

					let value = msg.encode_vec();
					let msg = Any { value, type_url: msg.type_url() };
					messages.push(msg);
				},
				IbcEvent::WriteAcknowledgement(write_ack) => {
					let port_id = &write_ack.packet.source_port.clone();
					let channel_id = &write_ack.packet.source_channel.clone();
					let seq = u64::from(write_ack.packet.sequence);
					let packet = write_ack.packet;
					let packet_acknowledgement_response = chain_a
						.query_packet_acknowledgement(write_ack.height, &port_id, &channel_id, seq)
						.await?;
					let acknowledgement = write_ack.ack;
					let commitment_proof =
						CommitmentProofBytes::try_from(packet_acknowledgement_response.proof)?;

					let proof_height = packet_acknowledgement_response
						.proof_height
						.expect("Proof height should be present");
					let proof_height =
						Height::new(proof_height.revision_number, proof_height.revision_height);
					let msg = MsgAcknowledgement {
						packet,
						acknowledgement: acknowledgement.into(),
						proofs: Proofs::new(commitment_proof, None, None, None, proof_height)?,

						signer: chain_b.account_id(),
					};

					let value = msg.encode_vec();
					let msg = Any { value, type_url: msg.type_url() };
					messages.push(msg)
				},
				_ => continue,
			}
		}

		// // 2. fetch timed-out packets
		// {
		// 	let latest_height = chain_b.latest_height().await?;
		// 	let host_latest_height = chain_a.latest_height().await?;
		// 	let consensus_state = chain_b.host_consensus_state(latest_height).await?;
		// 	let mut seqs_to_drop = vec![];
		// 	for packet in chain_a.cached_packets() {
		// 		if packet.timed_out(&consensus_state.timestamp(), latest_height) {
		// 			let chain_b_channel_response = chain_b
		// 				.query_channel_end(
		// 					host_latest_height,
		// 					packet.destination_channel,
		// 					packet.destination_port.clone(),
		// 				)
		// 				.await?;
		// 			let channel_response = chain_a
		// 				.query_channel_end(
		// 					host_latest_height,
		// 					packet.source_channel,
		// 					packet.source_port.clone(),
		// 				)
		// 				.await?;
		// 			let channel_end =
		// 				ChannelEnd::try_from(channel_response.channel.ok_or_else(|| {
		// 					Error::Custom(format!(
		// 						"[get_timeout_messages] ChannelEnd not found for {:?}/{:?}",
		// 						packet.source_channel,
		// 						packet.source_port.clone()
		// 					))
		// 				})?)?;

		// 			let chain_b_channel_end = ChannelEnd::try_from(
		// 				chain_b_channel_response.channel.ok_or_else(|| {
		// 					Error::Custom(format!(
		// 						"[get_timeout_messages] ChannelEnd not found for {:?}/{:?}",
		// 						packet.destination_channel,
		// 						packet.destination_port.clone()
		// 					))
		// 				})?,
		// 			)?;

		// 			let mut keys = vec![];
		// 			if chain_b_channel_end.state == State::Closed {
		// 				let path = format!(
		// 					"{}",
		// 					ChannelEndsPath(
		// 						packet.destination_port.clone(),
		// 						packet.destination_channel.clone()
		// 					)
		// 				);
		// 				keys.push(chain_b.apply_prefix(path))
		// 			}
		// 			if channel_end.ordering == Order::Ordered {
		// 				let path = format!(
		// 					"{}",
		// 					SeqRecvsPath(
		// 						packet.destination_port.clone(),
		// 						packet.destination_channel.clone()
		// 					)
		// 				);
		// 				keys.push(chain_b.apply_prefix(path))
		// 			} else {
		// 				let path = format!(
		// 					"{}",
		// 					ReceiptsPath {
		// 						port_id: packet.destination_port.clone(),
		// 						channel_id: packet.destination_channel.clone(),
		// 						sequence: packet.sequence
		// 					}
		// 				);
		// 				keys.push(chain_b.apply_prefix(path))
		// 			};

		// 			let proof = chain_b.query_proof(latest_height, keys).await?;
		// 			let next_sequence_recv = chain_b
		// 				.query_next_sequence_recv(
		// 					latest_height,
		// 					&packet.destination_port.clone(),
		// 					&packet.destination_channel.clone(),
		// 				)
		// 				.await?;
		// 			let commitment_proof = CommitmentProofBytes::try_from(proof)?;
		// 			if chain_b_channel_end.state == State::Closed {
		// 				let msg = MsgTimeoutOnClose {
		// 					packet: packet.clone(),
		// 					next_sequence_recv: next_sequence_recv.next_sequence_receive.into(),
		// 					proofs: Proofs::new(commitment_proof, None, None, None, latest_height)?,

		// 					signer: chain_a.account_id(),
		// 				};
		// 				let value = msg.encode_vec();
		// 				let msg = Any { value, type_url: msg.type_url() };
		// 				messages.push(msg)
		// 			} else {
		// 				let msg = MsgTimeout {
		// 					packet: packet.clone(),
		// 					next_sequence_recv: next_sequence_recv.next_sequence_receive.into(),
		// 					proofs: Proofs::new(commitment_proof, None, None, None, latest_height)?,

		// 					signer: chain_a.account_id(),
		// 				};
		// 				let value = msg.encode_vec();
		// 				let msg = Any { value, type_url: msg.type_url() };
		// 				messages.push(msg)
		// 			}

		// 			seqs_to_drop.push(packet.sequence)
		// 		}
		// 	}
		// 	chain_a.remove_packets(seqs_to_drop);
		// }

		// 3. insert update client message at first index
		{
			let msg = MsgUpdateAnyClient {
				client_id: chain_a.client_id(),
				header,
				signer: chain_b.account_id(),
			};
			let value = msg.encode_vec();
			let update_client = Any { value, type_url: msg.type_url() };

			messages.insert(0, update_client);
		}

		Ok(messages)
	}
}
