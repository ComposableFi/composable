//! Setup of Picasso running as if it is on Kusama relay
use common::{AccountId, Balance};
use cumulus_primitives_core::ParaId;

use frame_support::traits::GenesisBuild;
use polkadot_primitives::v2::{BlockNumber, MAX_CODE_SIZE, MAX_POV_SIZE};
use polkadot_runtime_parachains::configuration::HostConfiguration;
use primitives::currency::CurrencyId;
use sp_runtime::traits::AccountIdConversion;
use xcm_emulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain, XCM_VERSION};

use crate::prelude::*;
pub const PICA: Balance = 1_000_000_000_000;

// keep in sync with parachains, as macro does not allows for names
pub const THIS_PARA_ID: u32 = 2000;
pub const SIBLING_PARA_ID: u32 = 3000;

decl_test_parachain! {
	pub struct Statemine {
		Runtime = statemine_runtime::Runtime,
		Origin = statemine_runtime::Origin,
		XcmpMessageHandler = statemine_runtime::XcmpQueue,
		DmpMessageHandler = statemine_runtime::DmpQueue,
		new_ext = para_ext(common::topology::statemine::ID),
	}
}

decl_test_parachain! {
	pub struct This {
		Runtime = this_runtime::Runtime,
		Origin = this_runtime::Origin,
		XcmpMessageHandler = this_runtime::XcmpQueue,
		DmpMessageHandler = this_runtime::DmpQueue,
		new_ext = picasso_ext(THIS_PARA_ID),
	}
}

// we use picasso as test, need to test out transfer
// and then decide how to imitate hydra
decl_test_parachain! {
	pub struct Sibling {
		Runtime = sibling_runtime::Runtime,
		Origin = sibling_runtime::Origin,
		XcmpMessageHandler = sibling_runtime::XcmpQueue,
		DmpMessageHandler = sibling_runtime::DmpQueue,
		new_ext = picasso_ext(SIBLING_PARA_ID),
	}
}

decl_test_relay_chain! {
	pub struct KusamaRelay {
		Runtime = relay_runtime::Runtime,
		XcmConfig = relay_runtime::xcm_config::XcmConfig,
		new_ext = kusama_ext(),
	}
}

decl_test_network! {
	pub struct KusamaNetwork {
		relay_chain = KusamaRelay,
		parachains = vec![
			(1000, Statemine),
			(2000, This),
			(3000, Sibling),
		],
	}
}

fn default_parachains_host_configuration() -> HostConfiguration<BlockNumber> {
	HostConfiguration {
		minimum_validation_upgrade_delay: 5,
		validation_upgrade_cooldown: 5u32,
		validation_upgrade_delay: 5,
		code_retention_period: 1200,
		max_code_size: MAX_CODE_SIZE,
		max_pov_size: MAX_POV_SIZE,
		max_head_data_size: 32 * 1024,
		group_rotation_frequency: 20,
		chain_availability_period: 4,
		thread_availability_period: 4,
		max_upward_queue_count: 8,
		max_upward_queue_size: 1024 * 1024,
		max_downward_message_size: 1024,
		ump_service_total_weight: 4 * 1_000_000_000,
		max_upward_message_size: 50 * 1024,
		max_upward_message_num_per_candidate: 5,
		hrmp_sender_deposit: 0,
		hrmp_recipient_deposit: 0,
		hrmp_channel_max_capacity: 8,
		hrmp_channel_max_total_size: 8 * 1024,
		hrmp_max_parachain_inbound_channels: 4,
		hrmp_max_parathread_inbound_channels: 4,
		hrmp_channel_max_message_size: 1024 * 1024,
		hrmp_max_parachain_outbound_channels: 4,
		hrmp_max_parathread_outbound_channels: 4,
		hrmp_max_message_num_per_candidate: 5,
		dispute_period: 6,
		no_show_slots: 2,
		n_delay_tranches: 25,
		needed_approvals: 2,
		relay_vrf_modulo_samples: 2,
		zeroth_delay_tranche_width: 0,
		..Default::default()
	}
}

pub const ALICE_RELAY_BALANCE: u128 = 2002 * PICA;
pub const PICASSO_RELAY_BALANCE: u128 = 10 * PICA;

pub fn kusama_ext() -> sp_io::TestExternalities {
	use relay_runtime::{Runtime, System};
	let mut storage = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
	// TODO: remove implicit assets and mint these directly in test
	balances::GenesisConfig::<Runtime> {
		balances: vec![
			(AccountId::from(ALICE), ALICE_RELAY_BALANCE),
			(ParaId::from(THIS_PARA_ID).into_account_truncating(), PICASSO_RELAY_BALANCE),
		],
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	polkadot_runtime_parachains::configuration::GenesisConfig::<Runtime> {
		config: default_parachains_host_configuration(),
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	<pallet_xcm::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
		&pallet_xcm::GenesisConfig { safe_xcm_version: Some(XCM_VERSION) },
		&mut storage,
	)
	.unwrap();
	let mut externalities = sp_io::TestExternalities::new(storage);
	externalities.execute_with(|| System::set_block_number(1));
	externalities
}

pub const ALICE_PARACHAIN_BALANCE: u128 = 200 * PICA::ONE;
pub const ALICE_PARACHAIN_PICA: u128 = 200 * PICA::ONE;
pub const ALICE_PARACHAIN_KSM: u128 = 13 * RELAY_NATIVE::ONE;

pub fn picasso_ext(parachain_id: u32) -> sp_io::TestExternalities {
	let parachain_id = parachain_id.into();
	use sibling_runtime::{Runtime, System};
	let mut storage = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
	balances::GenesisConfig::<Runtime> {
		balances: vec![(AccountId::from(ALICE), ALICE_PARACHAIN_BALANCE)],
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	<parachain_info::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
		&parachain_info::GenesisConfig { parachain_id },
		&mut storage,
	)
	.unwrap();
	orml_tokens::GenesisConfig::<Runtime> {
		balances: vec![(AccountId::from(ALICE), CurrencyId::PICA, ALICE_PARACHAIN_PICA)],
	}
	.assimilate_storage(&mut storage)
	.unwrap();
	#[cfg(feature = "dali")]
	<liquidations::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
		&liquidations::GenesisConfig {},
		&mut storage,
	)
	.unwrap();
	<pallet_xcm::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
		&pallet_xcm::GenesisConfig { safe_xcm_version: Some(XCM_VERSION) },
		&mut storage,
	)
	.unwrap();

	let mut externalities = sp_io::TestExternalities::new(storage);
	externalities.execute_with(|| System::set_block_number(1));
	externalities
}

pub fn para_ext(parachain_id: u32) -> sp_io::TestExternalities {
	let parachain_id = parachain_id.into();
	use statemine_runtime::{Runtime, System};
	let mut storage = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
	balances::GenesisConfig::<Runtime> {
		balances: vec![
			(AccountId::from(ALICE), ALICE_PARACHAIN_BALANCE),
			(AccountId::from(alice()), ALICE_PARACHAIN_BALANCE),
		],
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	<parachain_info::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
		&parachain_info::GenesisConfig { parachain_id },
		&mut storage,
	)
	.unwrap();

	<pallet_xcm::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
		&pallet_xcm::GenesisConfig { safe_xcm_version: Some(XCM_VERSION) },
		&mut storage,
	)
	.unwrap();

	let mut externalities = sp_io::TestExternalities::new(storage);
	externalities.execute_with(|| System::set_block_number(1));
	externalities
}
