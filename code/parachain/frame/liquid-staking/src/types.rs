use parity_scale_codec::{Decode, Encode, HasCompact};
use sp_runtime::{
    generic,
    traits::{IdentifyAccount, Verify},
    FixedU128, MultiSignature, Permill, RuntimeDebug,
};
// pub use cumulus_primitives_core::{PersistedValidationData};

pub type Ratio = Permill;
pub type EraIndex = u32;
pub type DerivativeIndex = u16;
pub type Rate = FixedU128;

use super::{BalanceOf, Config};
use frame_support::{
    dispatch::DispatchResult,
    traits::{tokens::Balance as BalanceT, DefensiveSaturating},
};
use scale_info::TypeInfo;
use sp_runtime::{traits::Zero, ArithmeticError, DispatchError, FixedPointOperand};
use sp_std::{cmp::Ordering, result::Result, vec, vec::Vec};
// use polkadot_primitives::PersistedValidationData;
pub use polkadot_primitives::PersistedValidationData;


pub trait ValidationDataProvider {
    fn validation_data() -> Option<PersistedValidationData>;
}

pub trait DecimalProvider<CurrencyId> {
    fn get_decimal(asset_id: &CurrencyId) -> Option<u8>;
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct ReservableAmount<Balance> {
    pub total: Balance,
    pub reserved: Balance,
}

impl<Balance: BalanceT + FixedPointOperand> ReservableAmount<Balance> {
    pub fn free(&self) -> Result<Balance, DispatchError> {
        Ok(self
            .total
            .checked_sub(&self.reserved)
            .ok_or(ArithmeticError::Underflow)?)
    }
}

/// The matching pool's total stake & unstake amount in one era
#[derive(Copy, Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct MatchingLedger<Balance> {
    /// The total stake amount in one era
    pub total_stake_amount: ReservableAmount<Balance>,
    /// The total unstake amount in one era
    pub total_unstake_amount: ReservableAmount<Balance>,
}

impl<Balance: BalanceT + FixedPointOperand> MatchingLedger<Balance> {
    /// Matching requests in current period.
    ///
    /// `unbonding_amount` is the total amount of the unbonding asset on the relaychain.
    ///
    /// the returned tri-tuple is formed as `(bond_amount, rebond_amount, unbond_amount)`.
    pub fn matching(
        &self,
        unbonding_amount: Balance,
    ) -> Result<(Balance, Balance, Balance), DispatchError> {
        use Ordering::*;
        let total_free_stake_amount = self.total_stake_amount.free()?;
        let total_free_unstake_amount = self.total_unstake_amount.free()?;

        let (bond_amount, rebond_amount, unbond_amount) = if matches!(
            total_free_stake_amount.cmp(&total_free_unstake_amount),
            Less | Equal
        ) {
            (
                Zero::zero(),
                Zero::zero(),
                total_free_unstake_amount - total_free_stake_amount,
            )
        } else {
            let amount = total_free_stake_amount - total_free_unstake_amount;
            if amount < unbonding_amount {
                (Zero::zero(), amount, Zero::zero())
            } else {
                (amount - unbonding_amount, unbonding_amount, Zero::zero())
            }
        };

        Ok((bond_amount, rebond_amount, unbond_amount))
    }

    pub fn add_stake_amount(&mut self, amount: Balance) -> DispatchResult {
        self.total_stake_amount.total = self
            .total_stake_amount
            .total
            .checked_add(&amount)
            .ok_or(ArithmeticError::Overflow)?;
        Ok(())
    }

    pub fn add_unstake_amount(&mut self, amount: Balance) -> DispatchResult {
        self.total_unstake_amount.total = self
            .total_unstake_amount
            .total
            .checked_add(&amount)
            .ok_or(ArithmeticError::Overflow)?;
        Ok(())
    }

    pub fn consolidate_stake(&mut self, amount: Balance) -> DispatchResult {
        self.remove_stake_amount_lock(amount)?;
        self.sub_stake_amount(amount)?;
        self.clear()?;
        Ok(())
    }

