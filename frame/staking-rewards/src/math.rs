use sp_runtime::Perbill;

type Balance = u128;
type Timespan = u64;

// keep in sync with python math
pub fn honest_stake_increase(after_penalized: Perbill, original_amount: Balance, new_amount : Balance, duration: Timespan, passed : Timespan ) -> Option<Balance> {
   let  penalized_amount = after_penalized.mul_floor(original_amount);
   let total = penalized_amount.checked_add(original_amount)?;
   let bonus_time = penalized_amount.checked_mul(duration.checked_sub(passed)?)?;
   let new_time = new_amount.checked_mul(duration)?;
   bonus_time.checked_add(new_time)?.checked_div(total)
}  