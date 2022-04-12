// blockchain must be transparent, so most of things are encode/decode
pub use codec::{Decode, Encode, MaxEncodedLen};
pub use scale_info::TypeInfo;

// it is like +-0 built in into lang, but more typed
pub use num_traits::{One, Zero};

// it is like std defaults imports
pub use sp_std::prelude::*;

// cumulus based pallet default
pub use frame_support::pallet_prelude::*;
pub use frame_system::pallet_prelude::*;