    pub fn consolidate_unstake(&mut self, amount: Balance) -> DispatchResult {
        self.remove_unstake_amount_lock(amount)?;
        self.sub_unstake_amount(amount)?;
        self.clear()?;
        Ok(())
    }

    pub fn sub_stake_amount(&mut self, amount: Balance) -> DispatchResult {
        let total_free_stake_amount = self.total_stake_amount.free()?;
        if total_free_stake_amount < amount {
            return Err(ArithmeticError::Underflow.into());
        }

        self.total_stake_amount.total = self
            .total_stake_amount
            .total
            .checked_sub(&amount)
            .ok_or(ArithmeticError::Underflow)?;
        Ok(())
    }

    pub fn sub_unstake_amount(&mut self, amount: Balance) -> DispatchResult {
        let total_free_unstake_amount = self.total_unstake_amount.free()?;
        if total_free_unstake_amount < amount {
            return Err(ArithmeticError::Underflow.into());
        }

        self.total_unstake_amount.total = self
            .total_unstake_amount
            .total
            .checked_sub(&amount)
            .ok_or(ArithmeticError::Underflow)?;
        Ok(())
    }

    pub fn set_stake_amount_lock(&mut self, amount: Balance) -> DispatchResult {
        let new_reserved_stake_amount = self
            .total_stake_amount
            .reserved
            .checked_add(&amount)
            .ok_or(ArithmeticError::Overflow)?;
        if new_reserved_stake_amount > self.total_stake_amount.total {
            return Err(ArithmeticError::Overflow.into());
        }
        self.total_stake_amount.reserved = new_reserved_stake_amount;
        Ok(())
    }

    fn remove_stake_amount_lock(&mut self, amount: Balance) -> DispatchResult {
        self.total_stake_amount.reserved = self
            .total_stake_amount
            .reserved
            .checked_sub(&amount)
            .ok_or(ArithmeticError::Underflow)?;
        Ok(())
    }

    pub fn set_unstake_amount_lock(&mut self, amount: Balance) -> DispatchResult {
        let new_reserved_unstake_amount = self
            .total_unstake_amount
            .reserved
            .checked_add(&amount)
            .ok_or(ArithmeticError::Overflow)?;
        if new_reserved_unstake_amount > self.total_unstake_amount.total {
            return Err(ArithmeticError::Overflow.into());
        }
        self.total_unstake_amount.reserved = new_reserved_unstake_amount;
        Ok(())
    }

    fn remove_unstake_amount_lock(&mut self, amount: Balance) -> DispatchResult {
        self.total_unstake_amount.reserved = self
            .total_unstake_amount
            .reserved
            .checked_sub(&amount)
            .ok_or(ArithmeticError::Underflow)?;
        Ok(())
    }

