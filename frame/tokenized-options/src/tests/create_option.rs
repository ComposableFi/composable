use crate::{
	mocks::{
		accounts::*,
		assets::*,
		runtime::{Event, ExtBuilder, MockRuntime, Origin, System, TokenizedOptions},
	},
	pallet,
	tests::*,
	Error, OptionHashToOptionId, OptionIdToOption,
};
use frame_support::{assert_noop, assert_ok, error::BadOrigin};

// ----------------------------------------------------------------------------------------------------
//		Create Options Tests
// ----------------------------------------------------------------------------------------------------
/// Create BTC vault, create BTC option and check if option_id is correctly saved and event emitted
#[test]
fn test_create_option_success() {
	ExtBuilder::default().build().initialize_oracle_prices().execute_with(|| {
		// Get BTC and USDC vault config
		let btc_vault_config = VaultConfigBuilder::default().build();
		let usdc_vault_config = VaultConfigBuilder::default().asset_id(USDC).build();

		// Create BTC and USDC vaults
		assert_ok!(TokenizedOptions::create_asset_vault(Origin::signed(ADMIN), btc_vault_config));

		assert_ok!(TokenizedOptions::create_asset_vault(Origin::signed(ADMIN), usdc_vault_config));

		// Get BTC option config
		let option_config = OptionsConfigBuilder::default().build();

		let option_hash = TokenizedOptions::generate_id(
			option_config.base_asset_id,
			option_config.quote_asset_id,
			option_config.base_asset_strike_price,
			option_config.quote_asset_strike_price,
			option_config.option_type,
			option_config.expiring_date,
			option_config.exercise_type,
		);

		// Create option and get option id
		let option_id = trait_create_option(Origin::signed(ADMIN), option_config.clone())
			.expect("Error creating option");

		// Check option has been created
		assert!(OptionHashToOptionId::<MockRuntime>::contains_key(option_hash));
		assert!(OptionIdToOption::<MockRuntime>::contains_key(option_id));
		let option_id_from_hash = OptionHashToOptionId::<MockRuntime>::get(option_hash).unwrap();
		assert_eq!(option_id, option_id_from_hash);

		// Check event is emitted correctly
		System::assert_last_event(Event::TokenizedOptions(pallet::Event::CreatedOption {
			option_id,
			option_config,
		}));
	});
}

/// Create BTC vault, create BTC option and check if vault_id is correctly saved and event emitted
/// using extrinsic
#[test]
fn test_create_option_success_ext() {
	ExtBuilder::default().build().initialize_oracle_prices().execute_with(|| {
		// Get BTC and USDC vault config
		let btc_vault_config = VaultConfigBuilder::default().build();
		let usdc_vault_config = VaultConfigBuilder::default().asset_id(USDC).build();

		// Create BTC and USDC vaults
		assert_ok!(TokenizedOptions::create_asset_vault(Origin::signed(ADMIN), btc_vault_config));

		assert_ok!(TokenizedOptions::create_asset_vault(Origin::signed(ADMIN), usdc_vault_config));

		// Get BTC option config
		let option_config = OptionsConfigBuilder::default().build();

		// Create option and get option id
		assert_ok!(TokenizedOptions::create_option(Origin::signed(ADMIN), option_config.clone()));

		// Check option has been created (ID = 3 because first two IDs are used for the vaults
		// lp_tokens)
		assert!(OptionIdToOption::<MockRuntime>::contains_key(AssetId(100000000003u128)));

		// Check event is emitted correctly
		System::assert_last_event(Event::TokenizedOptions(pallet::Event::CreatedOption {
			option_id: AssetId(100000000003u128),
			option_config,
		}));
	});
}

