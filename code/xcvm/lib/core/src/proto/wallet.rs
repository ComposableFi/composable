pub use alloc::{
	boxed::Box,
	collections::VecDeque,
	string::{String, ToString},
	vec,
	vec::Vec,
};
pub use core::str::FromStr;
pub use cosmwasm_std::{Addr, Binary, Coin};
pub use serde::{Deserialize, Serialize};

use super::{pb, NonEmptyExt};
use crate::accounts;

impl super::Isomorphism for accounts::Packet {
	type Message = pb::wallet::AccountsPacket;
}

super::define_conversion! {
	(msg: pb::wallet::AccountsPacket) -> {
		use pb::wallet::accounts_packet::Request;
		Ok(match msg.request.non_empty()? {
			Request::Deposit(req) => Self::DepositNotification(req.try_into()?),
			Request::RelayedRequest(req) => Self::RelayedRequest(req.try_into()?),
		})
	}

	(value: accounts::Packet) -> {
		use pb::wallet::accounts_packet::Request;
		let request = match value {
			accounts::Packet::DepositNotification(req) => Request::Deposit(req.into()),
			accounts::Packet::RelayedRequest(req) => Request::RelayedRequest(req.into()),
		};
		Self { request: Some(request) }
	}
}

super::define_conversion! {
	(msg: pb::wallet::DepositNotificationPacket) -> {
		let deposits = msg.deposits.non_empty()?.into_iter().map(|deposit| {
			let asset_id = deposit.asset_id.non_empty()?.into();
			let amount = deposit.amount.non_empty()?.into();
			Ok((asset_id, amount))
		})
			.collect::<Result<Vec<_>, ()>>()?;
		Ok(Self {
			deposit_id: msg.deposit_id.non_empty()?.into(),
			account: msg.account.non_empty()?,
			deposits,
		})
	}

	(value: accounts::DepositNotificationPacket) -> {
		let deposits = value
			.deposits
			.into_iter()
			.map(|(asset_id, amount)| {
				pb::wallet::Deposit {
					asset_id: Some(u128::from(asset_id).into()),
					amount: Some(amount.into()),
				}
			})
			.collect::<Vec<_>>();
		Self {
			deposit_id: Some(value.deposit_id.into()),
			account: value.account,
			deposits,
		}
	}
}

super::define_conversion! {
	(msg: pb::wallet::RelayedRequestPacket) -> {
		Ok(Self {
			address: msg.address.non_empty()?,
			account: msg.account.non_empty()?,
			request: msg.request.non_empty()?.try_into()?,
		})
	}

	(msg: accounts::RelayedRequestPacket) -> {
		Self {
			address: msg.address,
			account: msg.account,
			request: Some(msg.request.into()),
		}
	}
}

super::define_conversion! {
	(msg: pb::wallet::relayed_request_packet::Request) -> {
		use pb::wallet::relayed_request_packet::Request;
		Ok(match msg {
			Request::DropAccount(req) => Self::DropAccount(req.try_into()?),
			Request::ExecuteSolution(req) => Self::ExecuteSolution(req.try_into()?),
		})
	}

	(msg: accounts::RelayedRequest) -> {
		match msg {
			accounts::RelayedRequest::DropAccount(req) => Self::DropAccount(req.into()),
			accounts::RelayedRequest::ExecuteSolution(req) => Self::ExecuteSolution(req.into()),
		}
	}
}

super::define_conversion! {
	(msg: pb::wallet::DropAccountRequest) -> {
		Ok(Self {
			beneficiary_account: msg.beneficiary_account.non_empty()?
		})
	}

	(msg: accounts::DropAccountRequest) -> {
		Self {
			beneficiary_account: msg.beneficiary_account
		}
	}
}

super::define_conversion! {
	(_msg: pb::wallet::ExecuteSolutionRequest) -> {
		Ok(Self { })
	}

	(_msg: accounts::ExecuteSolutionRequest) -> {
		Self { }
	}
}
