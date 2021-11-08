use polkadot_runtime_parachains::configuration::HostConfiguration;
use primitives::currency::CurrencyId;
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};
use common::AccountId;
use cumulus_primitives_core::ParaId;
use support::traits::GenesisBuild;
use sp_runtime::traits::AccountIdConversion;
use polkadot_primitives::v1::{BlockNumber, MAX_CODE_SIZE, MAX_POV_SIZE};
type Balances = u128;
pub const ALICE: [u8; 32] = [4u8; 32];
pub const PICA: Balances = 1_000_000_000_000;
pub const PICASSO_PARA_ID:u32 = 2000;

fn default_parachains_host_configuration() -> HostConfiguration<BlockNumber> {
	HostConfiguration {
		validation_upgrade_frequency: 1u32,
		validation_upgrade_delay: 1,
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
		max_upward_message_size: 1024 * 1024,
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

pub fn kusama_ext() -> sp_io::TestExternalities {
    use kusama_runtime::{Runtime, System};
    let mut storage = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
    balances::GenesisConfig::<Runtime> {
        balances : vec![
            (AccountId::from(ALICE), 2002 * PICA),
             (ParaId::from(PICASSO_PARA_ID).into_account(), 10 * PICA),
        ]
    }.assimilate_storage(&mut storage).unwrap();

    polkadot_runtime_parachains::configuration::GenesisConfig::<Runtime> {
        config : default_parachains_host_configuration(),
    }.assimilate_storage(&mut storage).unwrap();

    let mut externalities = sp_io::TestExternalities::new(storage);
    externalities.execute_with(|| System::set_block_number(1));;
    externalities
}

pub fn picasso_ext(para_id: ParaId) -> sp_io::TestExternalities {
    use picasso_runtime::{Runtime, System, };
    use primitives::currency::CurrencyId;
    let mut storage = frame_system::GenesisConfig::default()
		.build_storage::<Runtime>()
		.unwrap();
        balances::GenesisConfig::<Runtime> {
            balances: vec![(AccountId::from(ALICE), 200 * 1_000_000_000_000)],
        }
        .assimilate_storage(&mut storage)
        .unwrap();

    <parachain_info::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
        &parachain_info::GenesisConfig {
            parachain_id: para_id,
        },
        &mut storage).unwrap();
    orml_tokens::GenesisConfig::<Runtime> {
        balances: vec![(AccountId::from(ALICE), CurrencyId::PICA, 200 * 1_000_000_000_000)],
    }
        .assimilate_storage(&mut storage).unwrap();

    let mut externalities = sp_io::TestExternalities::new(storage);
    externalities.execute_with(|| System::set_block_number(1));
    externalities
}