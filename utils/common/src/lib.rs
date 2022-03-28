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
	/// Composable runtime events
	Composable(composable_runtime::Event),
}

/// Convenience method to match on [`AllRuntimeEvents`]
#[macro_export]
macro_rules! match_event {
	($ev:expr, $event:ident, $sub_ev:pat) => {{
		matches!(
			$ev,
			AllRuntimeEvents::Picasso(picasso_runtime::Event::$event($sub_ev)) |
				AllRuntimeEvents::Dali(dali_runtime::Event::$event($sub_ev)) |
				AllRuntimeEvents::Composable(composable_runtime::Event::$event($sub_ev))
		)
	}};
}

pub struct DaliXtConstructor;

impl ConstructExt for DaliXtConstructor {
	type Runtime = dali_runtime::Runtime;
	type Pair = sp_core::sr25519::Pair;
	type SignedExtra = dali_runtime::SignedExtra;

	fn signed_extras(
		account_id: <Self::Runtime as system::Config>::AccountId,
	) -> Self::SignedExtra {
		let nonce = system::Pallet::<Self::Runtime>::account_nonce(account_id);
		(
			system::CheckNonZeroSender::<Self::Runtime>::new(),
			system::CheckSpecVersion::<Self::Runtime>::new(),
			system::CheckTxVersion::<Self::Runtime>::new(),
			system::CheckGenesis::<Self::Runtime>::new(),
			system::CheckEra::<Self::Runtime>::from(Era::Immortal),
			system::CheckNonce::<Self::Runtime>::from(nonce),
			system::CheckWeight::<Self::Runtime>::new(),
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
		account_id: <Self::Runtime as system::Config>::AccountId,
	) -> Self::SignedExtra {
		let nonce = system::Pallet::<Self::Runtime>::account_nonce(account_id);
		(
			system::CheckNonZeroSender::<Self::Runtime>::new(),
			system::CheckSpecVersion::<Self::Runtime>::new(),
			system::CheckTxVersion::<Self::Runtime>::new(),
			system::CheckGenesis::<Self::Runtime>::new(),
			system::CheckEra::<Self::Runtime>::from(Era::Immortal),
			system::CheckNonce::<Self::Runtime>::from(nonce),
			system::CheckWeight::<Self::Runtime>::new(),
			transaction_payment::ChargeTransactionPayment::<Self::Runtime>::from(0),
		)
	}
}
