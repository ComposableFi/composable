use futures::StreamExt;
use ibc::events::IbcEvent;

use hyperspace::logging;
use parachain::calls::{SendPing, SendPingParams};
use primitives::IbcProvider;

use std::time::{Duration, Instant};

use common::{wait_for_client_and_connection, Args, ChannelToOpen, TestParams};

mod common;

#[tokio::main]
async fn main() {
	logging::setup_logging();
	let args = Args::default();
	let (handle, _, channel_id, client_a, client_b) =
		wait_for_client_and_connection(args, ChannelToOpen::Ping).await;
	let timeout_timestamp = Duration::from_secs(86400 * 30);
	// Send ping packet from chain A to B and vice versa
	let ping_params = SendPingParams {
		data: "ping".as_bytes().to_vec(),
		timeout_height_offset: 500,
		timeout_timestamp_offset: timeout_timestamp.as_nanos() as u64,
		channel_id: channel_id.sequence(),
	};

	println!("Sending ping packet");

	client_a
		.submit_sudo_call(SendPing { params: ping_params.clone() })
		.await
		.unwrap();

	client_b.submit_sudo_call(SendPing { params: ping_params }).await.unwrap();

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

	// Wait for packets to be received and processed on both chains

	let start = Instant::now();

	loop {
		if !test_params_a.should_check && !test_params_b.should_check {
			println!("Successfully verified packet receipt on both chains");
			break;
		}

		let time_elapsed = Instant::now().duration_since(start);
		if time_elapsed >= Duration::from_secs(1200) {
			println!("Could not verify packet receipt on either chain after waiting for 20mins");
			break;
		}

		tokio::select! {
			result = chain_a_events.next(), if test_params_a.should_check  => {
				match result {
					None => break,
					Some(Ok(events)) => {
						for event in events {
							match event {
								// Check for the first write acknowledegement and acknowledge packet events
								IbcEvent::WriteAcknowledgement(_) if !test_params_a.confirmed_receipt => {
									test_params_a.confirmed_receipt = true;
									break
								},
								IbcEvent::AcknowledgePacket(_) if !test_params_a.confirmed_acknowledgement => {
									test_params_a.confirmed_acknowledgement = true;
									break
								}
								_ => continue
							}
						}
						if test_params_a.confirmed_receipt && test_params_a.confirmed_acknowledgement {
							test_params_a.should_check = false;
							continue
						}
					}
					Some(Err(err)) => {
						println!("[parachain_ibc_test] Received Error from stream A {:?}", err);
					}
				}
			}
			result = chain_b_events.next(), if test_params_b.should_check => {
				match result {
					None => break,
					Some(Ok(events)) => {
						for event in events {
							match event {
								IbcEvent::WriteAcknowledgement(_) if !test_params_b.confirmed_receipt => {
									test_params_b.confirmed_receipt = true;
									break
								},
								ibc::events::IbcEvent::AcknowledgePacket(_) if !test_params_b.confirmed_acknowledgement => {
									test_params_b.confirmed_acknowledgement = true;
									break
								}
								_ => continue
							}
						}
						if test_params_b.confirmed_acknowledgement && test_params_b.confirmed_receipt {
							test_params_b.should_check = false;
							continue
						}
					}
					Some(Err(err)) => {
						println!("[parachain_ibc_test] Received Error from stream B {:?}", err);
					}
				}
			}
		}
	}

	handle.abort()
}
