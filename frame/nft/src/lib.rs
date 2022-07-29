//! Overview
//! Allows to add new assets internally. User facing mutating API is provided by other pallets.
#![cfg_attr(
	not(test),
	deny(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![deny(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
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

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use composable_support::math::safe::SafeAdd;
	use composable_traits::nft::{Key, NftClass, ReferenceNft, Value};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			tokens::nonfungibles::{Create, Inspect, Mutate, Transfer},
			IsType,
		},
	};
	use sp_runtime::traits::Zero;
	use sp_std::{
		collections::{btree_map::BTreeMap, btree_set::BTreeSet},
		vec::Vec,
	};

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
		type MaxProperties: Get<u32>;
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

	impl<T: Config> Pallet<T> {
		#[allow(dead_code)] // TODO: remove it when it will be used
		pub(crate) fn get_next_nft_id(
			class: &<Self as Inspect<AccountIdOf<T>>>::CollectionId,
		) -> Result<u128, DispatchError> {
			NftId::<T>::try_mutate(class, |x| -> Result<u128, DispatchError> {
				let id = *x;
				*x = x.safe_add(&1)?;
				Ok(id)
			})
		}
	}

	impl<T: Config> Inspect<AccountIdOf<T>> for Pallet<T> {
		type CollectionId = NftClass;
		type ItemId = NftInstanceId;

		fn owner(class: &Self::CollectionId, instance: &Self::ItemId) -> Option<AccountIdOf<T>> {
			Instance::<T>::get((class, instance)).map(|(owner, _)| owner)
		}

		fn attribute(
			class: &Self::CollectionId,
			instance: &Self::ItemId,
			key: &[u8],
		) -> Option<Vec<u8>> {
			Instance::<T>::get((class, instance))
				.and_then(|(_, instance_attributes)| instance_attributes.get(key).cloned())
		}

		fn collection_attribute(class: &Self::CollectionId, key: &[u8]) -> Option<Vec<u8>> {
			Class::<T>::get(class).and_then(|(_, _, attributes)| attributes.get(key).cloned())
		}
	}

	impl<T: Config> Create<AccountIdOf<T>> for Pallet<T> {
		fn create_collection(
			class: &Self::CollectionId,
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
			class: &Self::CollectionId,
			instance: &Self::ItemId,
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
						// theoretically, this branch should never be reached
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
			class: &Self::CollectionId,
			instance: &Self::ItemId,
			who: &AccountIdOf<T>,
		) -> DispatchResult {
			ensure!(Self::instance((class, instance)).is_none(), Error::<T>::InstanceAlreadyExists);
			Instance::<T>::insert((class, instance), (who, BTreeMap::<Vec<u8>, Vec<u8>>::new()));
			ClassInstances::<T>::mutate(class, insert_or_init_and_insert(*instance));
			OwnerInstances::<T>::mutate(who, insert_or_init_and_insert((*class, *instance)));

			Self::deposit_event(Event::NftCreated { class_id: *class, instance_id: *instance });

			Ok(())
		}

		fn burn(
			class: &Self::CollectionId,
			instance: &Self::ItemId,
			_maybe_check_owner: Option<&AccountIdOf<T>>,
		) -> DispatchResult {
			Instance::<T>::try_mutate_exists((class, instance), |entry| -> DispatchResult {
				match entry {
					Some((owner, _)) => {
						OwnerInstances::<T>::mutate(owner, |x| match x {
							Some(instances) => {
								instances.remove(&(*class, *instance));
							},
							None => {
								debug_assert!(false, "unreachable")
							},
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
				None => {
					debug_assert!(false, "unreachable")
				},
			});

			Self::deposit_event(Event::NftBurned { class_id: *class, instance_id: *instance });

			Ok(())
		}

		fn set_attribute(
			class: &Self::CollectionId,
			instance: &Self::ItemId,
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
			class: &Self::CollectionId,
			instance: &Self::ItemId,
			key: &K,
			value: &V,
		) -> DispatchResult {
			key.using_encoded(|k| {
				value.using_encoded(|v| Self::set_attribute(class, instance, k, v))
			})
		}

		fn set_collection_attribute(
			class: &Self::CollectionId,
			key: &[u8],
			value: &[u8],
		) -> DispatchResult {
			Class::<T>::try_mutate(class, |entry| match entry {
				Some((_, _, class)) => {
					class.insert(key.into(), value.into());
					Ok(())
				},
				None => Err(Error::<T>::ClassNotFound.into()),
			})
		}

		fn set_typed_collection_attribute<K: Encode, V: Encode>(
			class: &Self::CollectionId,
			key: &K,
			value: &V,
		) -> DispatchResult {
			key.using_encoded(|k| {
				value.using_encoded(|v| Self::set_collection_attribute(class, k, v))
			})
		}
	}

	impl<T: Config> ReferenceNft<T::AccountId> for Pallet<T> {
		type MaxProperties = T::MaxProperties;

		fn reference_mint_into<NFTProvider, NFT>(
			_class: &Self::CollectionId,
			_instance: &Self::ItemId,
			_who: &T::AccountId,
			_reference: composable_traits::nft::Reference,
		) -> DispatchResult {
			Err(DispatchError::Other("no implemented"))
		}

		fn mint_new_into(
			_class: &Self::CollectionId,
			_who: &T::AccountId,
			_properties: frame_support::BoundedBTreeMap<Key, Value, Self::MaxProperties>,
		) -> Result<Self::ItemId, DispatchError> {
			Err(DispatchError::Other("no implemented"))
		}
	}

	/// Returns a closure that inserts the given value into the contained set, initializing the set
	/// if the `Option` is `None`.
	fn insert_or_init_and_insert<T: Ord>(t: T) -> impl FnOnce(&'_ mut Option<BTreeSet<T>>) {
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
