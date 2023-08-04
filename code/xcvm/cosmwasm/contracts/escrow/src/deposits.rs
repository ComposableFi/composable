//! Module handling deposits.

use cosmwasm_std::{
	to_vec, Addr, Binary, DepsMut, Env, IbcBasicResponse, MessageInfo, Response, Storage,
};
use cw_storage_plus::{Item, Map};

use xc_core::AssetId;

use crate::{
	assets,
	assets::Cw20Ext,
	auth,
	error::{ContractError, Result},
	ibc, msg, state,
};

/// A deposit that has been communicated to the accounts contract but haven’t
/// been acknowledged yet.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct PendingDeposit {
	/// Name of the account in the virtual wallet to deposit funds to.
	account: String,
	/// Sender on local chain who made the deposit.  In case of failure, funds
	/// are returned to them.
	sender: Addr,
	/// Funds attached to this message to deposit to the user.
	deposits: Vec<(AssetId, assets::Local, u128)>,
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

/// Initialises the state of the contract.  Must be called when contract is
/// instantiated.
pub(crate) fn init_state(storage: &mut dyn Storage) -> Result<()> {
	LAST_DEPOSIT_ID.save(storage, &0).map_err(ContractError::from)
}

/// Handles a [`msg::DepositRequest`] message.
///
/// Counts all the coins attached to the message and then sends deposits
/// notification to the accounts contract.
///
/// Since we don’t care who sends those funds, this is unprivileged call.
pub(crate) fn handle_deposit_request(
	deps: DepsMut,
	env: Env,
	info: MessageInfo,
	msg::DepositAssetsRequest { account, tokens }: msg::DepositAssetsRequest,
) -> Result {
	let mut deposits = Vec::with_capacity(info.funds.len() + tokens.len());

	// Look through native funds attached to the message.
	for coin in info.funds {
		let (asset_id, denom) = assets::resolve_denom(deps.storage, coin.denom)?;
		deposits.push((asset_id, denom.into(), coin.amount.into()));
	}

	// Look through CW20 tokens.  Those are tokens that user gave us allowance
	// but hasn’t transferred yet.  We use the allowance to transfer the tokens
	// to us.  If any of the transfer fails, the entire transaction fails and
	// deposit doesn’t happen.
	let mut response = Response::default();
	response.messages.reserve(tokens.len());
	for (cw20_addr, amount) in tokens {
		let cw20_addr = deps.api.addr_validate(&cw20_addr)?;
		let (asset_id, cw20_addr) = assets::resolve_cw20(deps.storage, cw20_addr)?;
		response = response.add_message(cw20_addr.make_take_msg(
			&env.contract,
			info.sender.clone(),
			amount,
		)?);
		deposits.push((asset_id, cw20_addr.into(), amount));
	}

	send_deposit(response, deps.storage, info.sender, account, deposits)
}

/// Handles a [`msg::ExecuteMsg::Receive`] message from a CW20 contract.
///
/// This is called from the CW20 contract after the asset is transferred to us.
/// We just need to update the count in accounts contract on the main chain.
pub(crate) fn handle_receive(
	auth: auth::Cw20Contract,
	deps: DepsMut,
	_env: Env,
	cw20::Cw20ReceiveMsg { sender, amount, msg }: cw20::Cw20ReceiveMsg,
) -> Result {
	let sender = deps.api.addr_validate(&sender)?;
	let msg::ReceiveMsgBody { account } =
		serde_json_wasm::from_slice(&msg).map_err(|_| ContractError::InvalidPacket)?;
	let auth::policy::Cw20Contract { asset_id, address } = auth.into_inner();

	let deposits = vec![(asset_id, address.into(), amount.into())];
	send_deposit(<_>::default(), deps.storage, sender, account, deposits)
}

/// Records a pending deposit and sends notification about it to accounts
/// contract.
fn send_deposit(
	response: Response,
	storage: &mut dyn Storage,
	sender: Addr,
	account: String,
	mut deposits: Vec<(AssetId, assets::Local, u128)>,
) -> Result {
	deposits.retain(|&(_, _, amount)| amount > 0);
	if deposits.is_empty() {
		return Ok(Response::default())
	}

	let deposit_id = LAST_DEPOSIT_ID
		.update(storage, |id| id.checked_add(1).ok_or(ContractError::InternalError))?;

	let deposit = PendingDeposit { account, sender, deposits };
	PENDING_DEPOSITS.save(storage, deposit_id, &deposit)?;

	let deposits = deposit.deposits.into_iter().map(|(id, _, amount)| (id, amount)).collect();
	let packet =
		msg::accounts::DepositNotificationPacket { deposit_id, account: deposit.account, deposits };
	let send_packet = ibc::make_message(&msg::accounts::Packet::from(packet));

	let data = to_vec(&msg::DepositAssetsResponse { deposit_id })?;
	Ok(response.add_message(send_packet).set_data(data))
}

/// Handles acknowledgement or timeout of the deposit notification message sent
/// to the accounts contract.
///
/// If `ack` is `None`, the packet has timed out.  Otherwise, `ack` is the
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

	let mut response = IbcBasicResponse::default();
	if !ok {
		let PendingDeposit { sender, deposits, .. } = key.load(deps.storage)?;
		response = refund_deposits(response, sender, deposits)?;
	}

	key.remove(deps.storage);

	let event = msg::make_event(msg::Action::DepositDone)
		.add_attribute("deposit_id", packet.deposit_id.to_string())
		.add_attribute("result", result);
	Ok(response.add_event(event))
}

/// Sends funds back to the `recipient`.  The refunds are added to the
/// `response` as messages to execute once contract finishes.
fn refund_deposits(
	mut response: IbcBasicResponse,
	recipient: Addr,
	deposits: Vec<(AssetId, assets::Local, u128)>,
) -> Result<IbcBasicResponse> {
	let mut coins = Vec::new();

	// CW20 tokens are added one-by-one.  Local tokens are first collected into
	// `coins` so they can be handled with a single Bank message.
	for (_, local, amount) in deposits {
		match local {
			assets::Local::Native(denom) => {
				let coin = cosmwasm_std::Coin { denom: denom.into(), amount: amount.into() };
				coins.push(coin);
			},
			assets::Local::Cw20(addr) => {
				let msg = addr.make_transfer_msg(recipient.clone(), amount)?;
				response = response.add_message(msg);
			},
		}
	}

	// And now the local tokens in a single message.
	if !coins.is_empty() {
		let msg = assets::make_bank_transfer_msg(recipient, coins);
		response = response.add_message(msg);
	}

	Ok(response)
}
