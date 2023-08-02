//! mapping of proper CW/Cosmos enabled types to Centauri

use crate::prelude::*;
use xc_core::transport::ibc::ics20::Memo;

pub struct Map;
impl Map {
	pub fn try_from_xc_memo(value: Memo) -> Option<pallet_ibc::ics20::MemoData> {
		value.forward.map(|value| {
			let next: Option<Box<pallet_ibc::ics20::MemoData>> =
				value.next.and_then(|e| Map::try_from_xc_memo(*e)).map(Box::new);
			let forward = pallet_ibc::ics20::Forward {
				receiver: value.receiver,
				port: value.port.map(|x| x.to_string()),
				channel: value.channel.map(|x| x.to_string()),
				timeout: value.timeout,
				retries: value.retries.map(Into::into),
				para_id: value.substrate.and_then(|x| x.para_id),
				substrate: value.substrate.map(|_| true),
				next,
			};

			pallet_ibc::ics20::MemoData { forward }
		})
	}
}
