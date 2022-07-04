//! Relayer events.
use pallet_ibc::events::IbcEvent;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// IBC events to be consumed by relayer
pub enum IbcRelayerEvent {
	/// Client created
	CreateClient {
		/// light client id
		client_id: String,
		/// light client type
		client_type: String,
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
	},
	/// Client updated
	UpdateClient {
		/// light client id
		client_id: String,
		/// light client type
		client_type: String,
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
	},
	/// Client upgraded
	UpgradeClient {
		/// light client id
		client_id: String,
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
	},
	/// Client misbehaviour
	ClientMisbehaviour {
		/// light client id
		client_id: String,
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
	},
	/// Connection open init
	OpenInitConnection {
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
		/// Connection id
		connection_id: String,
	},
	/// Connection open confirm
	OpenConfirmConnection {
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
		/// Connection Id
		connection_id: String,
	},
	/// Connection try open
	OpenTryConnection {
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
		/// Connection Id
		connection_id: String,
	},
	/// Connection open acknowledge
	OpenAckConnection {
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
		/// Connection Id
		connection_id: String,
	},
	/// Channel open init
	OpenInitChannel {
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
		/// Port Id
		port_id: String,
		/// Channel Id
		channel_id: String,
	},
	/// Channel open confirm
	OpenConfirmChannel {
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
		/// Port Id
		port_id: String,
		/// Channel Id
		channel_id: String,
	},
	/// Channel try open
	OpenTryChannel {
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
		/// Port Id
		port_id: String,
		/// Channel Id
		channel_id: String,
	},
	/// Open ack channel
	OpenAckChannel {
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
		/// Port Id
		port_id: String,
		/// Channel Id
		channel_id: String,
	},
	/// Channel close init
	CloseInitChannel {
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
		/// Port Id
		port_id: String,
		/// Channel Id
		channel_id: String,
	},
	/// Channel close confirm
	CloseConfirmChannel {
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
		/// Channel Id
		channel_id: String,
	},
	/// Receive packet
	ReceivePacket {
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
		/// Source Port Id
		port_id: String,
		/// Source Channel Id
		channel_id: String,
		/// Destination Port
		dest_port: String,
		/// Destination Channel
		dest_channel: String,
		/// Packet Sequence
		sequence: u64,
	},
	/// Send packet
	SendPacket {
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
		/// Source Port Id
		port_id: String,
		/// Source Channel Id
		channel_id: String,
		/// Destination Port
		dest_port: String,
		/// Destination Channel
		dest_channel: String,
		/// Packet sequence
		sequence: u64,
	},
	/// Acknowledgement packet
	AcknowledgePacket {
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
		/// Source port
		port_id: String,
		/// Source Channel
		channel_id: String,
		/// Packet sequence
		sequence: u64,
	},
	/// WriteAcknowledgement
	WriteAcknowledgement {
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
		/// Source port
		port_id: String,
		/// Source Channel
		channel_id: String,
		/// Destination port
		dest_port: String,
		/// Destination Channel
		dest_channel: String,
		/// Packet Sequence
		sequence: u64,
	},
	/// Timeout packet
	TimeoutPacket {
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
		/// Source port
		port_id: String,
		/// Source Channel
		channel_id: String,
		/// Packet sequence
		sequence: u64,
	},
	/// Timeoutonclose packet
	TimeoutOnClosePacket {
		/// light client revision height
		revision_height: u64,
		/// light client revision number
		revision_number: u64,
		/// Source port
		port_id: String,
		/// Source channel
		channel_id: String,
		/// Packet sequence
		sequence: u64,
	},
}

