//! mapping of proper CW/Cosmos enabled types to Centauri

use crate::prelude::*;
use xc_core::ibc::ics20::MemoData;

pub struct Map;
impl Map {
	pub fn from_cw(mut value: MemoData) -> pallet_ibc::ics20::MemoData {
		let next = value.forward.next.take().map(|e| Box::new(Map::from_cw(*e)));
		let value = value.forward;
		let forward = pallet_ibc::ics20::Forward {
			receiver: value.receiver,
			port: value.port.map(|x| x.to_string()),
			channel: value.channel.map(|x| x.to_string()),
			timeout: value.timeout,
			retries: value.retries.map(Into::into),
			para_id: value.substrate.map(|x| x.para_id).flatten(),
			substrate: value.substrate.map(|_| true),
			next,
		};

		pallet_ibc::ics20::MemoData { forward }
	}
}
