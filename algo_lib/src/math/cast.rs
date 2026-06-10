/// Conversion between primitive numeric types via the `as` operator.
///
/// This is the trait form of an `as` cast, letting you write conversions
/// generically (e.g. behind a type parameter) instead of hard-coding `as U`
/// at each call site.
///
/// # Semantics
///
/// Both methods perform exactly `value as Target`, so they carry the **same
/// caveats as `as`**: conversions may truncate, wrap, or lose precision and
/// never panic or report failure. Use [`TryFrom`] instead when you need
/// lossless, checked conversions.
///
/// # Use as a trait bound
///
/// Because the conversion lives in the type system rather than in the `as`
/// operator (which is not a trait and so cannot appear in a bound), `Cast`
/// lets generic code require "this type can be cast to/from `U`". This is the
/// main reason to prefer it over a bare `as`: `as` only works on concrete
/// types, whereas `Cast<U>` works on a type parameter.
///
/// ```
/// use algo_lib::math::cast::Cast;
///
/// // Accepts any primitive that can be cast to u64, sums it in u64 to avoid
/// // overflow, regardless of the element type.
/// fn sum_as_u64<T: Cast<u64>>(items: &[T]) -> u64 {
///     items.iter().map(|&x| x.to()).sum()
/// }
///
/// assert_eq!(sum_as_u64(&[1u8, 2, 3]), 6);
/// assert_eq!(sum_as_u64(&[1u32, 2, 3]), 6);
/// ```
///
/// The bound can also flow the other way (`U: Cast<T>` to build a `T` from a
/// `U`), and multiple bounds compose, e.g. `T: Cast<u64> + Cast<usize>`.
///
/// # Choosing the target type
///
/// `T` is the target type and is selected by inference. When the surrounding
/// context does not pin it down (a common case is `acc += x.to()`, where
/// `AddAssign` accepts both `U` and `&U`), name it explicitly:
///
/// ```
/// use algo_lib::math::cast::Cast;
///
/// let g: u32 = 5;
/// let mut gold: u64 = 0;
///
/// // Ambiguous: `gold += g.to();` cannot infer the target type.
/// gold += Cast::<u64>::to(g); // turbofish pins the target to u64
///
/// let x: u64 = g.to(); // a typed binding also pins it
/// assert_eq!(x, 5);
/// ```
pub trait Cast<T>: Copy {
    /// Casts `self` into `T` (equivalent to `self as T`).
    fn to(self) -> T;

    /// Builds `Self` from a `T` (equivalent to `t as Self`).
    fn from(t: T) -> Self;
}

/// Implements `Cast<$u>` for `$t` for each target type `$u`.
macro_rules! impl_cast_for_type {
    ($t: ty, $($u: ty)+) => {
        $(impl Cast<$u> for $t {
            fn to(self) -> $u {
                self as $u
            }
            fn from(t: $u) -> Self {
                t as $t
            }
        })+
    };
}

/// Implements `Cast` between every pair of the listed primitive types.
macro_rules! impl_casts {
    ($($t: ty)+) => {
        $(impl_cast_for_type!($t, u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);)+
    };
}

impl_casts!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