    fn clear(&mut self) -> DispatchResult {
        let total_free_stake_amount = self.total_stake_amount.free()?;
        let total_free_unstake_amount = self.total_unstake_amount.free()?;
        if total_free_stake_amount != total_free_unstake_amount {
            return Ok(());
        }

        self.total_stake_amount.total = self
            .total_stake_amount
            .total
            .checked_sub(&total_free_stake_amount)
            .ok_or(ArithmeticError::Underflow)?;
        self.total_unstake_amount.total = self
            .total_unstake_amount
            .total
            .checked_sub(&total_free_stake_amount)
            .ok_or(ArithmeticError::Underflow)?;
        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub enum XcmRequest<T: Config> {
    Bond {
        index: DerivativeIndex,
        amount: BalanceOf<T>,
    },
    BondExtra {
        index: DerivativeIndex,
        amount: BalanceOf<T>,
    },
    Unbond {
        index: DerivativeIndex,
        amount: BalanceOf<T>,
    },
    Rebond {
        index: DerivativeIndex,
        amount: BalanceOf<T>,
    },
    WithdrawUnbonded {
        index: DerivativeIndex,
        num_slashing_spans: u32,
    },
    Nominate {
        index: DerivativeIndex,
        targets: Vec<T::AccountId>,
    },
}

/// Just a Balance/BlockNumber tuple to encode when a chunk of funds will be unlocked.
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct UnlockChunk<Balance: HasCompact> {
    /// Amount of funds to be unlocked.
    #[codec(compact)]
    pub value: Balance,
    /// Era number at which point it'll be unlocked.
    #[codec(compact)]
    pub era: EraIndex,
}

/// The ledger of a (bonded) stash.
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct StakingLedger<AccountId, Balance: HasCompact> {
    /// The stash account whose balance is actually locked and at stake.
    pub stash: AccountId,
    /// The total amount of the stash's balance that we are currently accounting for.
    /// It's just `active` plus all the `unlocking` balances.
    #[codec(compact)]
    pub total: Balance,
    /// The total amount of the stash's balance that will be at stake in any forthcoming
    /// rounds.
    #[codec(compact)]
    pub active: Balance,
    /// Any balance that is becoming free, which may eventually be transferred out
    /// of the stash (assuming it doesn't get slashed first).
    pub unlocking: Vec<UnlockChunk<Balance>>,
    /// List of eras for which the stakers behind a validator have claimed rewards. Only updated
    /// for validators.
    pub claimed_rewards: Vec<EraIndex>,
}

impl<AccountId, Balance: BalanceT + FixedPointOperand> StakingLedger<AccountId, Balance> {
    /// Initializes the default ledger using the given `Stash` account.
    pub fn new(stash: AccountId, value: Balance) -> Self {
        Self {
            stash,
            total: value,
            active: value,
            unlocking: vec![],
            claimed_rewards: vec![],
        }
    }

    /// Remove entries from `unlocking` that are sufficiently old and reduce the
    /// total by the sum of their balances.
    pub fn consolidate_unlocked(&mut self, current_era: EraIndex) {
        let mut total = self.total;
        self.unlocking.retain(|chunk| {
            if chunk.era > current_era {
                true
            } else {
                total = total.saturating_sub(chunk.value);
                false
            }
        });
        self.total = total;
    }

    /// Rebond funds that were scheduled for unlocking.
    pub fn rebond(&mut self, value: Balance) {
        let mut unlocking_balance: Balance = Zero::zero();

        while let Some(last) = self.unlocking.last_mut() {
            if unlocking_balance + last.value <= value {
                unlocking_balance += last.value;
                self.active += last.value;
                self.unlocking.pop();
            } else {
                let diff = value - unlocking_balance;

                unlocking_balance += diff;
                self.active += diff;
                last.value -= diff;
            }

            if unlocking_balance >= value {
                break;
            }
        }
    }

    /// Add some extra amount that have appeared in the stash `free_balance` into the balance up
    /// for staking.
    pub fn bond_extra(&mut self, value: Balance) {
        self.total += value;
        self.active += value;
    }

    /// Schedule a portion of the stash to be unlocked ready for transfer out after the bond
    /// period ends. If this leaves an amount actively bonded less than
    pub fn unbond(&mut self, value: Balance, target_era: EraIndex) {
        if let Some(mut chunk) = self
            .unlocking
            .last_mut()
            .filter(|chunk| chunk.era == target_era)
        {
            // To keep the chunk count down, we only keep one chunk per era. Since
            // `unlocking` is a FIFO queue, if a chunk exists for `era` we know that it will
            // be the last one.
            chunk.value = chunk.value.defensive_saturating_add(value);
        } else {
            self.unlocking.push(UnlockChunk {
                value,
                era: target_era,
            });
        };

        // Skipped the minimum balance check because the platform will
        // bond `MinNominatorBond` to make sure:
        // 1. No chill call is needed
        // 2. No minimum balance check
        self.active -= value;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum UnstakeProvider {
    RelayChain = 0,
    Loans = 1,
    MatchingPool = 2,
}

impl Default for UnstakeProvider {
    fn default() -> Self {
        UnstakeProvider::RelayChain
    }
}

impl UnstakeProvider {
    pub fn is_relay_chain(&self) -> bool {
        self == &UnstakeProvider::RelayChain
    }

    pub fn is_loans(&self) -> bool {
        self == &UnstakeProvider::Loans
    }

    pub fn is_matching_pool(&self) -> bool {
        self == &UnstakeProvider::MatchingPool
    }
}