/// Filter out non relayer events from pallet ibc events
pub fn filter_map_pallet_event(ev: IbcEvent) -> Option<IbcRelayerEvent> {
	match ev {
		IbcEvent::CreateClient { client_id, client_type, revision_height, revision_number } =>
			Some(IbcRelayerEvent::CreateClient {
				client_id: String::from_utf8(client_id).ok()?,
				client_type: String::from_utf8(client_type).ok()?,
				revision_height,
				revision_number,
			}),
		IbcEvent::UpdateClient { client_id, client_type, revision_height, revision_number } =>
			Some(IbcRelayerEvent::UpdateClient {
				client_id: String::from_utf8(client_id).ok()?,
				client_type: String::from_utf8(client_type).ok()?,
				revision_height,
				revision_number,
			}),
		IbcEvent::UpgradeClient { client_id, revision_height, revision_number } =>
			Some(IbcRelayerEvent::UpgradeClient {
				client_id: String::from_utf8(client_id).ok()?,
				revision_number,
				revision_height,
			}),
		IbcEvent::ClientMisbehaviour { client_id, revision_height, revision_number } =>
			Some(IbcRelayerEvent::ClientMisbehaviour {
				client_id: String::from_utf8(client_id).ok()?,
				revision_number,
				revision_height,
			}),
		IbcEvent::OpenInitConnection { revision_height, revision_number, connection_id } =>
			Some(IbcRelayerEvent::OpenInitConnection {
				revision_height,
				revision_number,
				connection_id: connection_id.and_then(|conn_id| String::from_utf8(conn_id).ok())?,
			}),
		IbcEvent::OpenConfirmConnection { revision_height, revision_number, connection_id } =>
			Some(IbcRelayerEvent::OpenConfirmConnection {
				revision_height,
				revision_number,
				connection_id: connection_id.and_then(|conn_id| String::from_utf8(conn_id).ok())?,
			}),
		IbcEvent::OpenTryConnection { revision_height, revision_number, connection_id } =>
			Some(IbcRelayerEvent::OpenTryConnection {
				revision_height,
				revision_number,
				connection_id: connection_id.and_then(|conn_id| String::from_utf8(conn_id).ok())?,
			}),
		IbcEvent::OpenAckConnection { revision_height, revision_number, connection_id } =>
			Some(IbcRelayerEvent::OpenAckConnection {
				revision_height,
				revision_number,
				connection_id: connection_id.and_then(|conn_id| String::from_utf8(conn_id).ok())?,
			}),
		IbcEvent::OpenInitChannel { revision_height, revision_number, port_id, channel_id } =>
			Some(IbcRelayerEvent::OpenInitChannel {
				revision_height,
				revision_number,
				port_id: String::from_utf8(port_id).ok()?,
				channel_id: channel_id.and_then(|channel_id| String::from_utf8(channel_id).ok())?,
			}),
		IbcEvent::OpenConfirmChannel { revision_height, revision_number, port_id, channel_id } =>
			Some(IbcRelayerEvent::OpenConfirmChannel {
				revision_height,
				revision_number,
				port_id: String::from_utf8(port_id).ok()?,
				channel_id: channel_id.and_then(|channel_id| String::from_utf8(channel_id).ok())?,
			}),
		IbcEvent::OpenTryChannel { revision_height, revision_number, port_id, channel_id } =>
			Some(IbcRelayerEvent::OpenTryChannel {
				revision_height,
				revision_number,
				port_id: String::from_utf8(port_id).ok()?,
				channel_id: channel_id.and_then(|channel_id| String::from_utf8(channel_id).ok())?,
			}),
		IbcEvent::OpenAckChannel { revision_height, revision_number, port_id, channel_id } =>
			Some(IbcRelayerEvent::OpenAckChannel {
				revision_height,
				revision_number,
				port_id: String::from_utf8(port_id).ok()?,
				channel_id: channel_id.and_then(|channel_id| String::from_utf8(channel_id).ok())?,
			}),
		IbcEvent::CloseInitChannel { revision_height, revision_number, port_id, channel_id } =>
			Some(IbcRelayerEvent::CloseInitChannel {
				revision_height,
				revision_number,
				port_id: String::from_utf8(port_id).ok()?,
				channel_id: String::from_utf8(channel_id).ok()?,
			}),
		IbcEvent::CloseConfirmChannel { revision_height, revision_number, channel_id } =>
			Some(IbcRelayerEvent::CloseConfirmChannel {
				revision_height,
				revision_number,
				channel_id: channel_id.and_then(|channel_id| String::from_utf8(channel_id).ok())?,
			}),
		IbcEvent::ReceivePacket {
			revision_height,
			revision_number,
			port_id,
			channel_id,
			dest_port,
			dest_channel,
			sequence,
		} => Some(IbcRelayerEvent::ReceivePacket {
			revision_height,
			revision_number,
			port_id: String::from_utf8(port_id).ok()?,
			channel_id: String::from_utf8(channel_id).ok()?,
			dest_port: String::from_utf8(dest_port).ok()?,
			dest_channel: String::from_utf8(dest_channel).ok()?,
			sequence,
		}),
		IbcEvent::SendPacket {
			revision_height,
			revision_number,
			port_id,
			channel_id,
			dest_port,
			dest_channel,
			sequence,
		} => Some(IbcRelayerEvent::SendPacket {
			revision_height,
			revision_number,
			port_id: String::from_utf8(port_id).ok()?,
			channel_id: String::from_utf8(channel_id).ok()?,
			dest_port: String::from_utf8(dest_port).ok()?,
			dest_channel: String::from_utf8(dest_channel).ok()?,
			sequence,
		}),
		IbcEvent::AcknowledgePacket {
			revision_height,
			revision_number,
			port_id,
			channel_id,
			sequence,
		} => Some(IbcRelayerEvent::AcknowledgePacket {
			revision_height,
			revision_number,
			port_id: String::from_utf8(port_id).ok()?,
			channel_id: String::from_utf8(channel_id).ok()?,
			sequence,
		}),
		IbcEvent::WriteAcknowledgement {
			revision_height,
			revision_number,
			port_id,
			channel_id,
			dest_port,
			dest_channel,
			sequence,
		} => Some(IbcRelayerEvent::WriteAcknowledgement {
			revision_height,
			revision_number,
			port_id: String::from_utf8(port_id).ok()?,
			channel_id: String::from_utf8(channel_id).ok()?,
			dest_port: String::from_utf8(dest_port).ok()?,
			dest_channel: String::from_utf8(dest_channel).ok()?,
			sequence,
		}),
		IbcEvent::TimeoutPacket {
			revision_height,
			revision_number,
			port_id,
			channel_id,
			sequence,
		} => Some(IbcRelayerEvent::TimeoutPacket {
			revision_height,
			revision_number,
			port_id: String::from_utf8(port_id).ok()?,
			channel_id: String::from_utf8(channel_id).ok()?,
			sequence,
		}),
		IbcEvent::TimeoutOnClosePacket {
			revision_height,
			revision_number,
			port_id,
			channel_id,
			sequence,
		} => Some(IbcRelayerEvent::TimeoutPacket {
			revision_height,
			revision_number,
			port_id: String::from_utf8(port_id).ok()?,
			channel_id: String::from_utf8(channel_id).ok()?,
			sequence,
		}),
		IbcEvent::NewBlock { .. } |
		IbcEvent::ChainError |
		IbcEvent::Empty |
		IbcEvent::AppModule { .. } => None,
	}
}
