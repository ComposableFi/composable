//!

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
	bad_style,
	bare_trait_objects,
	const_err,
	improper_ctypes,
	non_shorthand_field_patterns,
	no_mangle_generic_items,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_parens,
	while_true,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates
)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use codec::{Codec, FullCodec};
	use frame_support::{pallet_prelude::*, PalletId};
	use scale_info::TypeInfo;
	use sp_core::crypto::KeyTypeId;
	use sp_runtime::{
		traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One, Zero},
		FixedPointNumber, FixedPointOperand, FixedU128, KeyTypeId as CryptoKeyTypeId,
	};
	use sp_std::fmt::Debug;

	pub const PALLET_ID: PalletId = PalletId(*b"CurveAmm");
	pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"camm");
	pub const CRYPTO_KEY_TYPE: CryptoKeyTypeId = CryptoKeyTypeId(*b"camm");

	pub mod crypto {
		use super::KEY_TYPE;
		use sp_core::sr25519::Signature as Sr25519Signature;
		use sp_runtime::{
			app_crypto::{app_crypto, sr25519},
			traits::Verify,
			MultiSignature, MultiSigner,
		};
		app_crypto!(sr25519, KEY_TYPE);

		pub struct TestAuthId;

		// implementation for runtime
		impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
			type RuntimeAppPublic = Public;
			type GenericSignature = sp_core::sr25519::Signature;
			type GenericPublic = sp_core::sr25519::Public;
		}

		// implementation for mock runtime in test
		impl
			frame_system::offchain::AppCrypto<
				<Sr25519Signature as Verify>::Signer,
				Sr25519Signature,
			> for TestAuthId
		{
			type RuntimeAppPublic = Public;
			type GenericSignature = sp_core::sr25519::Signature;
			type GenericPublic = sp_core::sr25519::Public;
		}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo;
		type Balance: Default
			+ Parameter
			+ Codec
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ From<u64> // at least 64 bit
			+ Zero
			+ FixedPointOperand
			+ Into<u128>; // cannot do From<u128>, until LiftedFixedBalance integer part is larger than 128
			  // bit
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::event]
	// #[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> Pallet<T> {
		/// Find `ann = amp * n^n` where `amp` - amplification coefficient,
		/// `n` - number of coins.
		pub fn get_ann(amp: FixedU128, n: usize) -> Option<FixedU128> {
			let n_coins = FixedU128::saturating_from_integer(u128::try_from(n).ok()?);
			let mut ann = amp;
			for _ in 0..n {
				ann = ann.checked_mul(&n_coins)?;
			}
			Some(ann)
		}
		/// Find `d` preserving StableSwap invariant.
		/// Here `d` - total amount of coins when they have an equal price,
		/// `xp` - coin amounts, `ann` is amplification coefficient multiplied by `n^n`,
		/// where `n` is number of coins.
		///
		/// # Notes
		///
		/// D invariant calculation in non-overflowing integer operations iteratively
		///
		/// ```pseudocode
		///  A * sum(x_i) * n^n + D = A * D * n^n + D^(n+1) / (n^n * prod(x_i))
		/// ```
		///
		/// Converging solution:
		///
		/// ```pseudocode
		/// D[j + 1] = (A * n^n * sum(x_i) - D[j]^(n+1) / (n^n * prod(x_i))) / (A * n^n - 1)
		/// ```
		pub fn get_d(xp: &[FixedU128], ann: FixedU128) -> Option<FixedU128> {
			let zero = FixedU128::zero();
			let one = FixedU128::one();
			let prec = FixedU128::from_inner(FixedU128::accuracy());
			let n = FixedU128::saturating_from_integer(u128::try_from(xp.len()).ok()?);

			let sum = xp.iter().try_fold(zero, |s, x| s.checked_add(x))?;
			if sum == zero {
				return Some(zero)
			}
			let mut d = sum;

			for _ in 0..255 {
				let mut d_p = d;
				for x in xp.iter() {
					// d_p = d_p * d / (x * n)
					d_p = d_p.checked_mul(&d)?.checked_div(&x.checked_mul(&n)?)?;
				}
				let d_prev = d;

				// d = (ann * sum + d_p * n) * d / ((ann - 1) * d + (n + 1) * d_p)
				d = ann
					.checked_mul(&sum)?
					.checked_add(&d_p.checked_mul(&n)?)?
					.checked_mul(&d)?
					.checked_div(
						&ann.checked_sub(&one)?
							.checked_mul(&d)?
							.checked_add(&n.checked_add(&one)?.checked_mul(&d_p)?)?,
					)?;

				if d > d_prev {
					if d.checked_sub(&d_prev)? <= prec {
						return Some(d)
					}
				} else if d_prev.checked_sub(&d)? <= prec {
					return Some(d)
				}
			}
			None
		}

		/// Here `xp` - coin amounts, `ann` is amplification coefficient multiplied by `n^n`, where
		/// `n` is number of coins.
		///
		/// See https://github.com/equilibrium-eosdt/equilibrium-curve-amm/blob/master/docs/deducing-get_y-formulas.pdf
		/// for detailed explanation about formulas this function uses.
		///
		/// # Notes
		///
		/// Done by solving quadratic equation iteratively.
		///
		/// ```pseudocode
		/// x_1^2 + x_1 * (sum' - (A * n^n - 1) * D / (A * n^n)) = D^(n+1) / (n^2n * prod' * A)
		/// x_1^2 + b * x_1 = c
		///
		/// x_1 = (x_1^2 + c) / (2 * x_1 + b)
		/// ```
		pub fn get_y(
			i: usize,
			j: usize,
			x: FixedU128,
			xp: &[FixedU128],
			ann: FixedU128,
		) -> Option<FixedU128> {
			let zero = FixedU128::zero();
			let one = FixedU128::one();
			let two = FixedU128::saturating_from_integer(u128::try_from(2).ok()?);
			let n = FixedU128::saturating_from_integer(u128::try_from(xp.len()).ok()?);

			// Same coin
			if i == j {
				return None
			}
			// j above n
			if j >= xp.len() {
				return None
			}
			if i >= xp.len() {
				return None
			}
			let d = Self::get_d(xp, ann)?;

			let mut c = d;
			let mut s = zero;

			// Calculate s and c
			// p is implicitly calculated as part of c
			// note that loop makes n - 1 iterations
			for (k, xp_k) in xp.iter().enumerate() {
				let x_k;
				if k == i {
					x_k = x;
				} else if k != j {
					x_k = *xp_k;
				} else {
					continue
				}
				// s = s + x_k
				s = s.checked_add(&x_k)?;
				// c = c * d / (x_k * n)
				c = c.checked_mul(&d)?.checked_div(&x_k.checked_mul(&n)?)?;
			}
			// c = c * d / (ann * n)
			// At this step we have d^n in the numerator of c
			// and n^(n-1) in its denominator.
			// So we multiplying it by remaining d/n
			c = c.checked_mul(&d)?.checked_div(&ann.checked_mul(&n)?)?;

			// b = s + d / ann
			// We subtract d later
			let b = s.checked_add(&d.checked_div(&ann)?)?;
			let mut y = d;

			for _ in 0..255 {
				let y_prev = y;
				// y = (y^2 + c) / (2 * y + b - d)
				// Subtract d to calculate b finally
				y = y
					.checked_mul(&y)?
					.checked_add(&c)?
					.checked_div(&two.checked_mul(&y)?.checked_add(&b)?.checked_sub(&d)?)?;

				// Equality with the specified precision
				if y > y_prev {
					if y.checked_sub(&y_prev)? <= one {
						return Some(y)
					}
				} else if y_prev.checked_sub(&y)? <= one {
					return Some(y)
				}
			}

			None
		}
		/// Here `xp` - coin amounts, `ann` is amplification coefficient multiplied by `n^n`, where
		/// `n` is number of coins.
		/// Calculate `x[i]` if one reduces `d` from being calculated for `xp` to `d`.
		///
		/// # Notes
		///
		/// Done by solving quadratic equation iteratively.
		///
		/// ```pseudocode
		/// x_1^2 + x_1 * (sum' - (A * n^n - 1) * D / (A * n^n)) = D^(n+1) / (n^2n * prod' * A)
		/// x_1^2 + b * x_1 = c
		///
		/// x_1 = (x_1^2 + c) / (2 * x_1 + b)
		/// ```
		pub fn get_y_d(
			i: usize,
			d: FixedU128,
			xp: &[FixedU128],
			ann: FixedU128,
		) -> Option<FixedU128> {
			let prec = FixedU128::from_inner(FixedU128::accuracy());
			let zero = FixedU128::one();
			let two = FixedU128::saturating_from_integer(u128::try_from(2).ok()?);
			let n = FixedU128::saturating_from_integer(u128::try_from(xp.len()).ok()?);

			if i >= xp.len() {
				return None
			}

			let mut c = d;
			let mut s = zero;

			for (k, xp_k) in xp.iter().enumerate() {
				if k == i {
					continue
				}

				let x = xp_k;

				s = s.checked_add(x)?;
				// c = c * d / (x * n)
				c = c.checked_mul(&d)?.checked_div(&x.checked_mul(&n)?)?;
			}
			// c = c * d / (ann * n)
			c = c.checked_mul(&d)?.checked_div(&ann.checked_mul(&n)?)?;
			// b = s + d / ann
			let b = s.checked_add(&d.checked_div(&ann)?)?;
			let mut y = d;

			for _ in 0..255 {
				let y_prev = y;
				// y = (y*y + c) / (2 * y + b - d)
				y = y
					.checked_mul(&y)?
					.checked_add(&c)?
					.checked_div(&two.checked_mul(&y)?.checked_add(&b)?.checked_sub(&d)?)?;

				// Equality with the specified precision
				if y > y_prev {
					if y.checked_sub(&y_prev)? <= prec {
						return Some(y)
					}
				} else if y_prev.checked_sub(&y)? <= prec {
					return Some(y)
				}
			}

			None
		}
	}
}
