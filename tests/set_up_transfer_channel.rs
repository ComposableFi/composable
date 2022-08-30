use hyperspace::logging;
use parachain::calls::SetPalletParams;
mod common;
use crate::common::{wait_for_client_and_connection, Args, ChannelToOpen};
use pallet_ibc::PalletParams;

#[tokio::main]
async fn main() {
	logging::setup_logging();
	let args = Args::default();
	let (handle, _, _, client_a, client_b) =
		wait_for_client_and_connection(args.clone(), ChannelToOpen::Transfer).await;

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

	handle.await.unwrap();
}