#[test]
fn test_create_option_error_vaults_not_exist_ext() {
	ExtBuilder::default().build().initialize_oracle_prices().execute_with(|| {
		// Get default option config
		let option_config = OptionsConfigBuilder::default().build();

		// Create same option again and check error is raised
		assert_noop!(
			TokenizedOptions::create_option(Origin::signed(ADMIN), option_config),
			Error::<MockRuntime>::OptionAssetVaultsDoNotExist
		);

		// Check option has not been created
		assert!(!OptionIdToOption::<MockRuntime>::contains_key(AssetId(100000000001u128)));
	});
}

#[test]
fn test_create_option_error_invalid_epoch_ext() {
	ExtBuilder::default()
		.build()
		.initialize_oracle_prices()
		.initialize_all_vaults()
		.execute_with(|| {
			let epoch = Epoch { deposit: 1u64, purchase: 2u64, exercise: 3u64, end: 3u64 };

			// Get default option config
			let option_config = OptionsConfigBuilder::default().epoch(epoch).build();

			// Create same option again and check error is raised
			assert_noop!(
				TokenizedOptions::create_option(Origin::signed(ADMIN), option_config),
				Error::<MockRuntime>::OptionAttributesAreInvalid
			);

			// Check option has not been created
			assert!(!OptionIdToOption::<MockRuntime>::contains_key(AssetId(100000000001u128)));
		});
}

#[test]
fn test_create_option_error_base_quote_equal_ext() {
	ExtBuilder::default()
		.build()
		.initialize_oracle_prices()
		.initialize_all_vaults()
		.execute_with(|| {
			// Get default option config
			let option_config = OptionsConfigBuilder::default().quote_asset_id(BTC).build();

			// Create option with same base and quote asset and check error is raised
			assert_noop!(
				TokenizedOptions::create_option(Origin::signed(ADMIN), option_config),
				Error::<MockRuntime>::OptionAttributesAreInvalid
			);

			// Check option has not been created
			assert!(!OptionIdToOption::<MockRuntime>::contains_key(AssetId(100000000001u128)));
		});
}

#[test]
fn test_create_option_error_initial_issuance_not_zero_ext() {
	ExtBuilder::default()
		.build()
		.initialize_oracle_prices()
		.initialize_all_vaults()
		.execute_with(|| {
			// Get default option config
			let option_config =
				OptionsConfigBuilder::default().total_issuance_seller(1u128).build();

			// Create option with initial issuance seller not zero and check error is raised
			assert_noop!(
				TokenizedOptions::create_option(Origin::signed(ADMIN), option_config),
				Error::<MockRuntime>::OptionAttributesAreInvalid
			);
		});
}

/// Create BTC vault, create BTC option twice and check if error is correctly raised and storage not
/// changed
#[test]
fn test_create_option_error_option_already_exists() {
	ExtBuilder::default().build().initialize_oracle_prices().execute_with(|| {
		// Get BTC and USDC vault config
		let btc_vault_config = VaultConfigBuilder::default().build();
		let usdc_vault_config = VaultConfigBuilder::default().asset_id(USDC).build();

		// Create BTC and USDC vaults
		assert_ok!(TokenizedOptions::create_asset_vault(Origin::signed(ADMIN), btc_vault_config));

		assert_ok!(TokenizedOptions::create_asset_vault(Origin::signed(ADMIN), usdc_vault_config));

		// Get default option config
		let option_config = OptionsConfigBuilder::default().build();

		let option_id = trait_create_option(Origin::signed(ADMIN), option_config.clone())
			.expect("Error creating option");

		// Check option has been created
		assert!(OptionIdToOption::<MockRuntime>::contains_key(option_id));

		// Check event is emitted correctly
		System::assert_last_event(Event::TokenizedOptions(pallet::Event::CreatedOption {
			option_id,
			option_config: option_config.clone(),
		}));

		// Create same option again and check error is raised
		assert_noop!(
			TokenizedOptions::create_option(Origin::signed(ADMIN), option_config),
			Error::<MockRuntime>::OptionAlreadyExists
		);
	});
}

