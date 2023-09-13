use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, DepsMut, MessageInfo, Order, Storage};
use cw_storage_plus::Map;
use xc_core::NetworkId;

use crate::{
	auth,
	error::{ContractError, Result},
	ibc, msg, state,
};

/// A user account.
#[cw_serde]
pub(crate) struct Account {
	/// Name of the account.  Corresponds to a local address on the chain of
	/// a wallet who controls the account.
	pub address: Addr,
	/// Data associated with the account.
	data: AccountData,
}

/// Data associated with a user account saved in the persistent storage.
#[derive(Default)]
#[cw_serde]
pub(crate) struct AccountData {
	/// Balance on the account.
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	balances: Vec<msg::AssetBalance>,
}

impl core::ops::Deref for Account {
	type Target = AccountData;

	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl core::ops::DerefMut for Account {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}

/// Map of user accounts.
const ACCOUNTS: Map<Addr, AccountData> = Map::new(state::ACCOUNTS_NS);
/// Set of user account recovery addresses.
///
/// The key uses `(address, (network_id, remote_address))` format.
const RECOVERY_ADDRESSES: Map<(Addr, (u32, String)), u8> = Map::new(state::RECOVERY_ADDRESSES_NS);

impl Account {
	/// Creates a new account if one with given address doesn’t already exist.
	fn create(storage: &mut dyn Storage, address: Addr) -> Result<Account> {
		let data = ACCOUNTS.update(storage, address.clone(), |data| {
			if data.is_some() {
				Err(ContractError::AlreadyRegistered)
			} else {
				Ok(AccountData { balances: Vec::new() })
			}
		})?;
		Ok(Self { address, data })
	}

	/// Fetches given account from the storage.
	///
	/// Returns `None` if the account doesn’t exist.
	pub fn load(storage: &dyn Storage, address: Addr) -> Result<Option<Account>> {
		let data = ACCOUNTS.may_load(storage, address.clone())?;
		Ok(data.map(|data| Self { address, data }))
	}

	/// Saves account data to permanent storage.
	fn save(&self, storage: &mut dyn Storage) -> Result<()> {
		Ok(ACCOUNTS.save(storage, self.address.clone(), &self.data)?)
	}

	/// Deletes an account.  It’s caller responsibility to make sure that all
	/// the funds have been transferred or otherwise handled.
	fn delete(&self, storage: &mut dyn Storage) {
		ACCOUNTS.remove(storage, self.address.clone());
		let keys = RECOVERY_ADDRESSES
			.prefix(self.address.clone())
			.keys_raw(storage, None, None, Order::Ascending)
			.collect::<Vec<_>>();
		for key in keys {
			storage.remove(&key)
		}
	}

	/// Checks whether provided remote address is a recovery account of this
	/// account.
	pub fn has_recovery_address(
		&self,
		storage: &dyn Storage,
		network_id: NetworkId,
		address: String,
	) -> bool {
		let key = (self.address.clone(), (network_id.into(), address.clone()));
		RECOVERY_ADDRESSES.has(storage, key)
	}

	/// Adds a new recovery address to the account; the operation is idempotent.
	pub fn add_recovery_address(
		&self,
		storage: &mut dyn Storage,
		network_id: NetworkId,
		address: String,
	) -> Result<()> {
		let key = (self.address.clone(), (network_id.into(), address.clone()));
		Ok(RECOVERY_ADDRESSES.save(storage, key, &1)?)
	}
}

/// Handles [`msg::CreateAccountRequest`] execution message.
pub(crate) fn handle_create_account(
	_auth: auth::User,
	deps: DepsMut,
	info: MessageInfo,
	req: msg::CreateAccountRequest,
) -> Result {
	let account = Account::create(deps.storage, info.sender)?;
	for addr in req.recovery_addresses {
		account.add_recovery_address(deps.storage, addr.network_id, addr.address)?;
	}
	Ok(Default::default())
}

pub(crate) fn handle_drop_account(
	auth: auth::Account,
	mut deps: DepsMut,
	req: msg::DropAccountRequest,
) -> Result<ibc::EmptyResponse> {
	let account = auth.account();
	if !account.balances.is_empty() {
		transfer_balances(&mut deps, &account, req.beneficiary_account)?;
	}
	account.delete(deps.storage);
	Ok(ibc::EmptyResponse)
}

/// Transfers all balance from given account to given beneficiary account.
fn transfer_balances(deps: &mut DepsMut, src_account: &Account, beneficiary: String) -> Result<()> {
	let beneficiary = deps.api.addr_validate(&beneficiary)?;
	let mut beneficiary = match Account::load(deps.storage, beneficiary)? {
		Some(account) => account,
		None => return Err(ContractError::UnknownAccount),
	};
	let mut balances = beneficiary
		.balances
		.iter()
		.map(|balance| (balance.asset_id, (*balance.unlocked_amount, *balance.locked_amount)))
		.collect::<std::collections::HashMap<_, _>>();
	for asset in src_account.balances.iter() {
		if asset.locked_amount != 0 {
			return Err(ContractError::HasLockedBalance(asset.asset_id))
		}
		balances.entry(asset.asset_id).or_default().0 += *asset.unlocked_amount;
	}
	beneficiary.balances.clear();
	beneficiary.balances.extend(balances.into_iter().map(
		|(asset_id, (unlocked_amount, locked_amount))| msg::AssetBalance {
			asset_id,
			unlocked_amount: unlocked_amount.into(),
			locked_amount: locked_amount.into(),
		},
	));
	beneficiary.save(deps.storage).map_err(Into::into)
}

pub(crate) fn handle_deposit_notification(
	_: auth::EscrowContract,
	deps: DepsMut,
	packet: msg::DepositNotificationPacket,
) -> Result<ibc::EmptyResponse> {
	use std::collections::hash_map::Entry;

	let path = ACCOUNTS.key(deps.api.addr_validate(&packet.account)?);
	let mut account = path.may_load(deps.storage)?.unwrap_or_default();

	let mut idx_from_asset = account
		.balances
		.iter()
		.enumerate()
		.map(|(idx, balance)| (balance.asset_id, idx))
		.collect::<std::collections::HashMap<xc_core::AssetId, usize>>();
	for (asset_id, amount) in packet.deposits {
		match idx_from_asset.entry(asset_id) {
			Entry::Occupied(entry) => {
				let entry = &mut account.balances[*entry.get()];
				entry.unlocked_amount = entry
					.unlocked_amount
					.checked_add(*amount)
					.ok_or(ContractError::ArithmeticOverflow)?
					.into();
			},
			Entry::Vacant(entry) => {
				entry.insert(account.balances.len());
				account.balances.push(msg::AssetBalance {
					asset_id,
					unlocked_amount: amount,
					locked_amount: Default::default(),
				});
			},
		}
	}

	path.save(deps.storage, &account)?;
	Ok(ibc::EmptyResponse)
}

pub(crate) fn handle_submit_problem(
	_auth: auth::Account,
	_deps: DepsMut,
	_req: msg::ExecuteSolutionRequest,
) -> Result<ibc::EmptyResponse> {
	todo!()
}
