//! Protocol logic specific to ICS4 messages of type `MsgChannelOpenConfirm`.

use crate::{
	core::{
		ics03_connection::connection::State as ConnectionState,
		ics04_channel::{
			channel::{ChannelEnd, Counterparty, State},
			error::Error,
			events::Attributes,
			handler::{verify::verify_channel_proofs, ChannelIdState, ChannelResult},
			msgs::chan_open_confirm::MsgChannelOpenConfirm,
		},
		ics26_routing::context::ReaderContext,
	},
	events::IbcEvent,
	handler::{HandlerOutput, HandlerResult},
	prelude::*,
};

pub(crate) fn process<Ctx>(
	ctx: &Ctx,
	msg: &MsgChannelOpenConfirm,
) -> HandlerResult<ChannelResult, Error>
where
	Ctx: ReaderContext,
{
	let mut output = HandlerOutput::builder();

	// Unwrap the old channel end and validate it against the message.
	let mut channel_end = ctx.channel_end(&(msg.port_id.clone(), msg.channel_id))?;

	// Validate that the channel end is in a state where it can be confirmed.
	if !channel_end.state_matches(&State::TryOpen) {
		return Err(Error::invalid_channel_state(msg.channel_id, channel_end.state))
	}

	// An OPEN IBC connection running on the local (host) chain should exist.
	if channel_end.connection_hops().len() != 1 {
		return Err(Error::invalid_connection_hops_length(1, channel_end.connection_hops().len()))
	}

	let conn = ctx
		.connection_end(&channel_end.connection_hops()[0])
		.map_err(Error::ics03_connection)?;

	if !conn.state_matches(&ConnectionState::Open) {
		return Err(Error::connection_not_open(channel_end.connection_hops()[0].clone()))
	}

	// Proof verification in two steps:
	// 1. Setup: build the Channel as we expect to find it on the other party.

	let expected_counterparty = Counterparty::new(msg.port_id.clone(), Some(msg.channel_id));

	let connection_counterparty = conn.counterparty();
	let ccid = connection_counterparty.connection_id().ok_or_else(|| {
		Error::undefined_connection_counterparty(channel_end.connection_hops()[0].clone())
	})?;

	let expected_connection_hops = vec![ccid.clone()];

	let expected_channel_end = ChannelEnd::new(
		State::Open,
		*channel_end.ordering(),
		expected_counterparty,
		expected_connection_hops,
		channel_end.version().clone(),
	);
	//2. Verify proofs
	verify_channel_proofs::<Ctx>(
		ctx,
		msg.proofs.height(),
		&channel_end,
		&conn,
		&expected_channel_end,
		&msg.proofs.object_proof(),
	)
	.map_err(Error::chan_open_confirm_proof_verification)?;

	output.log("success: channel open confirm ");

	// Transition the channel end to the new state.
	channel_end.set_state(State::Open);

	let event_attributes = Attributes {
		channel_id: Some(msg.channel_id),
		height: ctx.host_height(),
		port_id: msg.port_id.clone(),
		connection_id: channel_end.connection_hops[0].clone(),
		counterparty_port_id: channel_end.counterparty().port_id.clone(),
		counterparty_channel_id: channel_end.counterparty().channel_id.clone(),
	};

	let result = ChannelResult {
		port_id: msg.port_id.clone(),
		channel_id: msg.channel_id,
		channel_id_state: ChannelIdState::Reused,
		channel_end,
	};

	output.emit(IbcEvent::OpenConfirmChannel(
		event_attributes.try_into().map_err(|_| Error::missing_channel_id())?,
	));

	Ok(output.with_result(result))
}

#[cfg(test)]
mod tests {
	use crate::prelude::*;

	use test_log::test;

	use crate::{
		core::{
			ics02_client::context::ClientReader,
			ics03_connection::{
				connection::{
					ConnectionEnd, Counterparty as ConnectionCounterparty, State as ConnectionState,
				},
				msgs::test_util::get_dummy_raw_counterparty,
				version::get_compatible_versions,
			},
			ics04_channel::{
				channel::{ChannelEnd, Counterparty, Order, State},
				handler::channel_dispatch,
				msgs::{
					chan_open_confirm::{
						test_util::get_dummy_raw_msg_chan_open_confirm, MsgChannelOpenConfirm,
					},
					ChannelMsg,
				},
				Version,
			},
			ics24_host::identifier::{ClientId, ConnectionId},
		},
		events::IbcEvent,
		mock::{
			client_state::MockClientState,
			context::{MockClientTypes, MockContext},
		},
		timestamp::ZERO_DURATION,
		Height,
	};

	// TODO: The tests here should use the same structure as `handler::chan_open_try::tests`.
	#[test]
	fn chan_open_confirm_msg_processing() {
		struct Test {
			name: String,
			ctx: MockContext<MockClientTypes>,
			msg: ChannelMsg,
			want_pass: bool,
		}
		let client_id = ClientId::new(&MockClientState::client_type(), 24).unwrap();
		let conn_id = ConnectionId::new(2);
		let context = MockContext::default();
		let client_consensus_state_height = context.host_height().revision_height;

		// The connection underlying the channel we're trying to open.
		let conn_end = ConnectionEnd::new(
			ConnectionState::Open,
			client_id.clone(),
			ConnectionCounterparty::try_from(get_dummy_raw_counterparty()).unwrap(),
			get_compatible_versions(),
			ZERO_DURATION,
		);

		let msg_chan_confirm = MsgChannelOpenConfirm::try_from(
			get_dummy_raw_msg_chan_open_confirm(client_consensus_state_height),
		)
		.unwrap();

		let chan_end = ChannelEnd::new(
			State::TryOpen,
			Order::default(),
			Counterparty::new(msg_chan_confirm.port_id.clone(), Some(msg_chan_confirm.channel_id)),
			vec![conn_id.clone()],
			Version::default(),
		);

		let tests: Vec<Test> = vec![Test {
			name: "Good parameters".to_string(),
			ctx: context
				.with_client(&client_id, Height::new(0, client_consensus_state_height))
				.with_connection(conn_id, conn_end)
				.with_channel(
					msg_chan_confirm.port_id.clone(),
					msg_chan_confirm.channel_id,
					chan_end,
				),
			msg: ChannelMsg::ChannelOpenConfirm(msg_chan_confirm),
			want_pass: true,
		}]
		.into_iter()
		.collect();

		for test in tests {
			let res = channel_dispatch(&test.ctx, &test.msg);
			// Additionally check the events and the output objects in the result.
			match res {
				Ok((proto_output, res)) => {
					assert!(
                            test.want_pass,
                            "chan_open_confirm: test passed but was supposed to fail for test: {}, \nparams {:?} {:?}",
                            test.name,
                            test.msg,
                            test.ctx.clone()
                        );

					let proto_output = proto_output.with_result(());
					assert!(!proto_output.events.is_empty()); // Some events must exist.

					// The object in the output is a ConnectionEnd, should have init state.
					//assert_eq!(res.channel_id, msg_chan_init.channel_id().clone());
					assert_eq!(res.channel_end.state().clone(), State::Open);

					for e in proto_output.events.iter() {
						assert!(matches!(e, &IbcEvent::OpenConfirmChannel(_)));
						assert_eq!(e.height(), test.ctx.host_height());
					}
				},
				Err(e) => {
					assert!(
						!test.want_pass,
						"chan_open_ack: did not pass test: {}, \nparams {:?} {:?}\nerror: {:?}",
						test.name,
						test.msg,
						test.ctx.clone(),
						e,
					);
				},
			}
		}
	}
}
