// Copyright 2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot. If not, see <http://www.gnu.org/licenses/>.

//! Track configurations for governance.

use super::*;

const fn percent(x: i32) -> sp_runtime::FixedI64 {
	sp_runtime::FixedI64::from_rational(x as u128, 100)
}

const fn permill(x: i32) -> sp_runtime::FixedI64 {
	sp_runtime::FixedI64::from_rational(x as u128, 1000)
}

pub const ONE_PICA: Balance = 1_000_000_000_000;

use pallet_referenda::Curve;
const TRACKS_DATA: [(u16, pallet_referenda::TrackInfo<Balance, BlockNumber>); 5] = [
	(
		0,
		pallet_referenda::TrackInfo {
			// Name of this track.
			name: "root",
			// A limit for the number of referenda on this track that can be being decided at once.
			// For Root origin this should generally be just one.
			max_deciding: 5,
			// Amount that must be placed on deposit before a decision can be made.
			#[cfg(feature = "fastnet")]
			decision_deposit: 50 * ONE_PICA,
			#[cfg(not(feature = "fastnet"))]
			decision_deposit: 5_000_000 * ONE_PICA,
			// Amount of time this must be submitted for before a decision can be made.
			#[cfg(feature = "fastnet")]
			prepare_period: 2 * MINUTES,
			#[cfg(not(feature = "fastnet"))]
			prepare_period: DAYS,
			// Amount of time that a decision may take to be approved prior to cancellation.
			#[cfg(feature = "fastnet")]
			decision_period: 200 * MINUTES,
			#[cfg(not(feature = "fastnet"))]
			decision_period: 10 * DAYS,
			// Amount of time that the approval criteria must hold before it can be approved.
			#[cfg(feature = "fastnet")]
			confirm_period: 10 * MINUTES,
			#[cfg(not(feature = "fastnet"))]
			confirm_period: DAYS,
			// Minimum amount of time that an approved proposal must be in the dispatch queue.
			#[cfg(feature = "fastnet")]
			min_enactment_period: 5 * MINUTES,
			#[cfg(not(feature = "fastnet"))]
			min_enactment_period: DAYS,
			// Minimum aye votes as percentage of overall conviction-weighted votes needed for
			// approval as a function of time into decision period.
			min_approval: Curve::make_reciprocal(2, 10, percent(80), percent(50), percent(100)),
			// Minimum pre-conviction aye-votes ("support") as percentage of overall population that
			// is needed for approval as a function of time into decision period.
			min_support: Curve::make_linear(10, 10, permill(5), percent(50)),
		},
	),
	(
		1,
		pallet_referenda::TrackInfo {
			name: "whitelisted_caller",
			max_deciding: 25,
			#[cfg(feature = "fastnet")]
			decision_deposit: 5 * ONE_PICA,
			#[cfg(not(feature = "fastnet"))]
			decision_deposit: 500_000 * ONE_PICA,
			#[cfg(feature = "fastnet")]
			prepare_period: 2 * MINUTES,
			#[cfg(not(feature = "fastnet"))]
			prepare_period: 10 * MINUTES,
			#[cfg(feature = "fastnet")]
			decision_period: 100 * MINUTES,
			#[cfg(not(feature = "fastnet"))]
			decision_period: 10 * DAYS,
			#[cfg(feature = "fastnet")]
			confirm_period: 5 * MINUTES,
			#[cfg(not(feature = "fastnet"))]
			confirm_period: 30 * MINUTES,
			#[cfg(feature = "fastnet")]
			min_enactment_period: 2 * MINUTES,
			#[cfg(not(feature = "fastnet"))]
			min_enactment_period: 10 * MINUTES,
			min_approval: Curve::make_reciprocal(2, 10, percent(80), percent(50), percent(100)),
			min_support: Curve::make_reciprocal(1, 10 * 24, percent(1), percent(0), percent(2)),
		},
	),
	(
		2,
		pallet_referenda::TrackInfo {
			name: "general_admin",
			max_deciding: 10,
			#[cfg(not(feature = "fastnet"))]
			decision_deposit: 1_000_000 * ONE_PICA,
			#[cfg(not(feature = "fastnet"))]
			prepare_period: HOURS,
			#[cfg(not(feature = "fastnet"))]
			decision_period: 10 * DAYS,
			#[cfg(not(feature = "fastnet"))]
			confirm_period: DAYS,
			#[cfg(not(feature = "fastnet"))]
			min_enactment_period: DAYS,
			min_approval: Curve::make_reciprocal(2, 10, percent(80), percent(50), percent(100)),
			min_support: Curve::make_reciprocal(5, 10, percent(10), percent(0), percent(50)),
			#[cfg(feature = "fastnet")]
			decision_deposit: 5 * ONE_PICA,
			#[cfg(feature = "fastnet")]
			prepare_period: 2 * MINUTES,
			#[cfg(feature = "fastnet")]
			decision_period: 100 * MINUTES,
			#[cfg(feature = "fastnet")]
			confirm_period: 5 * MINUTES,
			#[cfg(feature = "fastnet")]
			min_enactment_period: 2 * MINUTES,
		},
	),
	(
		3,
		pallet_referenda::TrackInfo {
			name: "referendum_canceller",
			max_deciding: 10,
			#[cfg(not(feature = "fastnet"))]
			decision_deposit: 1_000_000 * ONE_PICA,
			#[cfg(not(feature = "fastnet"))]
			prepare_period: HOURS,
			#[cfg(not(feature = "fastnet"))]
			decision_period: 10 * DAYS,
			#[cfg(not(feature = "fastnet"))]
			confirm_period: 3 * HOURS,
			#[cfg(not(feature = "fastnet"))]
			min_enactment_period: 10 * MINUTES,
			min_approval: Curve::make_reciprocal(2, 10, percent(80), percent(50), percent(100)),
			min_support: Curve::make_reciprocal(1, 10, percent(1), percent(0), percent(10)),
			#[cfg(feature = "fastnet")]
			decision_deposit: 10 * ONE_PICA,
			#[cfg(feature = "fastnet")]
			prepare_period: 2 * MINUTES,
			#[cfg(feature = "fastnet")]
			decision_period: 100 * MINUTES,
			#[cfg(feature = "fastnet")]
			confirm_period: MINUTES,
			#[cfg(feature = "fastnet")]
			min_enactment_period: 2 * MINUTES,
		},
	),
	(
		4,
		pallet_referenda::TrackInfo {
			name: "referendum_killer",
			max_deciding: 25,
			#[cfg(not(feature = "fastnet"))]
			decision_deposit: 1_000_000 * ONE_PICA,
			#[cfg(not(feature = "fastnet"))]
			prepare_period: HOURS,
			#[cfg(not(feature = "fastnet"))]
			decision_period: 10 * DAYS,
			#[cfg(not(feature = "fastnet"))]
			confirm_period: 3 * HOURS,
			#[cfg(not(feature = "fastnet"))]
			min_enactment_period: 10 * MINUTES,
			min_approval: Curve::make_reciprocal(2, 10, percent(80), percent(50), percent(100)),
			min_support: Curve::make_reciprocal(1, 10, percent(1), percent(0), percent(10)),
			#[cfg(feature = "fastnet")]
			decision_deposit: 20 * ONE_PICA,
			#[cfg(feature = "fastnet")]
			prepare_period: 2 * MINUTES,
			#[cfg(feature = "fastnet")]
			decision_period: 100 * MINUTES,
			#[cfg(feature = "fastnet")]
			confirm_period: MINUTES,
			#[cfg(feature = "fastnet")]
			min_enactment_period: 2 * MINUTES,
		},
	),
];

