// Copyright 2020-2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Pallet to spam the XCM/UMP.

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
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]

use cumulus_pallet_xcm::{ensure_sibling_para, Origin as CumulusOrigin};
use cumulus_primitives_core::ParaId;
use frame_system::Config as SystemConfig;
use sp_runtime::traits::Saturating;
use sp_std::prelude::*;
use xcm::latest::prelude::*;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Origin: From<<Self as SystemConfig>::Origin>
			+ Into<Result<CumulusOrigin, <Self as Config>::Origin>>;

		/// The overarching call type; we assume sibling chains use the same type.
		type Call: From<Call<Self>> + Encode;

		type XcmSender: SendXcm;

		#[pallet::constant]
		type MaxTargets: Get<u32>;

		#[pallet::constant]
		type MaxPayload: Get<u32>;
	}

	/// The target parachains to ping.
	#[pallet::storage]
	// Targets is an empty BoundedVec by default, which causes the pallet not to ping any targets.
	#[allow(clippy::disallowed_types)]
	pub(super) type Targets<T: Config> = StorageValue<
		_,
		BoundedVec<(ParaId, BoundedVec<u8, T::MaxPayload>), T::MaxTargets>,
		ValueQuery,
	>;

	/// The total number of pings sent.
	#[pallet::storage]
	// Absence of PingCount is equivalent to 0, so ValueQuery is valid here.
	#[allow(clippy::disallowed_types)]
	pub(super) type PingCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// The sent pings.
	#[pallet::storage]
	pub(super) type Pings<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, T::BlockNumber, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		PingSent(ParaId, u32, Vec<u8>),
		Pinged(ParaId, u32, Vec<u8>),
		PongSent(ParaId, u32, Vec<u8>),
		Ponged(ParaId, u32, Vec<u8>, T::BlockNumber),
		ErrorSendingPing(SendError, ParaId, u32, Vec<u8>),
		ErrorSendingPong(SendError, ParaId, u32, Vec<u8>),
		UnknownPong(ParaId, u32, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		MaxPayload,
		MaxTargets,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(n: T::BlockNumber) {
			for (para, payload) in Targets::<T>::get().into_iter() {
				let seq = PingCount::<T>::mutate(|seq| {
					*seq += 1;
					*seq
				});
				match T::XcmSender::send_xcm(
					(1, Junction::Parachain(para.into())),
					Xcm(vec![Transact {
						origin_type: OriginKind::Native,
						require_weight_at_most: 1_000,
						call: <T as Config>::Call::from(Call::<T>::ping {
							seq,
							payload: payload.to_vec(),
						})
						.encode()
						.into(),
					}]),
				) {
					Ok(()) => {
						Pings::<T>::insert(seq, n);
						Self::deposit_event(Event::PingSent(para, seq, payload.to_vec()));
					},
					Err(e) => {
						Self::deposit_event(Event::ErrorSendingPing(
							e,
							para,
							seq,
							payload.to_vec(),
						));
					},
				}
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn start(origin: OriginFor<T>, para: ParaId, payload: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;
			let payload = payload.try_into().map_err(|_| Error::<T>::MaxPayload)?;
			Targets::<T>::mutate(|t| -> DispatchResult {
				t.try_push((para, payload)).map_err(|_| Error::<T>::MaxTargets)?;
				Ok(())
			})?;

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn start_many(
			origin: OriginFor<T>,
			para: ParaId,
			count: u32,
			payload: Vec<u8>,
		) -> DispatchResult {
			ensure_root(origin)?;
			let payload = BoundedVec::try_from(payload).map_err(|_| Error::<T>::MaxPayload)?;
			for _ in 0..count {
				Targets::<T>::try_mutate(|t| -> DispatchResult {
					t.try_push((para, payload.clone())).map_err(|_| Error::<T>::MaxTargets)?;
					Ok(())
				})?;
			}
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn stop(origin: OriginFor<T>, para: ParaId) -> DispatchResult {
			ensure_root(origin)?;
			Targets::<T>::mutate(|t| {
				if let Some(p) = t.iter().position(|(p, _)| p == &para) {
					t.swap_remove(p);
				}
			});
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn stop_all(origin: OriginFor<T>, maybe_para: Option<ParaId>) -> DispatchResult {
			ensure_root(origin)?;
			if let Some(para) = maybe_para {
				Targets::<T>::mutate(|t| t.retain(|&(x, _)| x != para));
			} else {
				Targets::<T>::kill();
			}
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn ping(origin: OriginFor<T>, seq: u32, payload: Vec<u8>) -> DispatchResult {
			// Only accept pings from other chains.
			let para = ensure_sibling_para(<T as Config>::Origin::from(origin))?;

			Self::deposit_event(Event::Pinged(para, seq, payload.clone()));
			match T::XcmSender::send_xcm(
				(1, Junction::Parachain(para.into())),
				Xcm(vec![Transact {
					origin_type: OriginKind::Native,
					require_weight_at_most: 1_000,
					call: <T as Config>::Call::from(Call::<T>::pong {
						seq,
						payload: payload.clone(),
					})
					.encode()
					.into(),
				}]),
			) {
				Ok(()) => Self::deposit_event(Event::PongSent(para, seq, payload)),
				Err(e) => Self::deposit_event(Event::ErrorSendingPong(e, para, seq, payload)),
			}
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn pong(origin: OriginFor<T>, seq: u32, payload: Vec<u8>) -> DispatchResult {
			// Only accept pings from other chains.
			let para = ensure_sibling_para(<T as Config>::Origin::from(origin))?;

			if let Some(sent_at) = Pings::<T>::take(seq) {
				Self::deposit_event(Event::Ponged(
					para,
					seq,
					payload,
					frame_system::Pallet::<T>::block_number().saturating_sub(sent_at),
				));
			} else {
				// Pong received for a ping we apparently didn't send?!
				Self::deposit_event(Event::UnknownPong(para, seq, payload));
			}
			Ok(())
		}
	}
}
