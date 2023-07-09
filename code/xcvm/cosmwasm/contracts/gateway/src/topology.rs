//! Helps connecting identifiers into networks.
//! Allows to map asset identifiers, contracts, networks, channels, denominations from, to and on each chain
//! via contract storage, precompiles, host extensions.

use cosmwasm_std::{Addr, IbcTimeoutBlock, IbcTimeout};
use xc_core::{AssetId, Centauri, Picasso, NetworkId, Network};

use crate::error::ContractError;


pub fn this() -> NetworkId {
	Picasso::ID
}

pub fn get_route(
	from: xc_core::NetworkId,
	to: xc_core::NetworkId,
	asset: AssetId,
) -> Result<(String, String, Addr, IbcTimeout), ContractError> {
	let  timeout = IbcTimeout::with_block(IbcTimeoutBlock { revision: 0, height: 10000 });
	match (from, to) {
		(Picasso::ID, Centauri::ID) => Ok((
			"channel-75".to_owned(),
			asset.to_string(),
			Addr::unchecked("xc contract on other side".to_string()), 
			timeout,
		)),
		(Centauri::ID, Picasso::ID) => Ok((
			"channel-1".to_owned(),
			asset.to_string(),
			Addr::unchecked("xc contract on other side".to_string()),
			timeout,
		)),
		_ => Err(ContractError::RouteNotFound),
	}
}
