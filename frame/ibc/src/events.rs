use super::*;
use ibc::events::IbcEvent as RawIbcEvent;

#[derive(
	Encode, Decode, Clone, PartialEq, Eq, frame_support::RuntimeDebug, scale_info::TypeInfo,
)]
/// IBC events
// Using Vec<u8> instead of String because Encode and Decode are not implemented for Strings in
// no-std environment.
pub enum IbcEvent {
	/// New block
	NewBlock { revision_height: u64, revision_number: u64 },
	/// Client created
	CreateClient {
		client_id: Vec<u8>,
		client_type: Vec<u8>,
		revision_height: u64,
		revision_number: u64,
	},
	/// Client updated
	UpdateClient {
		client_id: Vec<u8>,
		client_type: Vec<u8>,
		revision_height: u64,
		revision_number: u64,
	},
	/// Client upgraded
	UpgradeClient { client_id: Vec<u8>, revision_height: u64, revision_number: u64 },
	/// Client misbehaviour
	ClientMisbehaviour { client_id: Vec<u8>, revision_height: u64, revision_number: u64 },
	/// Connection open init
	OpenInitConnection {
		revision_height: u64,
		revision_number: u64,
		connection_id: Option<Vec<u8>>,
	},
	/// Connection open confirm
	OpenConfirmConnection {
		revision_height: u64,
		revision_number: u64,
		connection_id: Option<Vec<u8>>,
	},
	/// Connection try open
	OpenTryConnection { revision_height: u64, revision_number: u64, connection_id: Option<Vec<u8>> },
	/// Connection open acknowledge
	OpenAckConnection { revision_height: u64, revision_number: u64, connection_id: Option<Vec<u8>> },
	/// Channel open init
	OpenInitChannel {
		revision_height: u64,
		revision_number: u64,
		port_id: Vec<u8>,
		channel_id: Option<Vec<u8>>,
	},
	/// Channel open confirm
	OpenConfirmChannel {
		revision_height: u64,
		revision_number: u64,
		port_id: Vec<u8>,
		channel_id: Option<Vec<u8>>,
	},
	/// Channel try open
	OpenTryChannel {
		revision_height: u64,
		revision_number: u64,
		port_id: Vec<u8>,
		channel_id: Option<Vec<u8>>,
	},
	/// Open ack channel
	OpenAckChannel {
		revision_height: u64,
		revision_number: u64,
		port_id: Vec<u8>,
		channel_id: Option<Vec<u8>>,
	},
	/// Channel close init
	CloseInitChannel {
		revision_height: u64,
		revision_number: u64,
		port_id: Vec<u8>,
		channel_id: Vec<u8>,
	},
	/// Channel close confirm
	CloseConfirmChannel { revision_height: u64, revision_number: u64, channel_id: Option<Vec<u8>> },
	/// Receive packet
	ReceivePacket {
		revision_height: u64,
		revision_number: u64,
		port_id: Vec<u8>,
		channel_id: Vec<u8>,
		dest_port: Vec<u8>,
		dest_channel: Vec<u8>,
		sequence: u64,
	},
	/// Send packet
	SendPacket {
		revision_height: u64,
		revision_number: u64,
		port_id: Vec<u8>,
		channel_id: Vec<u8>,
		dest_port: Vec<u8>,
		dest_channel: Vec<u8>,
		sequence: u64,
	},
	/// Acknowledgement packet
	AcknowledgePacket {
		revision_height: u64,
		revision_number: u64,
		port_id: Vec<u8>,
		channel_id: Vec<u8>,
		sequence: u64,
	},
	/// WriteAcknowledgement packet
	WriteAcknowledgement {
		revision_height: u64,
		revision_number: u64,
		port_id: Vec<u8>,
		channel_id: Vec<u8>,
		dest_port: Vec<u8>,
		dest_channel: Vec<u8>,
		sequence: u64,
	},
	/// Timeout packet
	TimeoutPacket {
		revision_height: u64,
		revision_number: u64,
		port_id: Vec<u8>,
		channel_id: Vec<u8>,
		sequence: u64,
	},
	/// Timeoutonclose packet
	TimeoutOnClosePacket {
		revision_height: u64,
		revision_number: u64,
		port_id: Vec<u8>,
		channel_id: Vec<u8>,
		sequence: u64,
	},
	/// Empty
	Empty,
	/// Chain Error
	ChainError,
	/// App module
	AppModule,
}

