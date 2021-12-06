//! Test that the runtime is config is good and secured, no sending XCM

use crate::{
	env_logger_init,
	kusama_test_net::{KusamaNetwork, *},
};
use cumulus_primitives_core::{ChannelStatus, GetChannelInfo, ParaId};
use xcm_emulator::TestExt;

///  there is no XCM `channel` opened to Relay by design (as it is only relay).
#[test]
fn channel_to_relay() {
	env_logger_init();
	KusamaNetwork::reset();
	Picasso::execute_with(|| {
		let status = <picasso_runtime::ParachainSystem as GetChannelInfo>::get_channel_status(
			ParaId::new(2090),
		);
		assert!(matches!(status, ChannelStatus::Closed));
	});
}

/// we have channel to self
#[test]
fn channel_to_self() {
	env_logger_init();
	KusamaNetwork::reset();
	Picasso::execute_with(|| {
		let status = <picasso_runtime::ParachainSystem as GetChannelInfo>::get_channel_status(
			ParaId::new(PICASSO_PARA_ID),
		);
		assert!(matches!(status, ChannelStatus::Ready(_, _)));
	});
}

/// we have channel to other Parachain on same relay
#[test]
fn channel_to_parachain() {
	env_logger_init();
	KusamaNetwork::reset();
	Picasso::execute_with(|| {
		let status = <picasso_runtime::ParachainSystem as GetChannelInfo>::get_channel_status(
			ParaId::new(DALI_PARA_ID),
		);

		assert!(matches!(status, ChannelStatus::Ready(_, _)));
	});
}
