//! Composable substrate testing.

// so we can run these tests on simnode or real node later :)

pub fn alice() -> [u8; 32] {
	sp_keyring::sr25519::Keyring::Alice.into()
}

pub fn bob() -> [u8; 32] {
	sp_keyring::sr25519::Keyring::Bob.into()
}

pub fn charlie() -> [u8; 32] {
	sp_keyring::sr25519::Keyring::Charlie.into()
}

pub const ALICE: [u8; 32] = [4_u8; 32];
pub const BOB: [u8; 32] = [5_u8; 32];

/// 40 < 42 < 40 + 3
#[macro_export]
macro_rules! assert_gt_by {
	($actual:expr, $lower:expr, $positive_delta:expr $(,)?) => {{
		more_asserts::assert_gt!($actual, $lower);
		more_asserts::assert_le!($actual, $lower + $positive_delta);
	}};
}

/// 43 - 3 < 42 < 43
/// ```ignore
/// local_integration_tests::assert_lt_by!(42,43,3);
/// ```
#[macro_export]
macro_rules! assert_lt_by {
	($actual:expr, $upper:expr, $negative_delta:expr $(,)?) => {{
		more_asserts::assert_lt!($actual, $upper);
		more_asserts::assert_ge!($actual, $upper - $negative_delta);
	}};
}
