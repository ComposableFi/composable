use crate::{weights::WeightInfo, Config, Pallet, VestingSchedules, VestingWindow};
use frame_support::{
	dispatch::GetStorageVersion,
	traits::{OnRuntimeUpgrade, StorageVersion},
	weights::Weight,
};

pub struct VestingV0ToV1<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> OnRuntimeUpgrade for VestingV0ToV1<T> {
	fn on_runtime_upgrade() -> Weight {
		let current = Pallet::<T>::on_chain_storage_version();
		let new = StorageVersion::new(1);
		if current < new {
			let mut total = 0;
			for (account, asset) in VestingSchedules::<T>::iter_keys() {
				VestingSchedules::<T>::mutate(account, asset, |schedules| {
					let mut new_schedules = schedules.clone();
					for (_id, schedule) in new_schedules.iter_mut() {
						total += 1;
						if let VestingWindow::MomentBased { start, period } =
							schedule.window.clone()
						{
							// start is start of period, not start of ability to
							// claim
							schedule.window =
								VestingWindow::MomentBased { start: start - period, period };
						}
					}
					*schedules = new_schedules;
				});
			}
			new.put::<Pallet<T>>();
			<() as WeightInfo>::update_vesting_schedules(total)
		} else {
			Weight::zero()
		}
	}
}
