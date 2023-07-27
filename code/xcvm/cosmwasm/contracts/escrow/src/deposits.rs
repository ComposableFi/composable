//! Module handling deposits.

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_vec, Addr, Binary, DepsMut, Env, IbcBasicResponse, MessageInfo, Response};
use cw_storage_plus::{Item, Map};

use crate::{
	auth,
	error::{ContractError, Result},
	ibc, msg, state,
};

/// A deposit that has been communicated to the accounts contract but haven’t
/// been acknowledged yet.
#[cw_serde]
struct PendingDeposit {
	/// Name of the account in the virtual wallet to deposit funds to.
	account: String,
	/// Sender on local chain who made the deposit.  In case of failure, funds
	/// are returned to them.
	sender: Addr,
	/// Funds attached to this message to deposit to the user.
	deposits: Vec<msg::LocalAssetAmount>,
}

/// All pending deposits.
///
/// Whenever sender makes a deposit, it’s assigned unique identifier and added
/// to this map.  Once deposit is acknowledged by the accounts contract the
/// pending deposit is removed from the list.  If deposit has been rejected, the
/// funds are returned to sender.
const PENDING_DEPOSITS: Map<u128, PendingDeposit> = Map::new(state::PENDING_DEPOSITS_NS);

/// ID of the last deposit made.  Unique within a single escrow contract.
const LAST_DEPOSIT_ID: Item<u128> = Item::new(state::LAST_DEPOSIT_ID_NS);

/// Handles a [`msg::DepositAssetsRequest`] message from a user.
///
/// Verifies that the sender attached all the funds they claim and then
pub(crate) fn handle_deposit_assets(
	_: auth::User,
	deps: DepsMut,
	_env: Env,
	info: MessageInfo,
	msg::DepositAssetsRequest { account, deposits }: msg::DepositAssetsRequest,
) -> Result {
	let deposit_id = LAST_DEPOSIT_ID
		.load(deps.storage)?
		.checked_add(1)
		.ok_or(ContractError::ArithmeticOverflow)?;

	let deposit = PendingDeposit { account, sender: info.sender, deposits };
	PENDING_DEPOSITS.save(deps.storage, deposit_id, &deposit)?;

	let account = deposit.account;
	let deposits = deposit
		.deposits
		.into_iter()
		.map(|asset| verify_deposit(&deps, asset))
		.collect::<Result<_, _>>()?;
	let packet = msg::accounts::DepositNotificationPacket { deposit_id, account, deposits };
	let send_packet = ibc::make_message(&msg::accounts::Packet::from(packet));

	let data = to_vec(&msg::DepositAssetsResponse { deposit_id })?;

	Ok(Response::default().add_message(send_packet).set_data(data))
}

/// Verifies a deposit and converts local asset identifiers into global ones.
///
/// Fails if the local asset is not recognised or user hasn’t transferred
/// declared asset.
fn verify_deposit(
	_deps: &DepsMut,
	_asset: msg::LocalAssetAmount,
) -> Result<msg::accounts::DepositAmount> {
	todo!()
}

/// Handles acknowledgement or timeout of the deposit notification message sent
/// to the accounts contract.
///
/// If `ack` is `None`, the packet has timed out.  Otherwise, `ack` indicate
/// response from the accounts contract `0u8` if it was rejected or `1u8` if it
/// was successful.
///
/// Unless deposit was successful, all funds will be returned to the sender.
pub(crate) fn handle_deposit_done(
	deps: DepsMut,
	packet: msg::accounts::DepositNotificationPacket,
	ack: Option<Binary>,
) -> Result<IbcBasicResponse> {
	let ack = ack.map(|binary| ibc::decode::<bool>(binary)).transpose()?;
	let (ok, result) = match ack {
		Some(ok) => (ok, if ok { "OK" } else { "KO" }),
		None => (false, "TO"),
	};

	let key = PENDING_DEPOSITS.key(packet.deposit_id);
	key.remove(deps.storage);

	if !ok {
		let _deposit = key.load(deps.storage)?;
		// TODO: Refund sender.
		todo!()
	}

	let event = msg::make_event(msg::Action::DepositDone)
		.add_attribute("deposit_id", packet.deposit_id.to_string())
		.add_attribute("result", result);
	Ok(IbcBasicResponse::default().add_event(event))
}
