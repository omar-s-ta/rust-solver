//! Algebraic structure traits for generic numeric code.
//!
//! Competitive-programming templates often want to be generic over "things you
//! can add" without caring whether the concrete type is an `i64`, a modular
//! integer, a matrix, or a polynomial. These traits capture that shared
//! structure as a small ladder of algebraic abstractions, each adding one more
//! operation on top of the previous one:
//!
//! - [`AdditionMonoid`] — has a [`Zero`] identity and associative `+`.
//! - [`AdditionMonoidWithSub`] — additionally supports `-` (e.g. saturating or
//!   wrapping unsigned subtraction, where a full inverse need not exist).
//! - [`AdditionGroup`] — additionally has unary negation, so every element has
//!   an additive inverse.
//!
//! The multiplicative side has its own ladder, and the two sides combine into
//! the usual ring-like structures:
//!
//! - [`MultiplicationMonoid`] — has a [`One`] identity and associative `*`.
//! - [`IntegerMultiplicationMonoid`] — additionally supports truncating `/` and
//!   `%`, as on built-in integers.
//! - [`MultiplicationGroup`] — additionally supports `/` through a checked
//!   multiplicative inverse ([`Invertible`]).
//! - [`SemiRing`], [`Ring`], [`Field`] and their `Integer*`/`WithSub` variants
//!   — an addition trait paired with a multiplication trait (see each one for
//!   the exact pairing).
//!
//! Each level is a blanket-implemented marker trait: any type that satisfies
//! the required operator bounds automatically qualifies, so you write these as
//! `where T: AdditionGroup` bounds and never implement them by hand. The one
//! thing a concrete type *must* provide is [`Zero`] (and [`One`], for
//! multiplicative contexts); all built-in integer types get both via the
//! blanket impls at the bottom of this module.
//!
//! The compiler only checks that the operators exist, not that they obey the
//! algebraic laws (associativity, distributivity, ...). A type with a
//! non-associative `+` will still satisfy [`AdditionMonoid`]; upholding the laws
//! is the implementor's responsibility.

use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::math::invertible::Invertible;

/// The additive identity of a type: the value `x` for which `x + a == a`.
pub trait Zero {
    fn zero() -> Self;
}

/// The multiplicative identity of a type: the value `x` for which `x * a == a`.
pub trait One {
    fn one() -> Self;
}

