//! Sorting as a chainable, by-value step.
//!
//! [`SortedExt`] turns the `let mut v = …; v.sort(); v` dance into one
//! expression, so a sort can sit inline in the pipeline that reads the input.
//! The bound is [`AsMut<[T]>`](AsMut) rather than a concrete container, so a
//! single blanket impl covers `Vec<T>`, `[T; N]`, `Box<[T]>` — and `&mut [T]`,
//! which sorts through the borrow and hands it back.
//!
//! ```
//! use algo_lib::collections::linear_ext::sorted_ext::SortedExt;
//!
//! assert_eq!(vec![3, 1, 2].sorted(), vec![1, 2, 3]);
//! assert_eq!([3, 1, 2].sorted_desc(), [3, 2, 1]);
//!
//! // Stable by default: equal keys stay in the order they were read.
//! let events = vec![(2, 'b'), (1, 'z'), (1, 'a')].sorted_by_key(|e| e.0);
//! assert_eq!(events, vec![(1, 'z'), (1, 'a'), (2, 'b')]);
//!
//! // Sort by one field, break ties with another, in one comparator.
//! let events = vec![(2, 'b'), (1, 'z'), (1, 'a')]
//!     .sorted_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
//! assert_eq!(events, vec![(1, 'a'), (1, 'z'), (2, 'b')]);
//!
//! // Through a borrow: the array itself ends up sorted.
//! let mut xs = [3, 1, 2];
//! let _ = xs.as_mut_slice().sorted();
//! assert_eq!(xs, [1, 2, 3]);
//! ```
//!
//! Sorted output is the precondition for
//! [`SortedOps`](crate::collections::slice_ext::sorted_ops::SortedOps), so
//! `.sorted()` followed by `lower_bound` is the usual pairing.
//!
//! # Stability
//!
//! Names mean what they mean in std: `sorted*` preserves the relative order of
//! elements that compare equal, `sorted_unstable*` does not.
//!
//! | stable                 | unstable                 |
//! |------------------------|--------------------------|
//! | `sorted`               | `sorted_unstable`        |
//! | `sorted_by`            | `sorted_unstable_by`     |
//! | `sorted_by_key`        | `sorted_unstable_by_key` |
//! | `sorted_by_cached_key` | *not needed*             |
//! | `sorted_desc`          | `sorted_unstable_desc`   |
//!
//! Prefer the unstable form whenever ties are indistinguishable — for a
//! `Vec<u32>` of read input they are, and it is faster and allocates nothing.
//! The stable form earns its keep when equal elements carry information the
//! comparator cannot see: "sort the events by time, equal times in the order
//! they were read".
//!
//! There is deliberately no `sorted_unstable_by_cached_key`, and the reason is
//! worth knowing: caching pairs each key with its original index, those pairs
//! are all distinct, so sorting them has no ties to break and comes out stable
//! whatever sort is used. std's `sort_by_cached_key` accordingly calls
//! `sort_unstable` internally, commenting that "the elements of `indices` are
//! unique, as they are indexed, so any sort will be stable with respect to the
//! original slice". An unstable cached-key sort would be the same algorithm at
//! the same cost — there is no trade to make.
//!
//! Never conclude a sort is stable from a small test. `sort_unstable`
//! insertion-sorts short slices, so it happens to preserve input order below
//! roughly 20 elements and then stops: with keys `i % 3` and a serial number,
//! `sorted_unstable_by_key` holds the order at 24 elements and breaks it at 64.

use std::cmp::Ordering;

