/// A type with a smallest and a largest value.
///
/// This is the trait form of the inherent `T::MIN` / `T::MAX` constants that
/// every primitive integer has. Inherent constants cannot be named through a
/// type parameter, so generic code (for example, turning an unbounded range
/// into concrete endpoints) needs a trait to say "`T` has a `MIN` and a `MAX`".
///
/// ```
/// use algo_lib::math::bounded::Bounded;
///
/// // Works for any integer type, because `Bounded` is a bound, not a concrete type.
/// fn span_of<T: Bounded + Into<i128>>() -> i128 {
///     T::MAX.into() - T::MIN.into()
/// }
///
/// assert_eq!(span_of::<u8>(), 255);
/// assert_eq!(span_of::<i8>(), 255);
/// ```
///
/// Implemented for all primitive integer types (`u8`..`u128`, `usize`, and the
/// signed counterparts) by the macro below. Floating-point types are not
/// covered: their notion of a "smallest" value is ambiguous (`MIN` vs
/// `-INFINITY`).
pub trait Bounded {
    /// The smallest value of this type.
    const MIN: Self;
    /// The largest value of this type.
    const MAX: Self;
}

/// Implements [`Bounded`] for each primitive integer type listed, forwarding to
/// the type's inherent `MIN` and `MAX` constants.
macro_rules! impl_bounded {
    ($($t:ty)+) => {
        $(impl Bounded for $t { const MIN: Self = <$t>::MIN; const MAX: Self = <$t>::MAX; })+
    };
}

impl_bounded!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
