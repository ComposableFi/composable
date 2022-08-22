use futures::StreamExt;
use ibc::{events::IbcEvent, timestamp::Timestamp};

use hyperspace::{
	chain::{
		parachain::calls::{SetPalletParams, Transfer, TransferParams},
		IbcProvider,
	},
	logging,
};
use transfer::PalletParams;

use std::time::{Duration, Instant};

use common::{wait_for_client_and_connection, Args, ChannelToOpen, TestParams};

mod common;

#[tokio::main]
async fn main() {
	logging::setup_logging();
	let args = Args::default();
	let (handle, _, channel_id, client_a, client_b) =
		wait_for_client_and_connection(args.clone(), ChannelToOpen::Transfer).await;
	let timeout_timestamp =
		(Timestamp::now() + Duration::from_secs(86400 * 30)).unwrap().nanoseconds();
	// Send Token from alice to alice on chain b
	let params = TransferParams {
		to: {
			let alice = sp_keyring::AccountKeyring::Alice.public().0;
			let mut hex_string = hex::encode(alice.to_vec());
			hex_string.insert_str(0, "0x");
			hex_string.as_bytes().to_vec()
		},
		source_channel: channel_id.to_string().as_bytes().to_vec(),
		timeout_timestamp_offset: timeout_timestamp,
		timeout_height_offset: 2000,
	};

	client_a
		.submit_sudo_call(SetPalletParams {
			params: PalletParams { send_enabled: true, receive_enabled: true },
		})
		.await
		.unwrap();
	client_b
		.submit_sudo_call(SetPalletParams {
			params: PalletParams { send_enabled: true, receive_enabled: true },
		})
		.await
		.unwrap();

	println!("Sending tokens");

	client_a
		.transfer_tokens(params.clone(), 1, 1_111_111_111_111_111)
		.await
		.unwrap();

	let mut test_params_a = TestParams {
		should_check: true,
		confirmed_acknowledgement: false,
		confirmed_receipt: false,
	};

	let mut test_params_b = TestParams {
		should_check: true,
		confirmed_acknowledgement: false,
		confirmed_receipt: false,
	};

	let (mut chain_a_events, mut chain_b_events) =
		(client_a.ibc_events().await, client_b.ibc_events().await);

	let start = Instant::now();

	loop {
		// If channel states are open on both chains we can stop listening for events
		// if !test_params_a.should_check && !test_params_b.should_check {
		// 	println!("Successfully verified packet receipt on both chains");
		// 	break
		// }

		// let time_elapsed = Instant::now().duration_since(start);
		// if time_elapsed >= Duration::from_secs(1600) {
		// 	println!("Could not verify packet receipt on either chain after waiting for 20mins");
		// 	break
		// }

		tokio::select! {
			result = chain_a_events.next(), if test_params_a.should_check  => {
				match result {
					None => break,
					Some(Ok(events)) => {
						for event in events {
							match event {
								//Check for  acknowledgement packet on sending chain
								IbcEvent::AcknowledgePacket(_ack_packet) if !test_params_a.confirmed_acknowledgement => {
									test_params_a.confirmed_acknowledgement = true;
									break
								}
								//Check for write acknowledgement event on receiving chain
								IbcEvent::WriteAcknowledgement(_write_ack) if !test_params_a.confirmed_receipt => {
									test_params_a.confirmed_receipt = true;
									break
								},
								_ => continue
							}
						}
						if test_params_a.confirmed_acknowledgement && test_params_a.confirmed_receipt {
							test_params_a.should_check = false;
							continue
						}
					}
					Some(Err(err)) => {
						println!("Received Error {:?}", err);
							break
					}
				}
			}
			result = chain_b_events.next(), if test_params_b.should_check => {
				match result {
					None => break,
					Some(Ok(events)) => {
						for event in events {
							match event {
								//Check for  acknowledgement packet on sending chain
								IbcEvent::AcknowledgePacket(_ack_packet) if !test_params_b.confirmed_acknowledgement => {
									test_params_b.confirmed_acknowledgement = true;
									break
								}
								//Check for write acknowledgement event on receiving chain
								IbcEvent::WriteAcknowledgement(_write_ack) if !test_params_b.confirmed_receipt => {
									test_params_b.confirmed_receipt = true;
									// Send tokens back from chain B
									// IBC Asset Id on chain b should be 400_000_000_001
									println!("Sending ibc token back to chain A");
									client_b.transfer_tokens(params.clone(), 400_000_000_001, 1_111_111_111_111_111).await.unwrap();
									break
								},
								_ => continue
							}
						}
						if test_params_b.confirmed_acknowledgement && test_params_b.confirmed_receipt {
							test_params_b.should_check = false;
							continue
						}
					}
					Some(Err(err)) => {
						println!("Received Error {:?}", err);
						break
					}
				}
			}
		}
	}
	handle.abort()
}
