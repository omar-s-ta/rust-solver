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
//! Each level is a blanket-implemented marker trait: any type that satisfies
//! the required operator bounds automatically qualifies, so you write these as
//! `where T: AdditionGroup` bounds and never implement them by hand. The one
//! thing a concrete type *must* provide is [`Zero`] (and [`One`], for
//! multiplicative contexts); all built-in integer types get both via the
//! blanket impls at the bottom of this module.

use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

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
