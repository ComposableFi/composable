#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::{DispatchResult, DispatchResultWithPostInfo, Vec},
        pallet_prelude::*,
        traits::{
            Currency, EnsureOrigin,
            ExistenceRequirement::{AllowDeath, KeepAlive},
            ReservableCurrency,
        },
    };

    use frame_system::pallet_prelude::*;
    use frame_system::Config as SystemConfig;
    use frame_system::{
        offchain::{
            AppCrypto, CreateSignedTransaction, SendSignedTransaction,
            SignedPayload, Signer, SigningTypes,
        },
    };
    use lite_json::json::JsonValue;
    use sp_core::crypto::KeyTypeId;
    use sp_runtime::{
        offchain::{
            http,
            Duration,
        },
        traits::{Saturating, Zero},
        Percent, PerThing, RuntimeDebug,
    };
    use sp_std::{borrow::ToOwned, str};

    pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"orac");

    pub mod crypto {
        use super::KEY_TYPE;
        use sp_core::sr25519::Signature as Sr25519Signature;
        use sp_runtime::{
            app_crypto::{app_crypto, sr25519},
            traits::Verify,
            MultiSignature, MultiSigner,
        };
        app_crypto!(sr25519, KEY_TYPE);

        pub struct TestAuthId;
        // implemented for ocw-runtime
        impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
            type RuntimeAppPublic = Public;
            type GenericSignature = sp_core::sr25519::Signature;
            type GenericPublic = sp_core::sr25519::Public;
        }

        impl
            frame_system::offchain::AppCrypto<
                <Sr25519Signature as Verify>::Signer,
                Sr25519Signature,
            > for TestAuthId
        {
            type RuntimeAppPublic = Public;
            type GenericSignature = sp_core::sr25519::Signature;
            type GenericPublic = sp_core::sr25519::Public;
        }
    }


    #[pallet::config]
    pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// The currency mechanism.
        type Currency: ReservableCurrency<Self::AccountId>;
        type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
        type MinStake: Get<BalanceOf<Self>>;
        type StakeLock: Get<Self::BlockNumber>;
        type StalePrice: Get<Self::BlockNumber>;
        type AddOracle: EnsureOrigin<Self::Origin>;
        type RequestCost: Get<BalanceOf<Self>>;
        type RewardAmount: Get<BalanceOf<Self>>;
        type SlashAmount: Get<BalanceOf<Self>>;
    }

    #[derive(Encode, Decode, Default, Debug, PartialEq)]
    pub struct Withdraw<Balance, BlockNumber> {
        pub stake: Balance,
        pub unlock_block: BlockNumber,
    }

    #[derive(Encode, Decode, Clone, Copy, Default, Debug, PartialEq)]
    pub struct PrePrice<BlockNumber, AccountId> {
        pub price: u64,
        pub block: BlockNumber,
        pub who: AccountId,
    }

    #[derive(Encode, Decode, Default, Debug, PartialEq)]
    pub struct Price<BlockNumber> {
        pub price: u64,
        pub block: BlockNumber,
    }

    #[derive(Encode, Decode, Default, Debug, PartialEq)]
    pub struct Settlement<AccountId> {
        pub who: AccountId,
        pub truthful: bool,
    }

	#[derive(Encode, Decode, Default, Debug, PartialEq)]
    pub struct AssetInfo<Percent> {
        pub threshold: Percent,
        pub min_answers: u64,
		pub max_answers: u64,
    }


    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as SystemConfig>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage
    #[pallet::storage]
    #[pallet::getter(fn position_count)]
    pub type AssetTypes<T: Config> = StorageValue<_, u128, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn settlements)]
    pub type Settlements<T: Config> = StorageValue<_, Vec<Settlement<T::AccountId>>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn assets_count)]
    pub type AssetsCount<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn signer_to_controller)]
    pub type SignerToController<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, T::AccountId, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn controller_to_signer)]
    pub type ControllerToSigner<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, T::AccountId, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn declared_withdraws)]
    pub type DeclaredWithdraws<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Withdraw<BalanceOf<T>, T::BlockNumber>>;

    #[pallet::storage]
    #[pallet::getter(fn oracle_stake)]
    pub type OracleStake<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn prices)]
    pub type Prices<T: Config> =
        StorageMap<_, Blake2_128Concat, u64, Price<T::BlockNumber>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pre_prices)]
    pub type PrePrices<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,
        Vec<PrePrice<T::BlockNumber, T::AccountId>>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn accuracy_threshold)]
    pub type AssetsInfo<T: Config> = StorageMap<_, Blake2_128Concat, u64, AssetInfo<Percent>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn requested)]
    pub type Requested<T: Config> = StorageMap<_, Blake2_128Concat, u64, bool, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn request_id)]
    pub type RequestedId<T: Config> = StorageMap<_, Blake2_128Concat, u64, u128, ValueQuery>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId",  BalanceOf<T> = "Balance", T::BlockNumber = "BlockNumber", Percent = "Percent")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NewAsset(u128),
		AssetInfoChange(u64, Percent, u64, u64),
		PriceRequested(T::AccountId, u64),
		/// Who added it, the amount added and the total cumulative amount
		StakeAdded(T::AccountId, BalanceOf<T>, BalanceOf<T>),
		StakeRemoved(T::AccountId, BalanceOf<T>, T::BlockNumber),
		StakeReclaimed(T::AccountId, BalanceOf<T>),
		PriceSubmitted(T::AccountId, u64, u64),
		UserSlashed(T::AccountId, u64, BalanceOf<T>),
		UserRewarded(T::AccountId, u64, BalanceOf<T>),
		AnswerPruned(T::AccountId, u64),

    }

    #[pallet::error]
    pub enum Error<T> {
        Unknown,
        NoPermission,
        NoStake,
        StakeLocked,
        NotEnoughStake,
        NotEnoughFunds,
        InvalidAssetId,
        AlreadySubmitted,
        MaxPrices,
        PriceNotRequested,
        UnsetSigner,
        AlreadySet,
        UnsetController,
        ControllerUsed,
		SignerUsed,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(block: T::BlockNumber) -> Weight {
            for i in 0..AssetsCount::<T>::get() {
				let asset_info = AssetsInfo::<T>::get(i);
                // TODO maybe add a check if price is requested, is less operations?
                let pre_pruned_prices = PrePrices::<T>::get(i);
                let mut pre_prices = Vec::new();
                if pre_pruned_prices.len() as u64 >= asset_info.min_answers {
                    pre_prices = Self::prune_old(pre_pruned_prices.clone(), block);
					PrePrices::<T>::insert(i, pre_prices.clone());
                }
                if pre_prices.len() as u64 >= asset_info.min_answers {
                    let price = Self::get_median_price(&pre_prices);
                    let set_price = Price { price, block };
                    Prices::<T>::insert(i, set_price);
                    Requested::<T>::insert(i, false);
					PrePrices::<T>::remove(i);
                    Self::handle_payout(&pre_prices, price, i);
                }
            }
            0
        }

        fn offchain_worker(_block_number: T::BlockNumber) {
            log::info!("Hello World from offchain workers!");
            Self::check_requests();
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(10_000)]
        pub fn add_asset_and_info(
            origin: OriginFor<T>,
            asset_id: u64,
            threshold: Percent,
			min_answers: u64,
			max_answers: u64
        ) -> DispatchResultWithPostInfo {
            T::AddOracle::ensure_origin(origin)?;
			let asset_info = AssetInfo {
				threshold,
				min_answers,
				max_answers
			};
            AssetsInfo::<T>::insert(asset_id, asset_info);
            AssetsCount::<T>::mutate(|a| *a += 1);
			Self::deposit_event(Event::AssetInfoChange(asset_id, threshold, min_answers, max_answers));
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn request_price(origin: OriginFor<T>, asset_id: u64) -> DispatchResultWithPostInfo {
            //TODO talk about the security and if this should be protected
            let who = ensure_signed(origin)?;
            Self::do_request_price(&who, asset_id)?;
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn set_signer(
            origin: OriginFor<T>,
            signer: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let current_controller = ControllerToSigner::<T>::get(&who);
			let current_signer = SignerToController::<T>::get(&signer);

            ensure!(current_controller == None, Error::<T>::ControllerUsed);
			ensure!(current_signer == None, Error::<T>::SignerUsed);

			Self::do_add_stake(who.clone(), signer.clone(), T::MinStake::get())?;

            ControllerToSigner::<T>::insert(&who, signer.clone());
            SignerToController::<T>::insert(signer, who);

            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn add_stake(origin: OriginFor<T>, stake: BalanceOf<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let signer = ControllerToSigner::<T>::get(&who).ok_or(Error::<T>::UnsetSigner)?;

			Self::do_add_stake(who, signer, stake)?;

            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn remove_stake(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let signer = ControllerToSigner::<T>::get(&who).ok_or(Error::<T>::UnsetSigner)?;
            let block = frame_system::Pallet::<T>::block_number();
            let unlock_block = block + T::StakeLock::get(); //TODO check type of add
            let stake = Self::oracle_stake(signer.clone()).ok_or(Error::<T>::NoStake)?;
            let withdrawal = Withdraw {
                stake,
                unlock_block,
            };
            OracleStake::<T>::remove(&signer);
            DeclaredWithdraws::<T>::insert(signer.clone(), withdrawal);
			Self::deposit_event(Event::StakeRemoved(signer, stake, unlock_block));
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn reclaim_stake(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let signer = ControllerToSigner::<T>::get(&who).ok_or(Error::<T>::UnsetSigner)?;
            let block = frame_system::Pallet::<T>::block_number();
            let withdrawal = DeclaredWithdraws::<T>::get(&signer).ok_or(Error::<T>::Unknown)?;
            ensure!(block > withdrawal.unlock_block, Error::<T>::StakeLocked);
            DeclaredWithdraws::<T>::remove(&signer);
            T::Currency::unreserve(&signer, withdrawal.stake.into());
            let _ = T::Currency::transfer(&signer, &who, withdrawal.stake.into(), AllowDeath);

			ControllerToSigner::<T>::remove(&who);
            SignerToController::<T>::remove(&signer);

			Self::deposit_event(Event::StakeReclaimed(signer, withdrawal.stake));
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn submit_price(
            origin: OriginFor<T>,
            price: u64,
            asset_id: u64,
        ) -> DispatchResultWithPostInfo {
            log::info!("inside submit {:#?}, {:#?}", asset_id, price);
            let who = ensure_signed(origin)?;
            let author_stake = OracleStake::<T>::get(&who).unwrap_or_else(Zero::zero);
            ensure!(Requested::<T>::get(asset_id), Error::<T>::PriceNotRequested);
            ensure!(
                author_stake >= T::MinStake::get(),
                Error::<T>::NotEnoughStake
            );

            let set_price = PrePrice {
                price,
                block: frame_system::Pallet::<T>::block_number(),
                who: who.clone(),
            };
            log::info!("inside submit 2 {:#?}, {:#?}", set_price, asset_id);
            PrePrices::<T>::try_mutate(asset_id, |current_prices| -> DispatchResult {
                if current_prices
                    .into_iter()
                    .any(|candidate| candidate.who == who)
                {
                    Err(Error::<T>::AlreadySubmitted)?
                }
                if current_prices.len() as u64 >= AssetsInfo::<T>::get(asset_id).max_answers {
                    Err(Error::<T>::MaxPrices)?
                }
                current_prices.push(set_price);
                Ok(())
            })?;
			Self::deposit_event(Event::PriceSubmitted(who, asset_id, price));
            Ok(().into())
        }
    }

    /// Payload used by this example crate to hold price
    /// data required to submit a transaction.
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
    pub struct PricePayload<Public, BlockNumber> {
        block_number: BlockNumber,
        price: u32,
        public: Public,
    }

    impl<T: SigningTypes> SignedPayload<T> for PricePayload<T::Public, T::BlockNumber> {
        fn public(&self) -> T::Public {
            self.public.clone()
        }
    }

    impl<T: Config> Pallet<T> {

        pub fn do_add_stake(who: T::AccountId, signer: T::AccountId, stake: BalanceOf<T>) ->  DispatchResult {
			T::Currency::transfer(&who, &signer, stake, KeepAlive)?;
            T::Currency::reserve(&signer, stake.into())?;
            let amount_staked = Self::oracle_stake(signer.clone()).unwrap_or(0u32.into()) + stake; // TODO maybe use checked add?
            OracleStake::<T>::insert(&signer, amount_staked);
			Self::deposit_event(Event::StakeAdded(signer, stake, amount_staked));
			Ok(())
		}


        pub fn handle_payout(pre_prices: &Vec<PrePrice<T::BlockNumber, T::AccountId>>, price: u64, asset_id: u64) {
			// TODO only take prices up to max prices
            for answer in pre_prices {
                let accuracy: Percent;
                if answer.price < price {
                    accuracy = PerThing::from_rational(answer.price, price);
                } else {
                    let adjusted_number = price.saturating_sub(answer.price - price);
                    accuracy = PerThing::from_rational(adjusted_number, price);
                }
				let min_accuracy = AssetsInfo::<T>::get(asset_id).threshold;
                if accuracy < min_accuracy {
					let slash_amount = T::SlashAmount::get();
                    let try_slash = T::Currency::can_slash(&answer.who, slash_amount);
                    if !try_slash {
                        log::warn!("Failed to slash {:?}", answer.who);
                    }
                    T::Currency::slash(&answer.who, slash_amount);
					Self::deposit_event(Event::UserSlashed(answer.who.clone(), asset_id, slash_amount));
                } else {
					let reward_amount = T::RewardAmount::get();
                    let controller =
                        SignerToController::<T>::get(&answer.who).unwrap_or(answer.who.clone());
                    // TODO: since inlflationary, burn a portion of tx fees of the chain to account for this
                    let _ = T::Currency::deposit_into_existing(&controller, reward_amount);
					Self::deposit_event(Event::UserRewarded(answer.who.clone(), asset_id, reward_amount));
                }
            }
        }
        pub fn do_request_price(who: &T::AccountId, asset_id: u64) -> DispatchResult {
			ensure!(AssetsInfo::<T>::get(asset_id).threshold != Percent::from_percent(0), Error::<T>::InvalidAssetId);
            if !Self::requested(asset_id) {
                ensure!(
                    T::Currency::can_slash(who, T::RequestCost::get()),
                    Error::<T>::NotEnoughFunds
                );
                T::Currency::slash(who, T::RequestCost::get());
                RequestedId::<T>::mutate(asset_id, |request_id| *request_id += 1);
                Requested::<T>::insert(asset_id, true);
            }
			Self::deposit_event(Event::PriceRequested(who.clone(), asset_id));
            Ok(())
        }

        pub fn prune_old(
            mut pre_pruned_prices: Vec<PrePrice<T::BlockNumber, T::AccountId>>,
			block: T::BlockNumber
        ) -> Vec<PrePrice<T::BlockNumber, T::AccountId>> {
			let stale_block = block.saturating_sub(T::StalePrice::get());
            if pre_pruned_prices.len() == 0 || pre_pruned_prices[0].block >= stale_block {
                pre_pruned_prices
            } else {
                let pruned_price = loop {
                    if pre_pruned_prices.len() == 0 {
                        break pre_pruned_prices;
                    }
                    if pre_pruned_prices[0].block >= stale_block {
                        break pre_pruned_prices;
                    } else {
						Self::deposit_event(Event::AnswerPruned(pre_pruned_prices[0].who.clone(), pre_pruned_prices[0].price));
                        pre_pruned_prices.remove(0);
                    }
                };
                pruned_price
            }
        }

        pub fn get_median_price(prices: &Vec<PrePrice<T::BlockNumber, T::AccountId>>) -> u64 {
            let mut numbers: Vec<u64> = prices
                .iter()
                .map(|current_prices| current_prices.price)
                .collect();
            numbers.sort();
            let mid = numbers.len() / 2;
            numbers[mid]
        }

        pub fn check_requests() {
            for i in 0..AssetsCount::<T>::get() {
                if Requested::<T>::get(i) {
                    let _ = Self::fetch_price_and_send_signed(&i);
                }
            }
        }

        pub fn fetch_price_and_send_signed(price_id: &u64) -> Result<(), &'static str> {
            let signer = Signer::<T, T::AuthorityId>::all_accounts();
            log::info!("signer");
            if !signer.can_sign() {
                log::info!("no signer");
                return Err(
                    "No local accounts available. Consider adding one via `author_insertKey` RPC.",
                )?;
            }
            // Make an external HTTP request to fetch the current price.
            // Note this call will block until response is received.
            let price = Self::fetch_price(&price_id).map_err(|_| "Failed to fetch price")?;
            log::info!("price {:#?}", price);

            // Using `send_signed_transaction` associated type we create and submit a transaction
            // representing the call, we've just created.
            // Submit signed will return a vector of results for all accounts that were found in the
            // local keystore with expected `KEY_TYPE`.
            let results = signer.send_signed_transaction(|_account| {
                // Received price is wrapped into a call to `submit_price` public function of this pallet.
                // This means that the transaction, when executed, will simply call that function passing
                // `price` as an argument.
                Call::submit_price(price, *price_id)
            });

            for (acc, res) in &results {
                match res {
                    Ok(()) => log::info!("[{:?}] Submitted price of {} cents", acc.id, price),
                    Err(e) => log::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e),
                }
            }

            Ok(())
        }

        pub fn fetch_price(price_id: &u64) -> Result<u64, http::Error> {
            // We want to keep the offchain worker execution time reasonable, so we set a hard-coded
            // deadline to 2s to complete the external call.
            // You can also wait idefinitely for the response, however you may still get a timeout
            // coming from the host machine.
            let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
            // Initiate an external HTTP GET request.
            // This is using high-level wrappers from `sp_runtime`, for the low-level calls that
            // you can find in `sp_io`. The API is trying to be similar to `reqwest`, but
            // since we are running in a custom WASM execution environment we can't simply
            // import the library here.]

            let kind = sp_core::offchain::StorageKind::PERSISTENT;
            let from_local = sp_io::offchain::local_storage_get(kind, b"ocw-url")
                .unwrap_or(b"http://localhost:3001/price/".to_vec());
            let base = str::from_utf8(&from_local).unwrap_or("http://localhost:3001/price/");
            let string_id = serde_json::to_string(&price_id).map_err(|_| http::Error::IoError)?;
            let request_id = RequestedId::<T>::get(&price_id);
            let string_request_id =
                serde_json::to_string(&request_id).map_err(|_| http::Error::IoError)?;
            let url = base.to_owned() + &string_id + "/" + &string_request_id;
            let request = http::Request::get(&url);

            log::info!("request incoming {:#?}", request);

            // We set the deadline for sending of the request, note that awaiting response can
            // have a separate deadline. Next we send the request, before that it's also possible
            // to alter request headers or stream body content in case of non-GET requests.
            let pending = request
                .deadline(deadline)
                .send()
                .map_err(|_| http::Error::IoError)?;

            // The request is already being processed by the host, we are free to do anything
            // else in the worker (we can send multiple concurrent requests too).
            // At some point however we probably want to check the response though,
            // so we can block current thread and wait for it to finish.
            // Note that since the request is being driven by the host, we don't have to wait
            // for the request to have it complete, we will just not read the response.
            let response = pending
                .try_wait(deadline)
                .map_err(|_| http::Error::DeadlineReached)??;
            // Let's check the status code before we proceed to reading the response.
            if response.code != 200 {
                log::warn!("Unexpected status code: {}", response.code);
                return Err(http::Error::Unknown);
            }

            // Next we want to fully read the response body and collect it to a vector of bytes.
            // Note that the return object allows you to read the body in chunks as well
            // with a way to control the deadline.
            let body = response.body().collect::<Vec<u8>>();

            // Create a str slice from the body.
            let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
                log::warn!("No UTF8 body");
                http::Error::Unknown
            })?;

            let price = match Self::parse_price(body_str) {
                Some(price) => Ok(price),
                None => {
                    log::warn!("Unable to extract price from the response: {:?}", body_str);
                    Err(http::Error::Unknown)
                }
            }?;

            log::warn!("Got price: {} cents", price);

            Ok(price)
        }

        pub fn parse_price(price_str: &str) -> Option<u64> {
            let val = lite_json::parse_json(price_str);
            let price = match val.ok()? {
                JsonValue::Object(obj) => {
                    let (_, v) = obj
                        .into_iter()
                        .find(|(k, _)| k.iter().copied().eq("USD".chars()))?;
                    match v {
                        JsonValue::Number(number) => number,
                        _ => return None,
                    }
                }
                _ => return None,
            };

            let exp = price.fraction_length.checked_sub(2).unwrap_or(0);
            Some(price.integer as u64 * 100 + (price.fraction / 10_u64.pow(exp)) as u64)
        }
    }
}
