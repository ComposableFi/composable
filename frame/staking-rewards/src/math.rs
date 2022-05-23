use core::ops::Add;

use composable_traits::time::DurationSeconds;
use sp_runtime::Perbill;
use sp_std::convert::TryInto;
use sp_std::num::TryFromIntError;
type Balance = u128;

// keep in sync with python math
pub fn honest_stake_increase(after_penalized: Perbill, original_amount: Balance, new_amount : Balance, duration: DurationSeconds, passed : DurationSeconds ) -> Option<DurationSeconds> {
   let  penalized_amount = after_penalized.mul_ceil(original_amount); // really we want to consider more original staking to reduce error in time
   let total = penalized_amount.checked_add(new_amount)?;
   let remaining = duration.checked_sub(passed)?;
   let bonus_time = penalized_amount.checked_mul(remaining as u128)?;
   let new_time = new_amount.checked_mul(duration as u128)?;
   let new_remaining_time = (duration as u128).min(bonus_time.checked_add(new_time)?.checked_div(total)?.checked_add(1)?);
   new_remaining_time.try_into().ok()
} 

#[cfg(test)]
mod tests {
    use sp_runtime::Perbill;

    use crate::math::honest_stake_increase;

    #[test]
    fn with_zero_time_passed_staking_gives_same_time() {
       let original_amount = 1_000;
       let new_amount = 1_000;
       let duration = 1_000;
       let passed = 0;
       let after_penalized = Perbill::from_rational(50_u32, 100_u32);
       let remaining = honest_stake_increase(after_penalized, original_amount, new_amount,  duration, passed).unwrap();
       assert_eq!(remaining, 1000, "does not allows to reduce duration doing staking");
    }

   #[test]
   fn with_some_time_passed_rounds_to_bigger() {
      let original_amount = 1_000;
      let new_amount = 100_000;
      let duration = 1_000;
      let passed = 500;
      let after_penalized = Perbill::from_rational(50_u32, 100_u32);
      let remaining = honest_stake_increase(after_penalized, original_amount, new_amount,  duration, passed).unwrap();
      assert_eq!(remaining, 998, "rounded up from 997.5124378109452");
   }
} 