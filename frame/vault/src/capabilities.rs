use bitflags::bitflags;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

bitflags! {
	/// Capabilities (or more accurately, `incapabilities`) restrict functionality of specific vault
	/// instances.
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct Capabilities: u32 {
		/// Tomb-stoning a vault is schedules it for deletion.
		const TOMBSTONED = 0b000000001;
		/// Stopped vaults have all functionality suspended. Use this state for emergency halts.
		const STOPPED = 0b000000010;
		/// Suspends withdrawals from the vault. In general this should not be used, and the entire
		/// vault should instead be `Stopped`.
		const WITHDRAWALS_STOPPED = 0b000000100;
		/// Suspends deposits to the vault.
		const DEPOSITS_STOPPED = 0b000001000;
	}
}

impl Default for Capabilities {
	fn default() -> Self {
		Self::empty()
	}
}

impl Capabilities {
	#[inline]
	pub fn is_active(&self) -> bool {
		!self.is_inactive()
	}

	#[inline]
	pub fn is_stopped(&self) -> bool {
		self.contains(Self::STOPPED)
	}

	#[inline]
	pub fn is_inactive(&self) -> bool {
		self.contains(Self::TOMBSTONED) || self.is_stopped()
	}

	#[inline]
	pub fn is_tombstoned(&self) -> bool {
		self.contains(Self::TOMBSTONED)
	}

	#[inline]
	pub fn set_tombstoned(&mut self) {
		self.insert(Self::TOMBSTONED)
	}

	#[inline]
	pub fn untombstone(&mut self) {
		self.remove(Self::TOMBSTONED)
	}

	#[inline]
	pub fn set_stopped(&mut self) {
		self.insert(Self::STOPPED)
	}

	#[inline]
	pub fn start(&mut self) {
		self.remove(Self::STOPPED)
	}

	#[inline]
	pub fn stop_deposits(&mut self) {
		self.insert(Self::DEPOSITS_STOPPED)
	}

	#[inline]
	pub fn stop_withdrawals(&mut self) {
		self.insert(Self::WITHDRAWALS_STOPPED)
	}

	#[inline]
	pub fn allow_deposits(&mut self) {
		self.remove(Self::DEPOSITS_STOPPED)
	}

	#[inline]
	pub fn allow_withdrawals(&mut self) {
		self.remove(Self::WITHDRAWALS_STOPPED)
	}

	#[inline]
	pub fn withdrawals_allowed(&self) -> bool {
		!self.withdrawals_stopped()
	}

	#[inline]
	pub fn withdrawals_stopped(&self) -> bool {
		self.contains(Self::WITHDRAWALS_STOPPED) || self.is_stopped()
	}

	#[inline]
	pub fn deposits_allowed(&self) -> bool {
		!self.deposits_stopped()
	}

	#[inline]
	pub fn deposits_stopped(&self) -> bool {
		self.contains(Self::DEPOSITS_STOPPED) || self.is_inactive()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn default_has_everything_enabled() {
		let cap = Capabilities::default();
		assert!(cap.is_active());
		assert!(cap.deposits_allowed());
		assert!(cap.withdrawals_allowed());
	}

	#[test]
	fn tombstoned_is_inactive() {
		let mut cap = Capabilities::default();
		cap.set_tombstoned();
		assert!(cap.is_inactive());
		// We want tombstoned vaults to still allow withdrawals, as users need to evacuate funds.
		assert!(cap.withdrawals_allowed());
		assert!(cap.deposits_stopped());
	}

	#[test]
	fn stopped_is_inactive() {
		let mut cap = Capabilities::default();
		cap.set_stopped();
		assert!(cap.is_inactive());
		assert!(cap.withdrawals_stopped());
		assert!(cap.deposits_stopped());
	}

	#[test]
	fn deposits_halted() {
		let mut cap = Capabilities::default();
		cap.stop_deposits();
		assert!(cap.is_active());
		assert!(cap.withdrawals_allowed());
		assert!(cap.deposits_stopped());
	}

	#[test]
	fn withdrawals_halted() {
		let mut cap = Capabilities::default();
		cap.stop_withdrawals();
		assert!(cap.is_active());
		assert!(cap.withdrawals_stopped());
		assert!(cap.deposits_allowed());
	}
}
