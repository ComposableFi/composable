//! Overview
//! Allows to add new assets internally. User facing mutating API is provided by other pallets.
#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
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

#[cfg(test)]
mod test;

use codec::FullCodec;
use frame_support::{
	pallet_prelude::{OptionQuery, StorageMap},
	storage::types::QueryKindTrait,
	traits::{Get, StorageInstance},
	StorageHasher,
};
pub use pallet::*;
use sp_runtime::DispatchError;

#[frame_support::pallet]
pub mod pallet {
	use composable_support::math::safe::SafeAdd;
	use composable_traits::financial_nft::{FinancialNftProvider, NftClass};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			tokens::nonfungibles::{Create, Inspect, Mutate, Transfer},
			IsType,
		},
	};
	use sp_runtime::traits::Zero;
	use sp_std::collections::{btree_map::BTreeMap, btree_set::BTreeSet};

	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type NftInstanceId = u128;

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		NftCreated { class_id: NftClass, instance_id: NftInstanceId },
		NftBurned { class_id: NftClass, instance_id: NftInstanceId },
		NftTransferred { class_id: NftClass, instance_id: NftInstanceId, to: AccountIdOf<T> },
	}

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

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	pub type NftId<T: Config> =
		StorageMap<_, Blake2_128Concat, NftClass, NftInstanceId, ValueQuery, NftIdOnEmpty<T>>;

	#[pallet::type_value]
	pub fn NftIdOnEmpty<T: Config>() -> NftInstanceId {
		Zero::zero()
	}

	#[pallet::storage]
	#[pallet::getter(fn instance)]
	pub type Instance<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		(NftClass, NftInstanceId),
		(AccountIdOf<T>, BTreeMap<Vec<u8>, Vec<u8>>),
		OptionQuery,
	>;

	/// Map of NFT classes to all of the instances of that class.
	#[pallet::storage]
	#[pallet::getter(fn class_instances)]
	pub type ClassInstances<T: Config> =
		StorageMap<_, Blake2_128Concat, NftClass, BTreeSet<NftInstanceId>, OptionQuery>;

	/// All the NFTs owned by an account.
	#[pallet::storage]
	#[pallet::getter(fn owner_instances)]
	pub type OwnerInstances<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		AccountIdOf<T>,
		BTreeSet<(NftClass, NftInstanceId)>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn class)]
	pub type Class<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		NftClass,
		// (who, admin, data)
		(AccountIdOf<T>, AccountIdOf<T>, BTreeMap<Vec<u8>, Vec<u8>>),
		OptionQuery,
	>;

	impl<T: Config> Inspect<AccountIdOf<T>> for Pallet<T> {
		type ClassId = NftClass;
		type InstanceId = NftInstanceId;

		fn owner(class: &Self::ClassId, instance: &Self::InstanceId) -> Option<AccountIdOf<T>> {
			Instance::<T>::get((class, instance)).map(|(owner, _)| owner)
		}

		fn attribute(
			class: &Self::ClassId,
			instance: &Self::InstanceId,
			key: &[u8],
		) -> Option<Vec<u8>> {
			Instance::<T>::get((class, instance))
				.and_then(|(_, attributes)| attributes.get(key).cloned())
		}

		fn class_attribute(class: &Self::ClassId, key: &[u8]) -> Option<Vec<u8>> {
			Class::<T>::get(class).and_then(|(_, _, attributes)| attributes.get(key).cloned())
		}
	}

	impl<T: Config> Create<AccountIdOf<T>> for Pallet<T> {
		fn create_class(
			class: &Self::ClassId,
			who: &AccountIdOf<T>,
			admin: &AccountIdOf<T>,
		) -> DispatchResult {
			ensure!(Class::<T>::get(class).is_none(), Error::<T>::ClassAlreadyExists);
			Class::<T>::insert(class, (who, admin, BTreeMap::<Vec<u8>, Vec<u8>>::new()));
			Ok(())
		}
	}

	impl<T: Config> Transfer<AccountIdOf<T>> for Pallet<T> {
		fn transfer(
			class: &Self::ClassId,
			instance: &Self::InstanceId,
			destination: &AccountIdOf<T>,
		) -> DispatchResult {
			Instance::<T>::try_mutate((class, instance), |entry| match entry {
				Some((owner, _)) => {
					OwnerInstances::<T>::mutate(owner.clone(), |x| match x {
						Some(owner_instances) => {
							let was_previously_owned = owner_instances.remove(&(*class, *instance));
							debug_assert!(was_previously_owned);
							Ok(())
						},
						None => Err(Error::<T>::InstanceNotFound),
					})?;

					OwnerInstances::<T>::mutate(
						destination.clone(),
						insert_or_init_and_insert((*class, *instance)),
					);
					*owner = destination.clone();
					Self::deposit_event(Event::NftTransferred {
						class_id: *class,
						instance_id: *instance,
						to: destination.clone(),
					});
					Ok(())
				},
				None => Err(Error::<T>::InstanceNotFound.into()),
			})
		}
	}

	impl<T: Config> Mutate<AccountIdOf<T>> for Pallet<T> {
		fn mint_into(
			class: &Self::ClassId,
			instance: &Self::InstanceId,
			who: &AccountIdOf<T>,
		) -> DispatchResult {
			ensure!(Self::instance((class, instance)).is_none(), Error::<T>::InstanceAlreadyExists);
			Instance::<T>::insert((class, instance), (who, BTreeMap::<Vec<u8>, Vec<u8>>::new()));
			ClassInstances::<T>::mutate(class, insert_or_init_and_insert(*instance));
			OwnerInstances::<T>::mutate(who, insert_or_init_and_insert((*class, *instance)));

			Self::deposit_event(Event::NftCreated { class_id: *class, instance_id: *instance });

			Ok(())
		}

		fn burn_from(class: &Self::ClassId, instance: &Self::InstanceId) -> DispatchResult {
			Instance::<T>::try_mutate_exists((class, instance), |entry| -> DispatchResult {
				match entry {
					Some((owner, _)) => {
						OwnerInstances::<T>::mutate(owner, |x| match x {
							Some(instances) => {
								instances.remove(&(*class, *instance));
							},
							None => {},
						});
						*entry = None;
						Ok(())
					},
					None => Err(Error::<T>::InstanceNotFound.into()),
				}
			})?;
			ClassInstances::<T>::mutate(class, |x| match x {
				Some(instances) => {
					instances.remove(instance);
				},
				None => {},
			});
			Ok(())
		}

		fn set_attribute(
			class: &Self::ClassId,
			instance: &Self::InstanceId,
			key: &[u8],
			value: &[u8],
		) -> DispatchResult {
			Instance::<T>::try_mutate((class, instance), |entry| match entry {
				Some((_, nft)) => {
					nft.insert(key.into(), value.into());
					Ok(())
				},
				None => Err(Error::<T>::InstanceNotFound.into()),
			})
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
			Class::<T>::try_mutate(class, |entry| match entry {
				Some((_, _, class)) => {
					class.insert(key.into(), value.into());
					Ok(())
				},
				None => Err(Error::<T>::ClassNotFound.into()),
			})
		}

		fn set_typed_class_attribute<K: Encode, V: Encode>(
			class: &Self::ClassId,
			key: &K,
			value: &V,
		) -> DispatchResult {
			key.using_encoded(|k| value.using_encoded(|v| Self::set_class_attribute(class, k, v)))
		}
	}

	impl<T: Config> FinancialNftProvider<AccountIdOf<T>> for Pallet<T> {
		fn mint_nft<K: Encode, V: Encode>(
			class: &Self::ClassId,
			who: &AccountIdOf<T>,
			key: &K,
			value: &V,
		) -> Result<Self::InstanceId, DispatchError> {
			let instance = NftId::<T>::try_mutate(class, |x| -> Result<u128, DispatchError> {
				let id = *x;
				*x = x.safe_add(&1)?;
				Ok(id)
			})?;
			Self::mint_into(class, &instance, who)?;
			Self::set_typed_attribute(class, &instance, key, value)?;
			Ok(instance)
		}
	}

	/// Returns a closure that inserts the given value into the contained set, initializing the set
	/// if the `Option` is `None`.
	fn insert_or_init_and_insert<T: Ord>(t: T) -> impl FnOnce(&'_ mut Option<BTreeSet<T>>) -> () {
		move |x: &mut Option<BTreeSet<T>>| match x {
			Some(instances) => {
				instances.insert(t);
			},
			None => {
				x.replace([t].into());
			},
		}
	}
}

