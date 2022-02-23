use sp_runtime::generic::Era;
use substrate_xt::ConstructExt;

/// Concrete event type for verbose event asserts in tests.
#[allow(clippy::large_enum_variant)]
#[derive(derive_more::From)]
pub enum AllRuntimeEvents {
	/// Picassoo runtime events
	Picasso(picasso_runtime::Event),
	/// Dali runtime events
	Dali(dali_runtime::Event),
}

/// Convenience method to match on [`AllRuntimeEvents`]
#[macro_export]
macro_rules! match_event {
	($ev:expr, $event:ident, $sub_ev:pat) => {{
		matches!(
			$ev,
			AllRuntimeEvents::Picasso(picasso_runtime::Event::$event($sub_ev))
				| AllRuntimeEvents::Dali(dali_runtime::Event::$event($sub_ev))
		)
	}};
}

pub struct DaliXtConstructor;

impl ConstructExt for DaliXtConstructor {
	type Runtime = dali_runtime::Runtime;
	type Pair = sp_core::sr25519::Pair;
	type SignedExtra = dali_runtime::SignedExtra;

	fn signed_extras(
		account_id: <Self::Runtime as frame_system::Config>::AccountId,
	) -> Self::SignedExtra {
		let nonce = frame_system::Pallet::<Self::Runtime>::account_nonce(account_id);
		(
			frame_system::CheckNonZeroSender::<Self::Runtime>::new(),
			frame_system::CheckSpecVersion::<Self::Runtime>::new(),
			frame_system::CheckTxVersion::<Self::Runtime>::new(),
			frame_system::CheckGenesis::<Self::Runtime>::new(),
			frame_system::CheckEra::<Self::Runtime>::from(Era::Immortal),
			frame_system::CheckNonce::<Self::Runtime>::from(nonce),
			frame_system::CheckWeight::<Self::Runtime>::new(),
			transaction_payment::ChargeTransactionPayment::<Self::Runtime>::from(0),
		)
	}
}

pub struct PicassoXtConstructor;

impl ConstructExt for PicassoXtConstructor {
	type Runtime = picasso_runtime::Runtime;
	type Pair = sp_core::sr25519::Pair;
	type SignedExtra = picasso_runtime::SignedExtra;

	fn signed_extras(
		account_id: <Self::Runtime as frame_system::Config>::AccountId,
	) -> Self::SignedExtra {
		let nonce = frame_system::Pallet::<Self::Runtime>::account_nonce(account_id);
		(
			frame_system::CheckNonZeroSender::<Self::Runtime>::new(),
			frame_system::CheckSpecVersion::<Self::Runtime>::new(),
			frame_system::CheckTxVersion::<Self::Runtime>::new(),
			frame_system::CheckGenesis::<Self::Runtime>::new(),
			frame_system::CheckEra::<Self::Runtime>::from(Era::Immortal),
			frame_system::CheckNonce::<Self::Runtime>::from(nonce),
			frame_system::CheckWeight::<Self::Runtime>::new(),
			transaction_payment::ChargeTransactionPayment::<Self::Runtime>::from(0),
		)
	}
}
