//! Duplicate removal as a chainable, by-value step.
//!
//! [`Dedup`] wraps [`Vec::dedup`] and `sort_unstable` + `dedup` so they can sit
//! directly in an expression instead of forcing a `let mut` and two statements.
//! The intended use is right after reading input: `sorted_deduped` is the first
//! half of coordinate compression — turn the raw values into the sorted list of
//! distinct values, then map each value to its rank with
//! [`SortedOps::lower_bound`](crate::collections::slice_ext::sorted_ops::SortedOps::lower_bound).
//!
//! ```
//! use algo_lib::collections::slice_ext::sorted_ops::SortedOps;
//! use algo_lib::collections::vec_ext::dedup::Dedup;
//!
//! // Coordinate compression: the distinct values, ascending.
//! let values = vec![50i32, 10, 50, 30, 10].sorted_deduped();
//! assert_eq!(values, vec![10, 30, 50]);
//!
//! // …then a value's rank is its position in that list.
//! assert_eq!(values.lower_bound(&30), 1);
//!
//! // `deduped` collapses only *runs* of equals, so unsorted input keeps both 50s.
//! assert_eq!(vec![50i32, 10, 50].deduped(), vec![50, 10, 50]);
//! ```
//!
//! Only `Vec` is covered: both operations shorten the container, which a slice
//! cannot do.

/// Removes duplicates and returns the shortened `Vec`.
///
/// Both methods consume `self` so they can be chained; use the inherent
/// [`Vec::dedup`] when you already have a `&mut Vec` and want to mutate it in
/// place.
///
/// The bounds are deliberately split: the implementation needs only
/// [`PartialEq`] for the run-collapsing [`deduped`](Dedup::deduped), so it stays
/// available for types with no total order (`Vec<f64>`), while
/// [`sorted_deduped`](Dedup::sorted_deduped) asks for [`Ord`] on the method
/// itself.
pub trait Dedup<T>: Sized {
    /// Removes *consecutive* repeated elements, keeping the first of each run.
    ///
    /// This is [`Vec::dedup`]: it is O(n) and does **not** sort, so equal
    /// elements that are not adjacent both survive. Reach for
    /// [`sorted_deduped`](Dedup::sorted_deduped) unless the input is already
    /// grouped, or unless collapsing runs is exactly what you want (compressing
    /// a path, squashing repeated characters).
    #[must_use]
    fn deduped(self) -> Self;

    /// Sorts ascending, then removes duplicates, leaving every distinct element
    /// exactly once in ascending order.
    ///
    /// O(n log n). The sort is unstable, which is free here: reordering equal
    /// elements is unobservable once all but one of them are dropped, and it
    /// avoids the auxiliary allocation a stable sort makes.
    #[must_use]
    fn sorted_deduped(self) -> Self
    where
        T: Ord;
}

impl<T: PartialEq> Dedup<T> for Vec<T> {
    #[inline]
    fn deduped(mut self) -> Self {
        self.dedup();
        self
    }

    #[inline]
    fn sorted_deduped(mut self) -> Self
    where
        T: Ord,
    {
        self.sort_unstable();
        self.dedup();
        self
    }
}
