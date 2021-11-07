use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};
use common::AccountId;
pub const ALICE: [u8; 32] = [4u8; 32];
type Balances = u128;
pub const PICA: Balances = 1_000_000_000_000;
use cumulus_primitives_core::ParaId;
use support::traits::GenesisBuild;
use sp_runtime::traits::AccountIdConversion;
pub fn kusama_ext() -> sp_io::TestExternalities {
    use kusama_runtime::{Runtime, System};
    let mut storage = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
    balances::GenesisConfig::<Runtime> {
        balances : vec![
            (AccountId::from(ALICE), 2002 * PICA),
             (ParaId::from(2000).into_account(), 10 * PICA),
        ]
    }.assimilate_storage(&mut storage).unwrap();

    todo!();
}