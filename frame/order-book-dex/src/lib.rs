


mod pallet {

	trait Config {
		type Balance;
		type AccountId;
	}

	pub struct DexInitialization {

	}

	pub enum OrderStatus {

	}

	/// Store on chain multi dictionary key (from, to, account) , dictionary per buy and sell
	pub struct Order<T:Config>
	{
		pub amount: Balance,
		pub price : T:Balance,
		pub time_stamp : UnixTimestamp,
		pub trader : AccountId,
		pub status: OrderStatus,
	}
}

