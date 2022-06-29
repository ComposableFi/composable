use pallet_ibc::events::IbcEvent;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// IBC events to be consumed by relayer
pub enum IbcRelayerEvent {
	/// Client created
	CreateClient {
		client_id: String,
		client_type: String,
		revision_height: u64,
		revision_number: u64,
	},
	/// Client updated
	UpdateClient {
		client_id: String,
		client_type: String,
		revision_height: u64,
		revision_number: u64,
	},
	/// Client upgraded
	UpgradeClient { client_id: String, revision_height: u64, revision_number: u64 },
	/// Client misbehaviour
	ClientMisbehaviour { client_id: String, revision_height: u64, revision_number: u64 },
	/// Connection open init
	OpenInitConnection { revision_height: u64, revision_number: u64, connection_id: String },
	/// Connection open confirm
	OpenConfirmConnection { revision_height: u64, revision_number: u64, connection_id: String },
	/// Connection try open
	OpenTryConnection { revision_height: u64, revision_number: u64, connection_id: String },
	/// Connection open acknowledge
	OpenAckConnection { revision_height: u64, revision_number: u64, connection_id: String },
	/// Channel open init
	OpenInitChannel {
		revision_height: u64,
		revision_number: u64,
		port_id: String,
		channel_id: String,
	},
	/// Channel open confirm
	OpenConfirmChannel {
		revision_height: u64,
		revision_number: u64,
		port_id: String,
		channel_id: String,
	},
	/// Channel try open
	OpenTryChannel {
		revision_height: u64,
		revision_number: u64,
		port_id: String,
		channel_id: String,
	},
	/// Open ack channel
	OpenAckChannel {
		revision_height: u64,
		revision_number: u64,
		port_id: String,
		channel_id: String,
	},
	/// Channel close init
	CloseInitChannel {
		revision_height: u64,
		revision_number: u64,
		port_id: String,
		channel_id: String,
	},
	/// Channel close confirm
	CloseConfirmChannel { revision_height: u64, revision_number: u64, channel_id: String },
	/// Receive packet
	ReceivePacket {
		revision_height: u64,
		revision_number: u64,
		port_id: String,
		channel_id: String,
		dest_port: String,
		dest_channel: String,
		sequence: u64,
	},
	/// Send packet
	SendPacket {
		revision_height: u64,
		revision_number: u64,
		port_id: String,
		channel_id: String,
		dest_port: String,
		dest_channel: String,
		sequence: u64,
	},
	/// Acknowledgement packet
	AcknowledgePacket {
		revision_height: u64,
		revision_number: u64,
		port_id: String,
		channel_id: String,
		sequence: u64,
	},
	/// WriteAcknowledgement packet
	WriteAcknowledgement {
		revision_height: u64,
		revision_number: u64,
		port_id: String,
		channel_id: String,
		dest_port: String,
		dest_channel: String,
		sequence: u64,
	},
	/// Timeout packet
	TimeoutPacket {
		revision_height: u64,
		revision_number: u64,
		port_id: String,
		channel_id: String,
		sequence: u64,
	},
	/// Timeoutonclose packet
	TimeoutOnClosePacket {
		revision_height: u64,
		revision_number: u64,
		port_id: String,
		channel_id: String,
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
