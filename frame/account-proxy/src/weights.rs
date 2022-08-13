#![allow(unused_parens, unused_imports, clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_proxy.
pub trait WeightInfo {
	fn proxy() -> Weight;
	fn proxy_announced() -> Weight;
	fn remove_announcement() -> Weight;
	fn reject_announcement() -> Weight;
	fn announce() -> Weight;
	fn add_proxy() -> Weight;
	fn remove_proxy() -> Weight;
	fn remove_proxies() -> Weight;
	fn anonymous() -> Weight;
	fn kill_anonymous() -> Weight;
}

/// Weights for pallet_proxy using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Proxy Proxies (r:1 w:0)
	fn proxy() -> Weight {
		17_768_000
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn proxy_announced() -> Weight {
		35_682_000
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn remove_announcement() -> Weight {
		25_586_000
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn reject_announcement() -> Weight {
		25_794_000
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn announce() -> Weight {
		33_002_000
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn add_proxy() -> Weight {
		28_166_000
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn remove_proxy() -> Weight {
		28_128_000
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn remove_proxies() -> Weight {
		24_066_000
	}
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Proxy Proxies (r:1 w:1)
	fn anonymous() -> Weight {
		31_077_000
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn kill_anonymous() -> Weight {
		24_657_000
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Proxy Proxies (r:1 w:0)
	fn proxy() -> Weight {
		17_768_000
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn proxy_announced() -> Weight {
		35_682_000
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn remove_announcement() -> Weight {
		25_586_000
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn reject_announcement() -> Weight {
		25_794_000
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn announce() -> Weight {
		33_002_000
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn add_proxy() -> Weight {
		28_166_000
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn remove_proxy() -> Weight {
		28_128_000
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn remove_proxies() -> Weight {
		24_066_000
	}
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Proxy Proxies (r:1 w:1)
	fn anonymous() -> Weight {
		31_077_000
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn kill_anonymous() -> Weight {
		24_657_000
	}
}
