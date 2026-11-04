/// A monoid: a carrier type `T` with an associative binary operation
/// `combine` and an identity element `empty`.
///
/// Implementors are usually zero-sized marker types (e.g. `struct Add;`), not
/// the carrier itself. `T` is an associated type, so each marker has exactly
/// one carrier. Data structures like segment trees can then be written as
/// `SegTree<M: Monoid>` and store `Vec<M::T>`.
///
/// # Laws
///
/// Implementations must satisfy, for all `a`, `b`, `c: M::T`:
/// - Identity: `combine(empty(), a) == a` and `combine(a, empty()) == a`
/// - Associativity: `combine(combine(a, b), c) == combine(a, combine(b, c))`
///
/// `combine` need not be commutative. Callers must preserve argument order
/// (left operand first) so non-commutative monoids stay correct.
///
/// The laws are not checked by the compiler. Violating them makes results of
/// range-query structures wrong.
pub trait Monoid {
    /// The carrier type. `Copy` so values can be combined without cloning.
    type T: Copy;

    /// The identity element: `combine(empty(), a) == a == combine(a, empty())`.
    fn empty() -> Self::T;

    /// The associative binary operation.
    fn combine(a: Self::T, b: Self::T) -> Self::T;
}
