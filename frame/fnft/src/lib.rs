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
	use codec::FullCodec;
	use composable_support::math::safe::SafeAdd;
	use composable_traits::{
		account_proxy::AccountProxy,
		currency::AssetIdLike,
		fnft::{FinancialNft, FnftAccountProxyTypeSelector},
	};
	use core::fmt::Debug;
	use frame_support::{
		pallet_prelude::*,
		traits::{
			tokens::nonfungibles::{Create, Inspect, Mutate, Transfer},
			IsType,
		},
		PalletId,
	};
	use sp_arithmetic::traits::One;
	use sp_runtime::traits::{AccountIdConversion, Zero};
	use sp_std::{
		collections::{btree_map::BTreeMap, btree_set::BTreeSet},
		vec::Vec,
	};

	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type FinancialNftCollectionIdOf<T> = <T as Config>::FinancialNftCollectionId;
	pub(crate) type FinancialNftInstanceIdOf<T> = <T as Config>::FinancialNftInstanceId;

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		FinancialNftCreated {
			collection_id: FinancialNftCollectionIdOf<T>,
			instance_id: FinancialNftInstanceIdOf<T>,
		},
		FinancialNftBurned {
			collection_id: FinancialNftCollectionIdOf<T>,
			instance_id: FinancialNftInstanceIdOf<T>,
		},
		FinancialNftTransferred {
			collection_id: FinancialNftCollectionIdOf<T>,
			instance_id: FinancialNftInstanceIdOf<T>,
			to: AccountIdOf<T>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		CollectionAlreadyExists,
		InstanceAlreadyExists,
		CollectionNotFound,
		InstanceNotFound,
		MustBeOwner,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type MaxProperties: Get<u32>;

		type FinancialNftCollectionId: Parameter
			+ Member
			+ AssetIdLike
			+ MaybeSerializeDeserialize
			+ Ord
			+ Into<u128>;

		type FinancialNftInstanceId: FullCodec
			+ Debug
			+ SafeAdd
			+ MaxEncodedLen
			+ Default
			+ TypeInfo
			+ Eq
			+ PartialEq
			+ Ord
			+ Copy
			+ Zero
			+ One;

		type ProxyType: Parameter + Member + Ord + PartialOrd + Default + MaxEncodedLen;

		/// Used for setting the owning account of a fNFT as the delegate for the fNFT asset_account
		type AccountProxy: AccountProxy<
			AccountId = Self::AccountId,
			ProxyType = Self::ProxyType,
			BlockNumber = Self::BlockNumber,
		>;

		type ProxyTypeSelector: FnftAccountProxyTypeSelector<Self::ProxyType>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	pub type FinancialNftId<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		FinancialNftCollectionIdOf<T>,
		FinancialNftInstanceIdOf<T>,
		ValueQuery,
		NftIdOnEmpty<T>,
	>;

	#[pallet::type_value]
	pub fn NftIdOnEmpty<T: Config>() -> FinancialNftInstanceIdOf<T> {
		Zero::zero()
	}

	#[pallet::storage]
	#[pallet::getter(fn instance)]
	pub type Instance<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		(FinancialNftCollectionIdOf<T>, FinancialNftInstanceIdOf<T>),
		(AccountIdOf<T>, BTreeMap<Vec<u8>, Vec<u8>>),
		OptionQuery,
	>;

	/// Map of NFT collections to all of the instances of that collection.
	#[pallet::storage]
	#[pallet::getter(fn collection_instances)]
	pub type CollectionInstances<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		FinancialNftCollectionIdOf<T>,
		BTreeSet<FinancialNftInstanceIdOf<T>>,
		OptionQuery,
	>;

	/// All the NFTs owned by an account.
	#[pallet::storage]
	#[pallet::getter(fn owner_instances)]
	pub type OwnerInstances<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		AccountIdOf<T>,
		BTreeSet<(FinancialNftCollectionIdOf<T>, FinancialNftInstanceIdOf<T>)>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn collection)]
	pub type Collection<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		FinancialNftCollectionIdOf<T>,
		// (who, admin, data)
		(AccountIdOf<T>, AccountIdOf<T>, BTreeMap<Vec<u8>, Vec<u8>>),
		OptionQuery,
	>;

	impl<T: Config> Inspect<AccountIdOf<T>> for Pallet<T> {
		type ItemId = FinancialNftInstanceIdOf<T>;
		type CollectionId = FinancialNftCollectionIdOf<T>;

		fn owner(
			collection: &Self::CollectionId,
			instance: &Self::ItemId,
		) -> Option<AccountIdOf<T>> {
			Instance::<T>::get((collection, instance)).map(|(owner, _)| owner)
		}

		fn attribute(
			collection: &Self::CollectionId,
			instance: &Self::ItemId,
			key: &[u8],
		) -> Option<Vec<u8>> {
			Instance::<T>::get((collection, instance))
				.and_then(|(_, instance_attributes)| instance_attributes.get(key).cloned())
		}

		fn collection_attribute(collection: &Self::CollectionId, key: &[u8]) -> Option<Vec<u8>> {
			Collection::<T>::get(collection)
				.and_then(|(_, _, attributes)| attributes.get(key).cloned())
		}
	}

	impl<T: Config> Create<AccountIdOf<T>> for Pallet<T> {
		fn create_collection(
			collection: &Self::CollectionId,
			who: &AccountIdOf<T>,
			admin: &AccountIdOf<T>,
		) -> DispatchResult {
			ensure!(
				Collection::<T>::get(collection).is_none(),
				Error::<T>::CollectionAlreadyExists
			);
			Collection::<T>::insert(collection, (who, admin, BTreeMap::<Vec<u8>, Vec<u8>>::new()));
			Ok(())
		}
	}

	impl<T: Config> Transfer<AccountIdOf<T>> for Pallet<T> {
		fn transfer(
			collection: &Self::CollectionId,
			instance: &Self::ItemId,
			destination: &AccountIdOf<T>,
		) -> DispatchResult {
			Instance::<T>::try_mutate((collection, instance), |entry| match entry {
				Some((owner, _)) => {
					OwnerInstances::<T>::mutate(owner.clone(), |x| match x {
						Some(owner_instances) => {
							let was_previously_owned =
								owner_instances.remove(&(*collection, *instance));
							debug_assert!(was_previously_owned);
							Ok(())
						},
						// theoretically, this branch should never be reached
						None => Err(Error::<T>::InstanceNotFound),
					})?;

					OwnerInstances::<T>::mutate(
						destination.clone(),
						insert_or_init_and_insert((*collection, *instance)),
					);
					*owner = destination.clone();

					// TODO (vim): Re-proxy NFT account to the transferee
					Self::deposit_event(Event::FinancialNftTransferred {
						collection_id: *collection,
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
			collection: &Self::CollectionId,
			instance: &Self::ItemId,
			who: &AccountIdOf<T>,
		) -> DispatchResult {
			ensure!(
				Self::instance((collection, instance)).is_none(),
				Error::<T>::InstanceAlreadyExists
			);
			Instance::<T>::insert(
				(collection, instance),
				(who, BTreeMap::<Vec<u8>, Vec<u8>>::new()),
			);
			CollectionInstances::<T>::mutate(collection, insert_or_init_and_insert(*instance));
			OwnerInstances::<T>::mutate(who, insert_or_init_and_insert((*collection, *instance)));

			// Set the owner as the proxy for certain types of actions for the financial NFT account
			// TODO (vim): Make sure that asset_account has the min deposit for proxying in the
			// runtime
			let asset_account =
				<Self as FinancialNft<AccountIdOf<T>>>::asset_account(collection, instance);
			for proxy_type in T::ProxyTypeSelector::get_proxy_types() {
				T::AccountProxy::add_proxy_delegate(
					&asset_account,
					who.clone(),
					proxy_type.clone(),
					frame_system::Pallet::<T>::block_number(),
				)?;
			}

			Self::deposit_event(Event::FinancialNftCreated {
				collection_id: *collection,
				instance_id: *instance,
			});

			Ok(())
		}

		fn burn(
			collection: &Self::CollectionId,
			instance: &Self::ItemId,
			_maybe_check_owner: Option<&AccountIdOf<T>>,
		) -> DispatchResult {
			Instance::<T>::try_mutate_exists((collection, instance), |entry| -> DispatchResult {
				match entry {
					Some((owner, _)) => {
						OwnerInstances::<T>::mutate(owner, |x| match x {
							Some(instances) => {
								instances.remove(&(*collection, *instance));
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
			CollectionInstances::<T>::mutate(collection, |x| match x {
				Some(instances) => {
					instances.remove(instance);
				},
				None => {
					debug_assert!(false, "unreachable")
				},
			});

			// TODO (vim): Remove account proxy ??
			Self::deposit_event(Event::FinancialNftBurned {
				collection_id: *collection,
				instance_id: *instance,
			});

			Ok(())
		}

		fn set_attribute(
			collection: &Self::CollectionId,
			instance: &Self::ItemId,
			key: &[u8],
			value: &[u8],
		) -> DispatchResult {
			Instance::<T>::try_mutate((collection, instance), |entry| match entry {
				Some((_, nft)) => {
					nft.insert(key.into(), value.into());
					Ok(())
				},
				None => Err(Error::<T>::InstanceNotFound.into()),
			})
		}

		fn set_typed_attribute<K: Encode, V: Encode>(
			collection: &Self::CollectionId,
			instance: &Self::ItemId,
			key: &K,
			value: &V,
		) -> DispatchResult {
			key.using_encoded(|k| {
				value.using_encoded(|v| Self::set_attribute(collection, instance, k, v))
			})
		}

		fn set_collection_attribute(
			collection: &Self::CollectionId,
			key: &[u8],
			value: &[u8],
		) -> DispatchResult {
			Collection::<T>::try_mutate(collection, |entry| match entry {
				Some((_, _, collection)) => {
					collection.insert(key.into(), value.into());
					Ok(())
				},
				None => Err(Error::<T>::CollectionNotFound.into()),
			})
		}

		fn set_typed_collection_attribute<K: Encode, V: Encode>(
			collection: &Self::CollectionId,
			key: &K,
			value: &V,
		) -> DispatchResult {
			key.using_encoded(|k| {
				value.using_encoded(|v| Self::set_collection_attribute(collection, k, v))
			})
		}
	}

	impl<T: Config> FinancialNft<AccountIdOf<T>> for Pallet<T> {
		/// TODO (vim): Assess the probability of collision in generating accounts
		///   with collection id type u128 and instance id type u128 this definitely collides
		///   because the seed is longer than the account ID
		fn asset_account(
			collection: &Self::CollectionId,
			instance: &Self::ItemId,
		) -> AccountIdOf<T> {
			T::PalletId::get().into_sub_account_truncating((collection, instance))
		}

		fn get_next_nft_id(
			collection: &<Self as Inspect<AccountIdOf<T>>>::CollectionId,
		) -> Result<Self::ItemId, DispatchError> {
			FinancialNftId::<T>::try_mutate(
				collection,
				|x| -> Result<FinancialNftInstanceIdOf<T>, DispatchError> {
					let id = *x;
					*x = x.safe_add(&FinancialNftInstanceIdOf::<T>::one())?;
					Ok(id)
				},
			)
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
