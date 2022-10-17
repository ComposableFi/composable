use orml_traits::MultiReservableCurrency;
use sp_runtime::DispatchError;

/// feesless exchange of reserved currencies
pub trait DefiMultiReservableCurrency<AccountId>: MultiReservableCurrency<AccountId> {
	fn exchange_reserved(
		base: Self::CurrencyId,
		seller: &AccountId,
		take_amount: Self::Balance,
		quote: Self::CurrencyId,
		taker: &AccountId,
		quote_amount: Self::Balance,
	) -> Result<(), DispatchError> {
		Self::unreserve(base, seller, take_amount);
		Self::unreserve(quote, taker, quote_amount);
		Self::transfer(base, seller, taker, take_amount)?;
		Self::transfer(quote, taker, seller, quote_amount)?;
		Ok(())
	}
}

impl<T, AccountId> DefiMultiReservableCurrency<AccountId> for T where
	T: MultiReservableCurrency<AccountId>
{
}
