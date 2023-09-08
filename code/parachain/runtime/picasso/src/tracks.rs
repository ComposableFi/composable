use super::*;

const fn percent(x: i32) -> sp_runtime::FixedI64 {
	sp_runtime::FixedI64::from_rational(x as u128, 100)
}
const fn permill(x: i32) -> sp_runtime::FixedI64 {
	sp_runtime::FixedI64::from_rational(x as u128, 1000)
}

use pallet_referenda::Curve;
const TRACKS_DATA: [(u16, pallet_referenda::TrackInfo<Balance, BlockNumber>); 1] = [
	(
		0,
		pallet_referenda::TrackInfo {
			// Name of this track.
			name: "root",
			// A limit for the number of referenda on this track that can be being decided at once.
			// For Root origin this should generally be just one.
			max_deciding: 5,
			// Amount that must be placed on deposit before a decision can be made.
			decision_deposit: 1,
			// Amount of time this must be submitted for before a decision can be made.
			prepare_period: 1 * DAYS,
			// Amount of time that a decision may take to be approved prior to cancellation.
			decision_period: 14 * DAYS,
			// Amount of time that the approval criteria must hold before it can be approved.
			confirm_period: 1 * DAYS,
			// Minimum amount of time that an approved proposal must be in the dispatch queue.
			min_enactment_period: 1 * DAYS,
			// Minimum aye votes as percentage of overall conviction-weighted votes needed for
			// approval as a function of time into decision period.
			min_approval: Curve::make_reciprocal(4, 14, percent(80), percent(50), percent(100)),
			// Minimum pre-conviction aye-votes ("support") as percentage of overall population that
			// is needed for approval as a function of time into decision period.
			min_support: Curve::make_linear(14, 14, permill(5), percent(25)),
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
		// } else if let Ok(custom_origin) = origins::Origin::try_from(id.clone()) {
		// 	match custom_origin {
		// 		origins::Origin::WhitelistedCaller => Ok(1),
		// 		// General admin
		// 		origins::Origin::StakingAdmin => Ok(10),
		// 		origins::Origin::Treasurer => Ok(11),
		// 		origins::Origin::LeaseAdmin => Ok(12),
		// 		origins::Origin::FellowshipAdmin => Ok(13),
		// 		origins::Origin::GeneralAdmin => Ok(14),
		// 		origins::Origin::AuctionAdmin => Ok(15),
		// 		// Referendum admins
		// 		origins::Origin::ReferendumCanceller => Ok(20),
		// 		origins::Origin::ReferendumKiller => Ok(21),
		// 		// Limited treasury spenders
		// 		origins::Origin::SmallTipper => Ok(30),
		// 		origins::Origin::BigTipper => Ok(31),
		// 		origins::Origin::SmallSpender => Ok(32),
		// 		origins::Origin::MediumSpender => Ok(33),
		// 		origins::Origin::BigSpender => Ok(34),
		// 		_ => Err(()),
		// 	}
		} else {
			Err(())
		}
	}
}

#[test]
/// To ensure voters are always locked into their vote
fn vote_locking_always_longer_than_enactment_period() {
	for (_, track) in TRACKS_DATA {
		assert!(
			<Runtime as pallet_conviction_voting::Config>::VoteLockingPeriod::get()
				>= track.min_enactment_period,
			"Track {} has enactment period {} < vote locking period {}",
			track.name,
			track.min_enactment_period,
			<Runtime as pallet_conviction_voting::Config>::VoteLockingPeriod::get(),
		);
	}
}

#[test]
fn all_tracks_have_origins() {
	for (_, track) in TRACKS_DATA {
		// check name.into() is successful either converts into "root" or custom origin
		let track_is_root = track.name == "root";
		let track_has_custom_origin = pallet_custom_origins::Origin::from_str(track.name).is_ok();
		assert!(track_is_root || track_has_custom_origin);
	}
}