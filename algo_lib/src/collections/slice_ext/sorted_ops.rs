use std::ops::{Bound, RangeBounds};

/// Binary-search queries over a slice that is assumed to be sorted in
/// ascending order.
///
/// Every method runs in O(log n) time. Results are unspecified if the slice is
/// not sorted.
pub trait SortedOps<T: PartialOrd> {
    /// Returns the index of the first element `>= elem`, i.e. the number of
    /// elements strictly less than `elem`. This is the insertion point that
    /// keeps the slice sorted while placing `elem` before any equal elements.
    fn lower_bound(&self, elem: &T) -> usize;

    /// Returns the index of the first element `> elem`, i.e. the number of
    /// elements less than or equal to `elem`. This is the insertion point that
    /// keeps the slice sorted while placing `elem` after any equal elements.
    fn upper_bound(&self, elem: &T) -> usize;

    /// Returns the index of some element equal to `elem`, or `None` if no such
    /// element exists. When duplicates are present, the leftmost match is
    /// returned.
    fn binary_find(&self, elem: &T) -> Option<usize>;

    /// Returns the number of elements strictly greater than `elem`.
    fn more_count(&self, elem: &T) -> usize;

    /// Returns the number of elements strictly less than `elem`.
    fn less_count(&self, elem: &T) -> usize {
        self.lower_bound(elem)
    }

    /// Returns the number of elements greater than or equal to `elem`.
    fn more_or_eq_count(&self, elem: &T) -> usize;

    /// Returns the number of elements less than or equal to `elem`.
    fn less_or_eq_count(&self, elem: &T) -> usize {
        self.upper_bound(elem)
    }

    /// Returns the number of elements that fall within `bounds`, honoring each
    /// end's inclusive/exclusive bound.
    fn in_range_count<'a>(&self, bounds: impl RangeBounds<&'a T>) -> usize
    where
        T: 'a;
}

impl<T: PartialOrd> SortedOps<T> for [T] {
    fn lower_bound(&self, elem: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left < right {
            let at = left + ((right - left) >> 1);
            if &self[at] < elem {
                left = at + 1;
            } else {
                right = at;
            }
        }
        left
    }

    fn upper_bound(&self, elem: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left < right {
            let at = left + ((right - left) >> 1);
            if &self[at] <= elem {
                left = at + 1;
            } else {
                right = at;
            }
        }
        left
    }

    fn binary_find(&self, elem: &T) -> Option<usize> {
        let at = self.lower_bound(elem);
        (at != self.len() && &self[at] == elem).then_some(at)
    }

    fn more_count(&self, elem: &T) -> usize {
        self.len() - self.upper_bound(elem)
    }

    fn more_or_eq_count(&self, elem: &T) -> usize {
        self.len() - self.lower_bound(elem)
    }

    fn in_range_count<'a>(&self, bounds: impl RangeBounds<&'a T>) -> usize
    where
        T: 'a,
    {
        let start = match bounds.start_bound() {
            Bound::Included(elem) => self.lower_bound(elem),
            Bound::Excluded(elem) => self.upper_bound(elem),
            Bound::Unbounded => 0,
        };
        let end = match bounds.end_bound() {
            Bound::Included(elem) => self.upper_bound(elem),
            Bound::Excluded(elem) => self.lower_bound(elem),
            Bound::Unbounded => self.len(),
        };
        end - start
    }
}
