//! Overview
//! Allows to add new assets internally. User facing mutating API is provided by other pallets.
#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_method,
		clippy::disallowed_type,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_type)]
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

#[frame_support::pallet]
pub mod pallet {
	use composable_traits::{
		financial_nft::{FinancialNFTProvider, NFTClass},
		math::SafeAdd,
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			tokens::nonfungibles::{Create, Inspect, Mutate, Transfer},
			IsType,
		},
	};
	use sp_runtime::traits::Zero;
	use sp_std::collections::btree_map::BTreeMap;

	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type NFTInstanceId = u128;

	#[pallet::event]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		ClassAlreadyExists,
		InstanceAlreadyExists,
		ClassNotFound,
		InstanceNotFound,
		MustBeOwner,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::type_value]
	pub fn NFTCountOnEmpty<T: Config>() -> NFTInstanceId {
		Zero::zero()
	}

	#[pallet::storage]
	#[pallet::getter(fn nft_count)]
	#[allow(clippy::disallowed_type)]
	pub type NFTCount<T: Config> = StorageValue<_, NFTInstanceId, ValueQuery, NFTCountOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn instance_owner)]
	pub type InstanceOwner<T: Config> =
		StorageMap<_, Blake2_128Concat, (NFTClass, NFTInstanceId), AccountIdOf<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn instance)]
	pub type Instance<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		(NFTClass, NFTInstanceId),
		BTreeMap<Vec<u8>, Vec<u8>>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn class_owners)]
	pub type ClassOwners<T: Config> =
		StorageMap<_, Blake2_128Concat, NFTClass, (AccountIdOf<T>, AccountIdOf<T>), OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn class)]
	pub type Class<T: Config> =
		StorageMap<_, Blake2_128Concat, NFTClass, BTreeMap<Vec<u8>, Vec<u8>>, OptionQuery>;

	impl<T: Config> Pallet<T> {
		fn ensure_instance_exists(
			class: &NFTClass,
			instance: &NFTInstanceId,
		) -> Result<(), DispatchError> {
			ensure!(
				InstanceOwner::<T>::contains_key((class, instance)),
				Error::<T>::InstanceNotFound
			);
			Ok(())
		}

		fn ensure_class_exists(class: &NFTClass) -> Result<(), DispatchError> {
			ensure!(ClassOwners::<T>::contains_key(class), Error::<T>::ClassNotFound);
			Ok(())
		}
	}

	impl<T: Config> Inspect<AccountIdOf<T>> for Pallet<T> {
		type ClassId = NFTClass;
		type InstanceId = NFTInstanceId;

		fn owner(class: &Self::ClassId, instance: &Self::InstanceId) -> Option<AccountIdOf<T>> {
			Self::instance_owner((class, instance))
		}

		fn attribute(
			class: &Self::ClassId,
			instance: &Self::InstanceId,
			key: &[u8],
		) -> Option<Vec<u8>> {
			Instance::<T>::get((class, instance)).and_then(|x| x.get(key).cloned())
		}

		fn class_attribute(class: &Self::ClassId, key: &[u8]) -> Option<Vec<u8>> {
			Class::<T>::get(class).and_then(|x| x.get(key).cloned())
		}
	}

	impl<T: Config> Create<AccountIdOf<T>> for Pallet<T> {
		fn create_class(
			class: &Self::ClassId,
			who: &AccountIdOf<T>,
			admin: &AccountIdOf<T>,
		) -> DispatchResult {
			ensure!(Self::class_owners(class).is_none(), Error::<T>::ClassAlreadyExists);
			ClassOwners::<T>::insert(class, (who, admin));
			Ok(())
		}
	}

	impl<T: Config> Transfer<AccountIdOf<T>> for Pallet<T> {
		fn transfer(
			class: &Self::ClassId,
			instance: &Self::InstanceId,
			destination: &AccountIdOf<T>,
		) -> DispatchResult {
			Self::ensure_instance_exists(class, instance)?;
			InstanceOwner::<T>::insert((class, instance), destination);
			Ok(())
		}
	}

	impl<T: Config> Mutate<AccountIdOf<T>> for Pallet<T> {
		fn mint_into(
			class: &Self::ClassId,
			instance: &Self::InstanceId,
			who: &AccountIdOf<T>,
		) -> DispatchResult {
			ensure!(
				Self::instance_owner((class, instance)).is_none(),
				Error::<T>::InstanceAlreadyExists
			);
			InstanceOwner::<T>::insert((class, instance), who);
			Ok(())
		}

		fn burn_from(class: &Self::ClassId, instance: &Self::InstanceId) -> DispatchResult {
			Self::ensure_instance_exists(class, instance)?;
			Instance::<T>::remove((class, instance));
			Ok(())
		}

		fn set_attribute(
			class: &Self::ClassId,
			instance: &Self::InstanceId,
			key: &[u8],
			value: &[u8],
		) -> DispatchResult {
			Self::ensure_instance_exists(class, instance)?;
			Instance::<T>::mutate((class, instance), |entry| {
				entry.as_mut().map(|nft_instance| nft_instance.insert(key.into(), value.into()));
			});
			Ok(())
		}

		fn set_typed_attribute<K: Encode, V: Encode>(
			class: &Self::ClassId,
			instance: &Self::InstanceId,
			key: &K,
			value: &V,
		) -> DispatchResult {
			key.using_encoded(|k| {
				value.using_encoded(|v| Self::set_attribute(class, instance, k, v))
			})
		}

		fn set_class_attribute(class: &Self::ClassId, key: &[u8], value: &[u8]) -> DispatchResult {
			Self::ensure_class_exists(class)?;
			Class::<T>::mutate(class, |entry| {
				entry.as_mut().map(|nft_class| nft_class.insert(key.into(), value.into()));
			});
			Ok(())
		}

		fn set_typed_class_attribute<K: Encode, V: Encode>(
			class: &Self::ClassId,
			key: &K,
			value: &V,
		) -> DispatchResult {
			key.using_encoded(|k| value.using_encoded(|v| Self::set_class_attribute(class, k, v)))
		}
	}

	impl<T: Config> FinancialNFTProvider<AccountIdOf<T>> for Pallet<T> {
		fn mint_nft<K: Encode, V: Encode>(
			class: &Self::ClassId,
			who: &AccountIdOf<T>,
			key: &K,
			value: &V,
		) -> Result<Self::InstanceId, DispatchError> {
			let instance = NFTCount::<T>::try_mutate(|x| -> Result<u128, DispatchError> {
				let id = *x;
				*x = x.safe_add(&1)?;
				Ok(id)
			})?;
			Self::mint_into(class, &instance, who)?;
			Self::set_typed_attribute(class, &instance, key, value)?;
			Ok(instance)
		}
	}
}
