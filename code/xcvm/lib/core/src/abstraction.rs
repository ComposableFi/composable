//! https://en.wikipedia.org/wiki/Peano_axioms
//! https://wiki.haskell.org/Peano_numbers#:~:text=Peano%20numbers%20are%20a%20simple,arithmetic%20due%20to%20their%20simplicity.
use core::marker::PhantomData;

/// Dummy structure used for the compiler to infer the position inside a tuple.
/// Base case, 0
pub struct Zero;
/// Dummy structure used for the compiler to infer the position inside a tuple.
/// Inductive case, 1 + x
pub struct Succ<T>(PhantomData<T>);

mod _priv {
	pub trait Sealed {}
	impl Sealed for super::Zero {}
	impl<X> Sealed for super::Succ<X> {}
}

pub trait Nat: _priv::Sealed {
	const VALUE: u8;
}

impl Nat for Zero {
	const VALUE: u8 = 0;
}

impl<X: Nat> Nat for Succ<X> {
	const VALUE: u8 = 1 + X::VALUE;
}

/// Compile time indexing of an element of type `T` inside a structure of type `U`
pub trait IndexOf<T, U> {
	const INDEX: u8;
}

/// Base case
impl<T, U> IndexOf<T, Zero> for (T, U) {
	const INDEX: u8 = Zero::VALUE;
}

/// Inductive case
impl<T, U, V, X> IndexOf<T, Succ<X>> for (U, V)
where
	X: Nat,
	V: IndexOf<T, X>,
{
	const INDEX: u8 = <Succ<X>>::VALUE;
}
