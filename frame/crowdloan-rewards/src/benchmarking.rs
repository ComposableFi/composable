use super::*;

use crate::{mocks::generate_accounts, Pallet as CrowdloanReward};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_runtime::AccountId32;
use sp_std::prelude::*;

type BlockNumber = u32;
type Balance = u128;
type AccountId = AccountId32;
type RelayChainAccountId = [u8; 32];

const MILLISECS_PER_BLOCK: u64 = 6000;
const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
const HOURS: BlockNumber = MINUTES * 60;
const DAYS: BlockNumber = HOURS * 24;
const WEEKS: BlockNumber = DAYS * 7;

const VESTING_PERIOD: BlockNumber = 48 * WEEKS;

benchmarks! {
  where_clause {
	  where
		T: frame_system::Config<BlockNumber = BlockNumber>,
	  T: Config<Balance = Balance, RelayChainAccountId = RelayChainAccountId, AccountId = AccountId>,
  }

  populate {
		let x in 100..1000;
		  let accounts =
				  generate_accounts(x as _)
				  .into_iter()
				  .map(|(_, a)| (a.as_remote_public(), 1_000_000_000_000, VESTING_PERIOD)).collect();
  }: _(RawOrigin::Root, accounts)

	initialize {
		  let x in 100..1000;
		  let accounts =
				generate_accounts (x as _)
				  .into_iter()
				  .map(|(_, a)| (a.as_remote_public(), 1_000_000_000_000, VESTING_PERIOD)).collect();
		  CrowdloanReward::<T>::do_populate(accounts)?;
  }: _(RawOrigin::Root)

  associate {
		  let x in 100..1000;
		  let accounts =
				generate_accounts(x as _);
		  let accounts_reward = accounts.clone()
				  .into_iter()
				  .map(|(_, a)| (a.as_remote_public(), 1_000_000_000_000, VESTING_PERIOD)).collect();
		  CrowdloanReward::<T>::do_populate(accounts_reward)?;
			CrowdloanReward::<T>::do_initialize()?;
		  frame_system::Pallet::<T>::set_block_number(VESTING_PERIOD);
	}: _(RawOrigin::None, accounts[0 as usize].0.clone(), accounts[0 as usize].1.clone().proof(accounts[0 as usize].0.clone()))

  claim {
		  let x in 100..1000;
		  let accounts =
				  generate_accounts(x as _);
		  let accounts_reward = accounts.clone()
				  .into_iter()
				  .map(|(_, a)| (a.as_remote_public(), 1_000_000_000_000, VESTING_PERIOD)).collect();
		  CrowdloanReward::<T>::do_populate(accounts_reward)?;
			CrowdloanReward::<T>::do_initialize()?;
		  for (reward_account, remote_account) in accounts.clone().into_iter() {
			  CrowdloanReward::<T>::do_associate(reward_account.clone(), remote_account.proof(reward_account))?;
		  }
		  frame_system::Pallet::<T>::set_block_number(VESTING_PERIOD);
	}: _(RawOrigin::Signed(accounts[0 as usize].0.clone()))
}
impl_benchmark_test_suite!(
	CrowdloanReward,
	crate::mocks::ExtBuilder::default().build(),
	crate::mocks::Test,
);
