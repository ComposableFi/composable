macro_rules! interest {
    ($chain:ident) => {
        pub mod $chain {
            use subxt::utils::AccountId32;
            use $chain::parachain::{api, api::runtime_types};
            #[derive(Debug, parity_scale_codec::Encode, parity_scale_codec::Decode)]
            pub enum ChangeOfInterest {
                Balances(runtime_types::pallet_balances::pallet::Event),
                StorageSystemAccount(
                    Vec<(
                        AccountId32,
                        runtime_types::frame_system::AccountInfo<
                            u32,
                            runtime_types::pallet_balances::AccountData<u128>,
                        >,
                    )>,
                ),
                Tokens(runtime_types::orml_tokens::module::Event),
                StorageTokensAccounts(
                    Vec<(
                        AccountId32,
                        u128,
                        runtime_types::orml_tokens::AccountData<u128>,
                    )>,
                ),
                StorageTokensTotalIssuance(Vec<(u128, u128)>),
                Ibc(runtime_types::pallet_ibc::pallet::Event),
                Ics20Fee(api::ics20_fee::Event),
                ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Event),
                DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Event),
                XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Event),
                CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Event),
                Treasury(runtime_types::pallet_treasury::pallet::Event),
                PolkadotXcm(runtime_types::pallet_xcm::pallet::Event),
                Utility(runtime_types::pallet_utility::pallet::Event),
                XTokens(runtime_types::orml_xtokens::module::Event),
            }
        }
    };
}

interest!(composable);
interest!(picasso);