pub struct TracksInfo;
impl pallet_referenda::TracksInfo<Balance, BlockNumber> for TracksInfo {
	type Id = u16;
	type RuntimeOrigin = <RuntimeOrigin as frame_support::traits::OriginTrait>::PalletsOrigin;
	fn tracks() -> &'static [(Self::Id, pallet_referenda::TrackInfo<Balance, BlockNumber>)] {
		&TRACKS_DATA[..]
	}
	fn track_for(id: &Self::RuntimeOrigin) -> Result<Self::Id, ()> {
		if let Ok(system_origin) = frame_system::RawOrigin::try_from(id.clone()) {
			match system_origin {
				frame_system::RawOrigin::Root => {
					if let Some((track_id, _)) =
						Self::tracks().iter().find(|(_, track)| track.name == "root")
					{
						Ok(*track_id)
					} else {
						Err(())
					}
				},
				_ => Err(()),
			}
		} else if let Ok(custom_origin) = pallet_custom_origins::Origin::try_from(id.clone()) {
			if let Some((track_id, _)) = Self::tracks().iter().find(|(_, track)| {
				if let Ok(track_custom_origin) = pallet_custom_origins::Origin::from_str(track.name)
				{
					track_custom_origin == custom_origin
				} else {
					false
				}
			}) {
				Ok(*track_id)
			} else {
				Err(())
			}
		} else {
			Err(())
		}
	}
}
