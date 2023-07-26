use cw_storage_plus::Map;
use xc_core::service::dex::{ExchangeId, ExchangeItem};

pub(crate) const EXCHANGE: Map<ExchangeId, ExchangeItem> = Map::new("exchange");
