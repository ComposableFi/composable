/// basics of native runtime governance
pub mod native {
	use crate::AccountId;
	use frame_support::traits::EitherOfDiverse;
	use frame_system::EnsureRoot;
	pub type NativeCouncilCollective = collective::Instance1;
	pub type NativeTechnicalCollective = collective::Instance2;
	pub type NativeTreasury = treasury::Instance1;

	/// Origin for either root or half of PICA council
	pub type EnsureRootOrHalfNativeCouncil = EitherOfDiverse<
		EnsureRoot<AccountId>,
		collective::EnsureProportionAtLeast<AccountId, NativeCouncilCollective, 50, 100>,
	>;

	pub type EnsureRootOrMoreThenHalfNativeCouncil = EitherOfDiverse<
		EnsureRoot<AccountId>,
		collective::EnsureProportionMoreThan<AccountId, NativeCouncilCollective, 50, 100>,
	>;

	/// Origin for either root or half of general council
	pub type EnsureRootOrHalfNativeTechnical = EitherOfDiverse<
		EnsureRoot<AccountId>,
		collective::EnsureProportionAtLeast<AccountId, NativeTechnicalCollective, 50, 100>,
	>;

	pub type EnsureRootOrOneThirdNativeTechnical = EitherOfDiverse<
		EnsureRoot<AccountId>,
		collective::EnsureProportionAtLeast<AccountId, NativeTechnicalCollective, 1, 3>,
	>;

	pub type EnsureRootOrAllNativeTechnical = EitherOfDiverse<
		EnsureRoot<AccountId>,
		collective::EnsureProportionAtLeast<AccountId, NativeTechnicalCollective, 50, 100>,
	>;

	pub type EnsureNativeTechnicalMember =
		collective::EnsureMember<AccountId, NativeTechnicalCollective>;
	pub type EnsureNativeCouncilMember =
		collective::EnsureMember<AccountId, NativeCouncilCollective>;

	pub type EnsureRootOrHalfNativeCouncilOrTechnical = EitherOfDiverse<
		EnsureRoot<AccountId>,
		EitherOfDiverse<
			collective::EnsureProportionAtLeast<AccountId, NativeTechnicalCollective, 50, 100>,
			collective::EnsureProportionAtLeast<AccountId, NativeCouncilCollective, 50, 100>,
		>,
	>;
}
