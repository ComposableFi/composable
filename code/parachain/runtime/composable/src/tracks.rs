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

use common::MINUTES;
use pallet_referenda::Curve;
const TRACKS_DATA: [(u16, pallet_referenda::TrackInfo<Balance, BlockNumber>); 2] = [
	(
		0,
		pallet_referenda::TrackInfo {
			// Name of this track.
			name: "root",
			// A limit for the number of referenda on this track that can be being decided at once.
			// For Root origin this should generally be just one.
			max_deciding: 1,
			// Amount that must be placed on deposit before a decision can be made.
			decision_deposit: 0,
			// Amount of time this must be submitted for before a decision can be made.
			#[cfg(feature = "fastnet")]
			prepare_period: 2 * MINUTES,
			#[cfg(not(feature = "fastnet"))]
			prepare_period: 2 * HOURS,
			// Amount of time that a decision may take to be approved prior to cancellation.
			#[cfg(feature = "fastnet")]
			decision_period: 200 * MINUTES,
			#[cfg(not(feature = "fastnet"))]
			decision_period: 7 * DAYS,
			// Amount of time that the approval criteria must hold before it can be approved.
			#[cfg(feature = "fastnet")]
			confirm_period: 15 * MINUTES,
			#[cfg(not(feature = "fastnet"))]
			confirm_period: 1 * DAYS,
			// Minimum amount of time that an approved proposal must be in the dispatch queue.
			#[cfg(feature = "fastnet")]
			min_enactment_period: 5 * MINUTES,
			#[cfg(not(feature = "fastnet"))]
			min_enactment_period: 1 * DAYS,
			// Minimum aye votes as percentage of overall conviction-weighted votes needed for
			// approval as a function of time into decision period.
			#[cfg(feature = "fastnet")]
			min_approval: Curve::make_reciprocal(4, 30, percent(80), percent(50), percent(100)),
			#[cfg(not(feature = "fastnet"))]
			min_approval: Curve::make_reciprocal(4, 28, percent(80), percent(50), percent(100)),
			// Minimum pre-conviction aye-votes ("support") as percentage of overall population that
			// is needed for approval as a function of time into decision period.
			#[cfg(feature = "fastnet")]
			min_support: Curve::make_linear(30, 30, percent(0), percent(50)),
			#[cfg(not(feature = "fastnet"))]
			min_support: Curve::make_linear(28, 28, percent(0), percent(50)),
		},
	),
	(
		1,
		pallet_referenda::TrackInfo {
			name: "whitelisted_caller",
			max_deciding: 2,
			decision_deposit: 0,
			#[cfg(feature = "fastnet")]
			prepare_period: 2 * MINUTES,
			#[cfg(not(feature = "fastnet"))]
			prepare_period: 30 * MINUTES,
			#[cfg(feature = "fastnet")]
			decision_period: 100 * MINUTES,
			#[cfg(not(feature = "fastnet"))]
			decision_period: 4 * DAYS,
			#[cfg(feature = "fastnet")]
			confirm_period: 5 * MINUTES,
			#[cfg(not(feature = "fastnet"))]
			confirm_period: 10 * MINUTES,
			#[cfg(feature = "fastnet")]
			min_enactment_period: 2 * MINUTES,
			#[cfg(not(feature = "fastnet"))]
			min_enactment_period: 10 * MINUTES,
			#[cfg(feature = "fastnet")]
			min_approval: Curve::make_reciprocal(1, 30, percent(96), percent(50), percent(100)),
			#[cfg(not(feature = "fastnet"))]
			min_approval: Curve::make_reciprocal(
				16,
				28 * 24,
				percent(96),
				percent(50),
				percent(100),
			),
			#[cfg(feature = "fastnet")]
			min_support: Curve::make_reciprocal(1, 30, percent(20), percent(5), percent(50)),
			#[cfg(not(feature = "fastnet"))]
			min_support: Curve::make_reciprocal(1, 28, percent(20), percent(5), percent(50)),
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
				frame_system::RawOrigin::Root => Ok(0),
				_ => Err(()),
			}
		} else if let Ok(custom_origin) = pallet_custom_origins::Origin::try_from(id.clone()) {
			match custom_origin {
				pallet_custom_origins::Origin::WhitelistedCaller => Ok(1),
			}
		} else {
			Err(())
		}
	}
}
