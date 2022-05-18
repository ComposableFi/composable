pub type BlockNumber = u64;
pub type Amount = i128;
pub type Balance = u64;
pub type AccountId = u64;
pub type AssetId = u128;

pub const MINIMUM_BALANCE: Balance = 1000;
pub const ALICE: AccountId = 0;
pub const BOB: AccountId = 1;
pub const CHARLIE: AccountId = 2;
pub const JEREMY: AccountId = 3;
pub const DARWIN: AccountId = 5;
pub const ACCOUNT_FREE_START: AccountId = JEREMY + 1;
pub const ACCOUNT_INITIAL_AMOUNT: u128 = 1_000_000;