pub trait MutateBoth<K1, K2, V1, V2> {
	fn mutate_both(
		k1: K1,
		k2: K2,
		f: impl FnOnce(&mut V1, &mut V2) -> Result<(V1, V2), DispatchError>,
	) -> Result<(V1, V2), DispatchError>;
}

impl<Prefix, Hasher, K1, K2, V1, V2> MutateBoth<K1, K2, V1, V2>
	for (
		StorageMap<Prefix, Hasher, OptionQuery, K1, V1>,
		StorageMap<Prefix, Hasher, OptionQuery, K2, V2>,
	) where
	Prefix: StorageInstance,
	Hasher: StorageHasher,
	K1: FullCodec,
	K2: FullCodec,
	V1: FullCodec,
	V2: FullCodec,
{
	fn mutate_both(
		k1: K1,
		k2: K2,
		f: impl FnOnce(&mut V1, &mut V2) -> Result<(V1, V2), DispatchError>,
	) -> Result<(V1, V2), DispatchError> {
		StorageMap::<Prefix, Hasher, K1, V1, OptionQuery>::mutate(k1, |v1| {
			if let Some(v1) = v1 {
				StorageMap::<Prefix, Hasher, K2, V2, OptionQuery>::mutate(k2, |v2| {
					if let Some(v2) = v2 {
						f(v1, v2)
					} else {
					}
				})
			}
		})
	}
}
