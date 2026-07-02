//! Bulk `+1` / `-1` for the classic 1-indexed ↔ 0-indexed conversion.
//!
//! Competitive-programming input is almost always 1-indexed (vertices, array
//! positions, queries), while Rust containers are 0-indexed. [`IncDec`] makes
//! the `-1` adjustment a single call that reaches through the whole shape of a
//! value: decrementing a `Vec<(usize, usize)>` of read edges subtracts one from
//! every endpoint of every pair at once.
//!
//! ```
//! use algo_lib::collections::vec_ext::inc_dec::IncDec;
//!
//! // Two 1-indexed edges read from input, converted to 0-indexed in one call.
//! let edges = vec![(1usize, 2usize), (3, 1)].dec();
//! assert_eq!(edges, vec![(0, 1), (2, 0)]);
//! ```
//!
//! The trait is blanket-implemented for any numeric type (via
//! [`AdditionMonoidWithSub`] + [`One`]) and then recursively for slices
//! (`[T]`), [`Vec`], and fixed-size arrays (`[T; N]`), which step every element.
//! Tuples (arity 2–5) are instead treated as records: only the first two fields
//! — the coordinates — are shifted, while any trailing fields ride along as an
//! untouched payload (capacity, cost, char, weight, …). So a
//! `Vec<(usize, usize, i64)>` of weighted edges converts both endpoints and
//! leaves the weight alone.

use crate::math::algebra::{AdditionMonoidWithSub, One};

/// Adds or subtracts one, in place or by value.
///
/// `inc`/`dec` consume `self` and return it (handy for chaining right after a
/// read), while `inc_mut`/`dec_mut` mutate in place. Only the in-place methods
/// are available for unsized types such as slices, since the by-value ones
/// require `Self: Sized`.
pub trait IncDec {
    /// Adds one to `self` in place.
    fn inc_mut(&mut self);
    /// Subtracts one from `self` in place.
    fn dec_mut(&mut self);

    /// Returns `self` incremented by one.
    #[must_use]
    fn inc(mut self) -> Self
    where
        Self: Sized,
    {
        self.inc_mut();
        self
    }

    /// Returns `self` decremented by one.
    #[must_use]
    fn dec(mut self) -> Self
    where
        Self: Sized,
    {
        self.dec_mut();
        self
    }
}

/// Base case: any additive numeric type steps by its own [`One`].
impl<T: AdditionMonoidWithSub + One> IncDec for T {
    fn inc_mut(&mut self) {
        *self += T::one()
    }

    fn dec_mut(&mut self) {
        *self -= T::one()
    }
}

/// Steps every element in place, so a borrowed slice of indices shifts at once.
impl<T: IncDec> IncDec for [T] {
    fn inc_mut(&mut self) {
        self.iter_mut().for_each(T::inc_mut);
    }

    fn dec_mut(&mut self) {
        self.iter_mut().for_each(T::dec_mut);
    }
}

/// Applies the step to every element, so a `Vec` of indices shifts as a whole.
impl<T: IncDec> IncDec for Vec<T> {
    fn inc_mut(&mut self) {
        self.iter_mut().for_each(T::inc_mut);
    }

    fn dec_mut(&mut self) {
        self.iter_mut().for_each(T::dec_mut);
    }
}

/// Steps every element of a fixed-size array, e.g. an edge stored as `[u; 2]`.
impl<T: IncDec, const N: usize> IncDec for [T; N] {
    fn inc_mut(&mut self) {
        self.iter_mut().for_each(T::inc_mut);
    }

    fn dec_mut(&mut self) {
        self.iter_mut().for_each(T::dec_mut);
    }
}

/// Implements [`IncDec`] for tuples treated as records: the **first two fields
/// are coordinates** and are shifted, while any remaining fields are an
/// untouched payload (edge capacity, cost, char, weight, …).
///
/// Only `.0` and `.1` are inc/dec'd; fields from `.2` onward are left as-is and
/// carry **no** `IncDec`/numeric bound — so `(usize, usize, char)` and
/// `(usize, usize, i64)` both work, and payloads are never corrupted.
///
/// Do not extend this to recurse into every field: decrementing a flow capacity
/// or cost is a silent bug, and it would reject valid non-numeric payloads.
macro_rules! tuple_inc_dec_impl {
    ($U: tt $V: tt $($tail: tt)*) => {
        impl<$U: IncDec, $V: IncDec, $($tail,)*> IncDec for ($U, $V, $($tail,)*) {
            fn inc_mut(&mut self) {
                self.0.inc_mut();
                self.1.inc_mut();
            }

            fn dec_mut(&mut self) {
                self.0.dec_mut();
                self.1.dec_mut();
            }
        }
    };
}

tuple_inc_dec_impl!(U V);
tuple_inc_dec_impl!(T U V);
tuple_inc_dec_impl!(T U V W);
tuple_inc_dec_impl!(T U V W X);
