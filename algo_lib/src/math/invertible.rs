/// A value that may have a multiplicative inverse.
///
/// `inv` returns `Some(x)` such that `self * x` is the multiplicative identity,
/// or `None` when no such `x` exists. The result is an `Option` rather than a
/// panic because invertibility is a property of the *value*, not the type:
/// zero is never invertible, and in a modular ring an element sharing a factor
/// with the modulus is not either.
///
/// # Associated type
///
/// [`Output`](Invertible::Output) is the type of the inverse. It is usually
/// `Self`, as for a modular integer, but is kept separate so a type can
/// invert into a different one (for example, an integer that inverts into a
/// rational).
///
/// The compiler does not check that `inv` really produces an inverse;
/// implementors are responsible for that. This trait is the inversion
/// requirement behind
/// [`MultiplicationGroup`](crate::math::algebra::MultiplicationGroup) and
/// [`Field`](crate::math::algebra::Field).
pub trait Invertible {
    /// The type of the inverse.
    type Output;

    /// Returns the multiplicative inverse of `self`, or `None` if it has none.
    fn inv(&self) -> Option<Self::Output>;
}