/// Declares a marker trait aliasing a set of bounds, plus a blanket impl for
/// every type meeting them — so `$atype` becomes usable as a single `where`
/// bound with nothing to implement by hand. Used for the addition-trait ladder
/// below, where each level just adds one operator bound to the previous.
macro_rules! alias_with_blanket_impl {
    ($(#[$meta: meta])* $atype: ident: $($bound: tt)+) => {
        $(#[$meta])*
        pub trait $atype: $($bound)+ {}
        impl<T: $($bound)+> $atype for T {}
    };
}

alias_with_blanket_impl!(
    /// A type with an associative `+` and a [`Zero`] identity, comparable for
    /// equality.
    ///
    /// This is the weakest of the addition traits: it only guarantees that values
    /// can be added and combined but not necessarily undone. The [`Eq`] bound lets
    /// generic algorithms compare against `zero()` (for example, to detect an empty
    /// contribution). It is blanket-implemented, so there is nothing to write for a
    /// concrete type beyond the underlying operator impls and [`Zero`].
    AdditionMonoid: Add<Output = Self> + AddAssign + Zero + Eq + Sized
);

alias_with_blanket_impl!(
    /// An [`AdditionMonoid`] that also supports subtraction.
    ///
    /// Kept separate from [`AdditionGroup`] because subtraction and negation are
    /// distinct capabilities: unsigned integers can subtract (`Sub`/`SubAssign`)
    /// but cannot negate, so they land exactly here and no higher.
    AdditionMonoidWithSub: AdditionMonoid + Sub<Output = Self> + SubAssign
);

alias_with_blanket_impl!(
    /// A full additive group: an [`AdditionMonoidWithSub`] in which every element
    /// has an additive inverse via unary [`Neg`].
    ///
    /// Signed integers and modular integers satisfy this; unsigned integers do not.
    /// Require this bound when an algorithm genuinely needs negation rather than
    /// just subtraction.
    AdditionGroup: AdditionMonoidWithSub + Neg<Output = Self>
);

alias_with_blanket_impl!(
    /// A type with an associative `*` and a [`One`] identity, comparable for
    /// equality.
    ///
    /// The multiplicative counterpart of [`AdditionMonoid`]: values can be
    /// multiplied (e.g. for exponentiation by squaring) but not necessarily
    /// divided. Matrices and modular integers qualify, as do the built-in
    /// integers.
    MultiplicationMonoid: Mul<Output = Self> + MulAssign + One + Eq + Sized
);

alias_with_blanket_impl!(
    /// A [`MultiplicationMonoid`] with integer-style division: `/` and `%`
    /// (and their assigning forms).
    ///
    /// Intended for types where division is truncating with a remainder, such as
    /// the built-in integers, which is what algorithms like gcd need. It does
    /// not imply an inverse exists; use [`MultiplicationGroup`] for that.
    IntegerMultiplicationMonoid: MultiplicationMonoid + Div<Output = Self> + Rem<Output = Self> + DivAssign + RemAssign
);

alias_with_blanket_impl!(
    /// A [`MultiplicationMonoid`] in which elements can be inverted, giving
    /// exact division.
    ///
    /// Inversion goes through [`Invertible::inv`], which returns an `Option`
    /// because not every element has an inverse (zero never does; in a modular
    /// ring, neither do elements sharing a factor with the modulus). `/` is
    /// expected to agree with multiplying by that inverse.
    MultiplicationGroup: MultiplicationMonoid + Div<Output = Self> + DivAssign + Invertible<Output = Self>
);

alias_with_blanket_impl!(
    /// An [`AdditionMonoid`] paired with a [`MultiplicationMonoid`]: addition and
    /// multiplication, with no subtraction or division.
    ///
    /// Distributivity of `*` over `+` is assumed but not checked. Unsigned
    /// integers are a semiring, which is why path-counting style DP over `u64`
    /// can be written against this bound.
    SemiRing: AdditionMonoid + MultiplicationMonoid
);

alias_with_blanket_impl!(
    /// A [`SemiRing`] whose addition also supports subtraction (an
    /// [`AdditionMonoidWithSub`]), without requiring negation.
    ///
    /// This is the level unsigned integers reach: they can subtract but not
    /// negate, so they are a `SemiRingWithSub` but not a [`Ring`].
    SemiRingWithSub: SemiRing + AdditionMonoidWithSub
);

alias_with_blanket_impl!(
    /// A [`SemiRing`] whose multiplication has integer-style `/` and `%`
    /// ([`IntegerMultiplicationMonoid`]).
    ///
    /// Satisfied by every built-in integer type, signed or unsigned.
    IntegerSemiRing: SemiRing + IntegerMultiplicationMonoid
);

alias_with_blanket_impl!(
    /// An [`IntegerSemiRing`] that also supports subtraction.
    ///
    /// Combines [`IntegerSemiRing`] with [`SemiRingWithSub`]; the unsigned
    /// built-in integers sit here, signed ones go on to [`IntegerRing`].
    IntegerSemiRingWithSub: SemiRingWithSub + IntegerSemiRing
);

alias_with_blanket_impl!(
    /// An [`IntegerSemiRing`] that is also a [`Ring`]: integer-style `/` and `%`
    /// plus full additive inverses.
    ///
    /// Satisfied by the signed built-in integers.
    IntegerRing: IntegerSemiRing + Ring
);

alias_with_blanket_impl!(
    /// A [`SemiRing`] whose addition is an [`AdditionGroup`], so every element
    /// has an additive inverse.
    ///
    /// Signed integers and modular integers are rings; unsigned integers are not
    /// because they cannot negate.
    Ring: SemiRing + AdditionGroup
);

alias_with_blanket_impl!(
    /// A [`Ring`] whose multiplication is a [`MultiplicationGroup`]: addition,
    /// subtraction, multiplication and division by invertible elements.
    ///
    /// Modular integers with a prime modulus are the typical example. As with
    /// [`MultiplicationGroup`], division is only defined where
    /// [`Invertible::inv`] returns `Some`, so zero is excluded.
    Field: Ring + MultiplicationGroup
);

/// Implements [`Zero`] and [`One`] for each primitive integer type listed,
/// using the literals `0` and `1`.
macro_rules! zero_one_impl {
    ($($t: ty)+) => {$(
        impl Zero for $t {
            fn zero() -> Self {
                0
            }
        }

        impl One for $t {
            fn one() -> Self {
                1
            }
        }
    )+};
}

zero_one_impl!(i128 i64 i32 i16 i8 isize u128 u64 u32 u16 u8 usize);
