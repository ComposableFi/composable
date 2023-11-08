use cw_storage_plus::Map;
use xc_core::service::dex::ExchangeItem;

pub(crate) const EXCHANGE: Map<u128, ExchangeItem> = Map::new("exchange");