/// Create BTC vault, create BTC option twice and check if error is correctly raised and storage not
/// changed using extrinsic
#[test]
fn test_create_option_error_option_already_exists_ext() {
	ExtBuilder::default().build().initialize_oracle_prices().execute_with(|| {
		// Get BTC and USDC vault config
		let btc_vault_config = VaultConfigBuilder::default().build();
		let usdc_vault_config = VaultConfigBuilder::default().asset_id(USDC).build();

		// Create BTC and USDC vaults
		assert_ok!(TokenizedOptions::create_asset_vault(Origin::signed(ADMIN), btc_vault_config));

		assert_ok!(TokenizedOptions::create_asset_vault(Origin::signed(ADMIN), usdc_vault_config));

		// Get default option config
		let option_config = OptionsConfigBuilder::default().build();

		assert_ok!(TokenizedOptions::create_option(Origin::signed(ADMIN), option_config.clone()));

		// Check option has been created
		assert!(OptionIdToOption::<MockRuntime>::contains_key(AssetId(100000000003u128)));

		// Check event is emitted correctly
		System::assert_last_event(Event::TokenizedOptions(pallet::Event::CreatedOption {
			option_id: AssetId(100000000003u128),
			option_config: option_config.clone(),
		}));

		// Create same option again and check error is raised
		assert_noop!(
			TokenizedOptions::create_option(Origin::signed(ADMIN), option_config),
			Error::<MockRuntime>::OptionAlreadyExists
		);
	});
}

/// Create BTC vault, create BTC option and check if vault_id is correctly saved and event emitted
/// using extrinsic
#[test]
fn test_create_option_error_not_protocol_origin_ext() {
	ExtBuilder::default().build().initialize_oracle_prices().execute_with(|| {
		// Get BTC and USDC vault config
		let btc_vault_config = VaultConfigBuilder::default().build();
		let usdc_vault_config = VaultConfigBuilder::default().asset_id(USDC).build();

		// Create BTC and USDC vaults
		assert_ok!(TokenizedOptions::create_asset_vault(Origin::signed(ADMIN), btc_vault_config));

		assert_ok!(TokenizedOptions::create_asset_vault(Origin::signed(ADMIN), usdc_vault_config));

		// Get BTC option config
		let option_config = OptionsConfigBuilder::default().build();

		// Check no changes has been performed with ALICE caller
		assert_noop!(
			TokenizedOptions::create_option(Origin::signed(ALICE), option_config.clone()),
			BadOrigin
		);

		// Check root can create option
		assert_ok!(TokenizedOptions::create_option(Origin::root(), option_config.clone()));

		// Check option has been created (ID = 3 because first two IDs are used for the vaults
		// lp_tokens)
		assert!(OptionIdToOption::<MockRuntime>::contains_key(AssetId(100000000003u128)));

		// Check event is emitted correctly
		System::assert_last_event(Event::TokenizedOptions(pallet::Event::CreatedOption {
			option_id: AssetId(100000000003u128),
			option_config,
		}));
	});
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(20))]
	#[test]
	fn proptest_create_option(
		option_configs in random_option_configs(5..10, any::<u128>(), 0..1000, 10..100)
	) {
		// Create all the asset vaults before creating options
		ExtBuilder::default().build().initialize_oracle_prices().initialize_all_vaults().execute_with(|| {
			option_configs.into_iter().for_each(|option_config|{
				match trait_create_option(Origin::signed(ADMIN), option_config.clone()) {
					Ok(option_id) => {
						assert!(OptionIdToOption::<MockRuntime>::contains_key(option_id));

						System::assert_last_event(Event::TokenizedOptions(pallet::Event::CreatedOption {
							option_id,
							option_config,
						}));
					},
					Err(error) => {
						assert_eq!(error, DispatchError::from(Error::<MockRuntime>::OptionAssetVaultsDoNotExist));
					}
				};
			})
		});
	}
}
