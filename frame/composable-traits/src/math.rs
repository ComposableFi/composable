

/// Number like of higher bits, so that amount and balance calculations are done it it with higher
/// precision via fixed point.
/// While this is 128 bit, cannot support u128 because 18 bits are for of mantissa (so maximal integer is 110 bit).
/// Can support u128 if lift upper to use FixedU256 analog.
type LiftedFixedBalance = FixedU128;