impl From<RawIbcEvent> for IbcEvent {
	fn from(event: RawIbcEvent) -> Self {
		match event {
			RawIbcEvent::NewBlock(ev) => IbcEvent::NewBlock {
				revision_height: ev.height.revision_height,
				revision_number: ev.height.revision_number,
			},
			RawIbcEvent::CreateClient(ev) => IbcEvent::CreateClient {
				client_id: ev.0.client_id.as_bytes().to_vec(),
				client_type: ev.0.client_type.to_string().as_bytes().to_vec(),
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
			},
			RawIbcEvent::UpdateClient(ev) => IbcEvent::UpdateClient {
				client_id: ev.client_id().as_bytes().to_vec(),
				client_type: ev.client_type().to_string().as_bytes().to_vec(),
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
			},
			RawIbcEvent::UpgradeClient(ev) => IbcEvent::UpgradeClient {
				client_id: ev.client_id().as_bytes().to_vec(),
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
			},
			RawIbcEvent::ClientMisbehaviour(ev) => IbcEvent::ClientMisbehaviour {
				client_id: ev.client_id().as_bytes().to_vec(),
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
			},
			RawIbcEvent::OpenInitConnection(ev) => IbcEvent::OpenInitConnection {
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
				connection_id: ev.connection_id().map(|val| val.as_bytes().to_vec()),
			},
			RawIbcEvent::OpenTryConnection(ev) => IbcEvent::OpenTryConnection {
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
				connection_id: ev.connection_id().map(|val| val.as_bytes().to_vec()),
			},
			RawIbcEvent::OpenAckConnection(ev) => IbcEvent::OpenAckConnection {
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
				connection_id: ev.connection_id().map(|val| val.as_bytes().to_vec()),
			},
			RawIbcEvent::OpenConfirmConnection(ev) => IbcEvent::OpenConfirmConnection {
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
				connection_id: ev.connection_id().map(|val| val.as_bytes().to_vec()),
			},
			RawIbcEvent::OpenInitChannel(ev) => IbcEvent::OpenInitChannel {
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
				channel_id: ev
					.channel_id()
					.map(|channel_id| channel_id.to_string().as_bytes().to_vec()),
				port_id: ev.port_id().as_bytes().to_vec(),
			},
			RawIbcEvent::OpenTryChannel(ev) => IbcEvent::OpenTryChannel {
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
				channel_id: ev
					.channel_id()
					.map(|channel_id| channel_id.to_string().as_bytes().to_vec()),
				port_id: ev.port_id().as_bytes().to_vec(),
			},
			RawIbcEvent::OpenAckChannel(ev) => IbcEvent::OpenAckChannel {
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
				channel_id: ev
					.channel_id()
					.map(|channel_id| channel_id.to_string().as_bytes().to_vec()),
				port_id: ev.port_id().as_bytes().to_vec(),
			},
			RawIbcEvent::OpenConfirmChannel(ev) => IbcEvent::OpenConfirmChannel {
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
				channel_id: ev
					.channel_id()
					.map(|channel_id| channel_id.to_string().as_bytes().to_vec()),
				port_id: ev.port_id().as_bytes().to_vec(),
			},
			RawIbcEvent::CloseInitChannel(ev) => IbcEvent::CloseInitChannel {
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
				channel_id: ev.channel_id().to_string().as_bytes().to_vec(),
				port_id: ev.port_id().as_bytes().to_vec(),
			},
			RawIbcEvent::CloseConfirmChannel(ev) => IbcEvent::CloseConfirmChannel {
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
				channel_id: ev
					.channel_id()
					.map(|channel_id| channel_id.to_string().as_bytes().to_vec()),
			},
			RawIbcEvent::SendPacket(ev) => IbcEvent::SendPacket {
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
				channel_id: ev.src_channel_id().to_string().as_bytes().to_vec(),
				port_id: ev.src_port_id().as_bytes().to_vec(),
				dest_port: ev.dst_port_id().as_bytes().to_vec(),
				dest_channel: ev.dst_channel_id().to_string().as_bytes().to_vec(),
				sequence: ev.packet.sequence.into(),
			},
			RawIbcEvent::ReceivePacket(ev) => IbcEvent::ReceivePacket {
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
				channel_id: ev.src_channel_id().to_string().as_bytes().to_vec(),
				port_id: ev.src_port_id().as_bytes().to_vec(),
				dest_port: ev.dst_port_id().as_bytes().to_vec(),
				dest_channel: ev.dst_channel_id().to_string().as_bytes().to_vec(),
				sequence: ev.packet.sequence.into(),
			},
			RawIbcEvent::WriteAcknowledgement(ev) => IbcEvent::WriteAcknowledgement {
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
				channel_id: ev.src_channel_id().to_string().as_bytes().to_vec(),
				port_id: ev.src_port_id().as_bytes().to_vec(),
				dest_port: ev.dst_port_id().as_bytes().to_vec(),
				dest_channel: ev.dst_channel_id().to_string().as_bytes().to_vec(),
				sequence: ev.packet.sequence.into(),
			},
			RawIbcEvent::AcknowledgePacket(ev) => IbcEvent::AcknowledgePacket {
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
				channel_id: ev.src_channel_id().to_string().as_bytes().to_vec(),
				port_id: ev.src_port_id().as_bytes().to_vec(),
				sequence: ev.packet.sequence.into(),
			},
			RawIbcEvent::TimeoutPacket(ev) => IbcEvent::TimeoutPacket {
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
				channel_id: ev.src_channel_id().to_string().as_bytes().to_vec(),
				port_id: ev.src_port_id().as_bytes().to_vec(),
				sequence: ev.packet.sequence.into(),
			},
			RawIbcEvent::TimeoutOnClosePacket(ev) => IbcEvent::TimeoutOnClosePacket {
				revision_height: ev.height().revision_height,
				revision_number: ev.height().revision_number,
				channel_id: ev.src_channel_id().to_string().as_bytes().to_vec(),
				port_id: ev.src_port_id().as_bytes().to_vec(),
				sequence: ev.packet.sequence.into(),
			},
			RawIbcEvent::Empty(_) => IbcEvent::Empty,
			RawIbcEvent::ChainError(_) => IbcEvent::ChainError,
			RawIbcEvent::AppModule(_) => IbcEvent::AppModule,
		}
	}
}

impl<T: Config> From<Vec<RawIbcEvent>> for Event<T> {
	fn from(events: Vec<RawIbcEvent>) -> Self {
		let events: Vec<IbcEvent> = events.into_iter().map(|ev| ev.into()).collect();
		Event::IbcEvents { events }
	}
}
