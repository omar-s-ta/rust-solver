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
//! [`AdditionMonoidWithSub`] + [`One`]) and then recursively for [`Vec`] and
//! 2-tuples, so it applies element-wise to nested structures automatically.

use crate::math::algebra::{AdditionMonoidWithSub, One};

/// Adds or subtracts one, in place or by value.
///
/// `inc`/`dec` consume `self` and return it (handy for chaining right after a
/// read), while `inc_mut`/`dec_mut` mutate in place.
pub trait IncDec: Sized {
    /// Adds one to `self` in place.
    fn inc_mut(&mut self);
    /// Subtracts one from `self` in place.
    fn dec_mut(&mut self);

    /// Returns `self` incremented by one.
    #[must_use]
    fn inc(mut self) -> Self {
        self.inc_mut();
        self
    }

    /// Returns `self` decremented by one.
    #[must_use]
    fn dec(mut self) -> Self {
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

/// Applies the step to every element, so a `Vec` of indices shifts as a whole.
impl<T: IncDec> IncDec for Vec<T> {
    fn inc_mut(&mut self) {
        self.iter_mut().for_each(T::inc_mut);
    }

    fn dec_mut(&mut self) {
        self.iter_mut().for_each(T::dec_mut);
    }
}

/// Steps both components, e.g. both endpoints of an edge pair.
impl<U: IncDec, V: IncDec> IncDec for (U, V) {
    fn inc_mut(&mut self) {
        self.0.inc_mut();
        self.1.inc_mut();
    }

    fn dec_mut(&mut self) {
        self.0.dec_mut();
        self.1.dec_mut();
    }
}

/// Steps all three components, e.g. a weighted edge `(u, v, w)`.
impl<T: IncDec, U: IncDec, V: IncDec> IncDec for (T, U, V) {
    fn inc_mut(&mut self) {
        self.0.inc_mut();
        self.1.inc_mut();
        self.2.inc_mut();
    }

    fn dec_mut(&mut self) {
        self.0.dec_mut();
        self.1.dec_mut();
        self.2.dec_mut();
    }
}
