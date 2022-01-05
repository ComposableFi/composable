use frame_support::pallet_prelude::*;

/// A wrapper around the `Relayer` configuration which forces the user to respect the TTL and update
/// the relayer `AccountId` if mandated.
#[derive(Decode, Encode, TypeInfo, Default)]
pub struct StaleRelayer<AccountId, BlockNumber> {
	relayer: RelayerConfig<AccountId, BlockNumber>,
}

impl<AccountId, BlockNumber> StaleRelayer<AccountId, BlockNumber> {
	/// Create a relayer configuration, without scheduling a new `AccountId`.
	pub fn new(account: AccountId) -> StaleRelayer<AccountId, BlockNumber> {
		StaleRelayer { relayer: RelayerConfig { current: Some(account), next: None } }
	}
}

impl<AccountId, BlockNumber: PartialOrd> StaleRelayer<AccountId, BlockNumber> {
	/// Enforces Relayer TTL and returns the relayer configuration.
	pub fn update(self, now: BlockNumber) -> RelayerConfig<AccountId, BlockNumber> {
		self.relayer.rejig(now)
	}
}

/// Configuration for the relayer account.
#[derive(Decode, Encode, TypeInfo)]
pub struct RelayerConfig<AccountId, BlockNumber> {
	/// Current AccountId used by the relayer.
	current: Option<AccountId>,

	/// Scheduled update of the AccountId.
	next: Option<Next<AccountId, BlockNumber>>,
}

impl<AccountId, BlockNumber> From<RelayerConfig<AccountId, BlockNumber>>
	for StaleRelayer<AccountId, BlockNumber>
{
	fn from(relayer: RelayerConfig<AccountId, BlockNumber>) -> Self {
		Self { relayer }
	}
}

impl<AccountId, BlockNumber> Default for RelayerConfig<AccountId, BlockNumber> {
	fn default() -> Self {
		RelayerConfig { current: None, next: None }
	}
}

/// Next relayer configuration to be used.
#[derive(Decode, Encode, TypeInfo)]
pub struct Next<AccountId, BlockNumber> {
	ttl: BlockNumber,
	account: AccountId,
}

impl<AccountId: PartialEq, BlockNumber> RelayerConfig<AccountId, BlockNumber> {
	pub fn is_relayer(&self, account: &AccountId) -> bool {
		match &self.current {
			None => false,
			Some(acc) => acc == account,
		}
	}
}

impl<AccountId, BlockNumber: PartialOrd> RelayerConfig<AccountId, BlockNumber> {
	fn rejig(self, current: BlockNumber) -> Self {
		match self.next {
			None => self,
			Some(next) =>
				if next.ttl <= current {
					RelayerConfig { current: Some(next.account), next: None }
				} else {
					RelayerConfig { current: self.current, next: Some(next) }
				},
		}
	}
}

impl<AccountId, BlockNumber: PartialOrd> RelayerConfig<AccountId, BlockNumber> {
	pub fn rotate(mut self, account: AccountId, ttl: BlockNumber) -> Self {
		self.next = Some(Next { ttl, account });
		self
	}
}
