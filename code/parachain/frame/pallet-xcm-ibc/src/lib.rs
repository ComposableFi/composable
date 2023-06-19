pub use pallet::*;
pub use pallet::Error;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
    use frame_support::pallet_prelude::*;

	/// ## Configuration
	/// The pallet's configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

	// The pallet's events
	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		
	}

	#[pallet::error]
	pub enum Error<T> {
		Error1,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}


	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's dispatchable functions.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
    }

    impl<T> MultiCurrencyCallback for Pallet<T> {

    }
}


use xcm::v3::*;
pub trait MultiCurrencyCallback{
	fn deposit_asset(asset: &MultiAsset, location: &MultiLocation, context: &XcmContext, deposit_result : Result){
		//check result, unwrap memo if exists and execute ibc packet
	}
}