/// Sorting and reversing that consume `self` and return it, for any container
/// that can hand out a `&mut [T]`.
///
/// Every method is a thin wrapper over the slice method of the same name; the
/// `_unstable` ones trade tie order for speed and no allocation, exactly as in
/// std (see the [module docs](self#stability)). Use the inherent slice methods
/// directly when you already hold a `&mut` and have nothing to chain.
///
/// [`Sized`] is a supertrait because each method returns `Self` by value; it is
/// not decoration and cannot be dropped.
pub trait SortedExt<T>: AsMut<[T]> + Sized {
    /// Sorts ascending, stably. O(n log n), allocates.
    #[must_use]
    #[inline]
    fn sorted(mut self) -> Self
    where
        T: Ord,
    {
        self.as_mut().sort();
        self
    }

    /// Sorts by a comparator, stably.
    ///
    /// `cmp` must be a total order. A contradictory one is not undefined
    /// behaviour, but leaves the order unspecified or panics.
    #[must_use]
    #[inline]
    fn sorted_by<F>(mut self, cmp: F) -> Self
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.as_mut().sort_by(cmp);
        self
    }

    /// Sorts by an extracted key, stably.
    ///
    /// The key is recomputed on every comparison, i.e. O(n log n) times. When it
    /// is more than a field read, prefer
    /// [`sorted_by_cached_key`](SortedExt::sorted_by_cached_key).
    #[must_use]
    #[inline]
    fn sorted_by_key<F, K>(mut self, key: F) -> Self
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.as_mut().sort_by_key(key);
        self
    }

    /// Sorts by an extracted key, computing each key exactly once, stably.
    ///
    /// Buys O(n) key evaluations instead of O(n log n) by allocating a table of
    /// keys and indices. Worth it when the key does real work — building a
    /// `String`, walking a list — and a loss when it is a field read, where the
    /// allocation costs more than the recomputation it saves.
    ///
    /// This one has no unstable counterpart, and needs none: the cached keys are
    /// paired with their indices, so the inner sort has no ties to break and is
    /// stable for free. See the [module docs](self#stability).
    #[must_use]
    #[inline]
    fn sorted_by_cached_key<F, K>(mut self, key: F) -> Self
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.as_mut().sort_by_cached_key(key);
        self
    }

    /// Sorts ascending without preserving the order of equal elements.
    /// O(n log n), no allocation — the default choice when ties are
    /// indistinguishable anyway.
    #[must_use]
    #[inline]
    fn sorted_unstable(mut self) -> Self
    where
        T: Ord,
    {
        self.as_mut().sort_unstable();
        self
    }

    /// Sorts by a comparator without preserving the order of equal elements.
    ///
    /// `cmp` must be a total order; see [`sorted_by`](SortedExt::sorted_by).
    #[must_use]
    #[inline]
    fn sorted_unstable_by<F>(mut self, cmp: F) -> Self
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.as_mut().sort_unstable_by(cmp);
        self
    }

    /// Sorts by an extracted key without preserving the order of equal
    /// elements. The key is recomputed on every comparison.
    #[must_use]
    #[inline]
    fn sorted_unstable_by_key<F, K>(mut self, key: F) -> Self
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.as_mut().sort_unstable_by_key(key);
        self
    }

    /// Sorts descending, stably.
    ///
    /// One pass, unlike `.sorted().reversed()` — which is two passes and, worse,
    /// flips equal elements among themselves, so it is *not* the same operation.
    #[must_use]
    #[inline]
    fn sorted_desc(mut self) -> Self
    where
        T: Ord,
    {
        self.as_mut().sort_by(|a, b| b.cmp(a));
        self
    }

    /// Sorts descending without preserving the order of equal elements.
    #[must_use]
    #[inline]
    fn sorted_unstable_desc(mut self) -> Self
    where
        T: Ord,
    {
        self.as_mut().sort_unstable_by(|a, b| b.cmp(a));
        self
    }

    /// Reverses the order of the elements. O(n), and no ordering bound on `T` —
    /// this is not a sort, just the chainable form of `reverse`.
    #[must_use]
    #[inline]
    fn reversed(mut self) -> Self {
        self.as_mut().reverse();
        self
    }
}

impl<T, C: AsMut<[T]>> SortedExt<T> for C {}
