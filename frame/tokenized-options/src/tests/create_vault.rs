use crate::{
	mocks::{
		accounts::*,
		assets::*,
		runtime::{Event, ExtBuilder, MockRuntime, Origin, System, TokenizedOptions},
	},
	pallet::{self, AssetToVault, Error},
	tests::*,
};

use frame_support::{assert_noop, error::BadOrigin};

// ----------------------------------------------------------------------------------------------------
//		Create Vault Tests
// ----------------------------------------------------------------------------------------------------

/// Create BTC vault; check that vault_id is correct and event emitted
#[test]
fn test_create_vault_success() {
	ExtBuilder::default().build().initialize_oracle_prices().execute_with(|| {
		// Get default vault config
		let vault_config = VaultConfigBuilder::default().build();

		// Check that the vault has not already been created
		assert!(!AssetToVault::<MockRuntime>::contains_key(vault_config.asset_id));

		// Create vault
		let vault_id = trait_create_asset_vault(Origin::signed(ADMIN), vault_config.clone())
			.expect("Error creating vault");

		// Check vault has been created
		assert!(AssetToVault::<MockRuntime>::contains_key(vault_config.asset_id));

		// Check vault_id correctly saved
		assert_eq!(vault_id, AssetToVault::<MockRuntime>::get(vault_config.asset_id).unwrap());

		// Check event is correctly emitted
		System::assert_last_event(Event::TokenizedOptions(pallet::Event::CreatedAssetVault {
			vault_id,
			asset_id: vault_config.asset_id,
		}));
	});
}

/// Create BTC vault using extrinsic; check if vault_id is correctly saved and event emitted
#[test]
fn test_create_vault_success_ext() {
	ExtBuilder::default().build().initialize_oracle_prices().execute_with(|| {
		let vault_config = VaultConfigBuilder::default().build();

		assert!(!AssetToVault::<MockRuntime>::contains_key(vault_config.asset_id));

		assert_ok!(TokenizedOptions::create_asset_vault(
			Origin::signed(ADMIN),
			vault_config.clone()
		));

		assert!(AssetToVault::<MockRuntime>::contains_key(vault_config.asset_id));

		System::assert_last_event(Event::TokenizedOptions(pallet::Event::CreatedAssetVault {
			vault_id: 1u64,
			asset_id: vault_config.asset_id,
		}));
	});
}

/// Create BTC vault using extrinsic; try to create it again; check if error is raised and storage
/// not changed
#[test]
fn test_create_vault_error_vault_already_exists_ext() {
	ExtBuilder::default().build().initialize_oracle_prices().execute_with(|| {
		let vault_config = VaultConfigBuilder::default().build();

		assert!(!AssetToVault::<MockRuntime>::contains_key(vault_config.asset_id));

		assert_ok!(TokenizedOptions::create_asset_vault(
			Origin::signed(ADMIN),
			vault_config.clone()
		));

		assert!(AssetToVault::<MockRuntime>::contains_key(vault_config.asset_id));

		System::assert_last_event(Event::TokenizedOptions(pallet::Event::CreatedAssetVault {
			vault_id: 1u64,
			asset_id: vault_config.asset_id,
		}));

		assert_noop!(
			TokenizedOptions::create_asset_vault(Origin::signed(ADMIN), vault_config),
			Error::<MockRuntime>::AssetVaultAlreadyExists
		);
	});
}

/// Create ETH vault (not supported by oracle); check that correct error is raised
#[test]
fn test_create_vault_error_asset_not_supported_ext() {
	ExtBuilder::default().build().initialize_oracle_prices().execute_with(|| {
		// Get ETH vault config (not supported by oracle)
		let vault_config = VaultConfigBuilder::default().asset_id(ETH).build();

		// Check that the vault has not already been created
		assert!(!AssetToVault::<MockRuntime>::contains_key(vault_config.asset_id));

		// Check no changes have been performed
		assert_noop!(
			TokenizedOptions::create_asset_vault(Origin::signed(ADMIN), vault_config),
			Error::<MockRuntime>::AssetIsNotSupported
		);
	});
}

/// Create vault with no root account; check that correct error is raised
#[test]
fn test_create_vault_error_not_protocol_origin_ext() {
	ExtBuilder::default().build().initialize_oracle_prices().execute_with(|| {
		// Get ETH vault config (not supported by oracle)
		let vault_config = VaultConfigBuilder::default().build();

		// Check that the vault has not already been created
		assert!(!AssetToVault::<MockRuntime>::contains_key(vault_config.asset_id));

		// Check no changes have been performed with ALICE caller
		assert_noop!(
			TokenizedOptions::create_asset_vault(Origin::signed(ALICE), vault_config.clone()),
			BadOrigin
		);

		// Check root can create vault
		assert_ok!(TokenizedOptions::create_asset_vault(Origin::root(), vault_config));
	});
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(20))]

	/// Create random vaults and check if error is raised correctly
	#[test]
	fn proptest_create_vault_ext(assets in prop::collection::vec(random_asset(), 10)) {
		ExtBuilder::default().build().initialize_oracle_prices().execute_with(|| {
			assets.iter().for_each(|&asset| {
				let vault_config = VaultConfigBuilder::default().asset_id(asset).build();

				if !AssetToVault::<MockRuntime>::contains_key(vault_config.asset_id) {
					assert_ok!(TokenizedOptions::create_asset_vault(Origin::signed(ADMIN), vault_config.clone()));
					assert!(AssetToVault::<MockRuntime>::contains_key(vault_config.asset_id));
				} else {
					assert_noop!(
						TokenizedOptions::create_asset_vault(Origin::signed(ADMIN), vault_config),
						Error::<MockRuntime>::AssetVaultAlreadyExists
					);
				}
			});
		});
	}

}
