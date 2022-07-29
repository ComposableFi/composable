use core::marker::PhantomData;

/// Dummy structure used for the compiler to infer the position inside a tuple.
/// Base case, 0
pub struct Zero;
/// Dummy structure used for the compiler to infer the position inside a tuple.
/// Inductive case, x + 1
pub struct Succ<T>(PhantomData<T>);

/// Compile time indexing of an element of type `T` inside a structure of type `U`
pub trait IndexOf<T, U> {
	const INDEX: u8;
}

/// Base case
impl<T, U> IndexOf<T, Zero> for (T, U) {
	const INDEX: u8 = 0;
}

/// Inductive case
impl<T, U, V, X> IndexOf<T, Succ<X>> for (U, V)
where
	V: IndexOf<T, X>,
{
	const INDEX: u8 = 1 + V::INDEX;
}
