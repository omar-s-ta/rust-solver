//! A Fenwick tree (binary indexed tree): point updates and range sums, both
//! in `O(log n)`.
//!
//! [`FenwickTree<T>`] keeps `n` values in a single flat [`Vec<T>`] — no
//! pointers, no padding to a power of two — and answers "add to one element"
//! and "sum a range" in logarithmic time. Every method is a short `while` loop
//! driven by one of two bit walks.
//!
//! # Layout
//!
//! Indices are 0-based. Cell `i` holds the sum of the original elements over
//! the inclusive block `[i & (i + 1), i]`, whose length is `2` to the power of
//! the number of trailing one bits in `i`. So cell `6` (`0b110`) covers just
//! `[6, 6]`, cell `5` (`0b101`) covers `[4, 5]`, and cell `3` (`0b11`) covers
//! `[0, 3]`.
//!
//! Two walks follow from that:
//!
//! - **Up**, over the cells whose block contains element `i`: `i |= i + 1`.
//!   This is the update path, used by [`add`](FenwickTree::add).
//! - **Down**, over the disjoint blocks that tile a prefix ending at `i`:
//!   `i = (i & (i + 1)) - 1`. This is the query path, used by
//!   [`get_to`](FenwickTree::get_to).
//!
//! Each walk clears or sets one bit per step, so both visit at most
//! `log2(n)` cells. The subtraction in the query walk underflows past the
//! first block, which is exactly the termination signal: `usize::MAX` is out
//! of bounds, so the loop's `get` returns `None`.
//!
//! # Conventions
//!
//! Ranges follow the same rules as slice indexing, so `get(a..b)` equals
//! `array[a..b].iter().sum()` and `get_to(i)` equals `array[..i].iter().sum()`.
//! Out-of-range bounds are clamped rather than panicking, and an `add` outside
//! the array is a no-op.
//!
//! # Why this instead of a segment tree?
//!
//! A segment tree does everything this does and more, so the case for a
//! Fenwick tree is entirely about cost:
//!
//! - **`n` cells, not `2n` to `4n`.** One allocation, no explicit tree, and
//!   the whole structure is a contiguous array that stays cache-resident for
//!   sizes where a segment tree would not.
//! - **A far smaller constant.** A query is a handful of loads, an `and`, and
//!   an add per level, with no recursion, no child-index arithmetic, and no
//!   branches on node boundaries.
//! - **Little code to get wrong.** The tricky parts are two bit expressions.
//!
//! The price is that range queries are computed as a *difference* of two
//! prefixes, which requires the operation to be invertible — hence the
//! [`AdditionMonoidWithSub`] bound. Operations without an inverse, such as
//! `min` or `max`, cannot be answered this way and need a segment tree or a
//! sparse table.
//!
//! # Example
//!
//! ```
//! use algo_lib::collections::fenwick_tree::FenwickTree;
//!
//! let mut ft = FenwickTree::from([1, 2, 3, 4, 5].as_slice());
//! assert_eq!(ft.get(1..4), 9);
//! ft.add(2, 10);
//! assert_eq!(ft.get(1..4), 19);
//! assert_eq!(ft.get(..), 25);
//! ```

use std::ops::RangeBounds;

use crate::{
    collections::slice_ext::indices::Indices,
    math::{algebra::AdditionMonoidWithSub, cast::Cast},
};

/// An array of values supporting point addition and range sum, each in
/// `O(log n)`.
///
/// See the [module documentation](self) for the cell layout and the two bit
/// walks that drive every method.
#[derive(Clone)]
pub struct FenwickTree<T> {
    ft: Vec<T>,
}

impl<T: AdditionMonoidWithSub + Copy> FenwickTree<T> {
    /// Creates a tree of `n` zeros. `O(n)`.
    ///
    /// To load existing data, prefer `FenwickTree::from(slice)`: it builds in
    /// `O(n)`, where `n` calls to [`add`](Self::add) would cost `O(n log n)`.
    pub fn new(n: usize) -> Self {
        Self {
            ft: vec![T::zero(); n],
        }
    }

    /// Sum over `[0, to)` — the first `to` elements, mirroring `slice[..to]`.
    ///
    /// `to` is clamped to [`len`](Self::len), so `get_to(0)` is zero and any
    /// `to` at or past the end gives the total. `O(log n)`.
    pub fn get_to(&self, mut to: usize) -> T {
        to = to.min(self.len()).wrapping_sub(1);
        let mut result = T::zero();
        while let Some(v) = self.ft.get(to) {
            result += *v;
            to = (to & (to + 1)).wrapping_sub(1);
        }
        result
    }

    /// Sum over `bounds`, interpreted exactly as slice indexing is: `get(a..b)`
    /// matches `array[a..b].iter().sum()` and `get(a..=b)` includes `b`.
    ///
    /// Empty and reversed ranges give zero, and both endpoints are clamped to
    /// [`len`](Self::len), so out-of-range bounds saturate instead of
    /// panicking. `O(log n)`, as the difference of two prefix sums.
    pub fn get(&self, bounds: impl RangeBounds<usize>) -> T {
        let (from, to) = resolve_bounds(bounds, self.len());
        if from >= to {
            T::zero()
        } else {
            self.get_to(to) - self.get_to(from)
        }
    }

    /// Adds `value` to the element at `at`, walking up to every cell whose
    /// block covers it. `O(log n)`.
    ///
    /// An out-of-range `at` is ignored: the walk stops at the first cell
    /// outside the array, so adding past the end is a silent no-op.
    pub fn add(&mut self, mut at: usize, value: T) {
        while let Some(v) = self.ft.get_mut(at) {
            *v += value;
            at |= at + 1;
        }
    }

    /// The original array, recovered by inverting the build's parent walk.
    ///
    /// Allocates a copy of the tree; `Vec::from(tree)` reuses its storage
    /// instead. `O(n)`.
    pub fn to_vec(&self) -> Vec<T> {
        undo_parent_walk(self.ft.clone())
    }

    /// Resets every element to zero, keeping the length. `O(n)`.
    pub fn clear(&mut self) {
        self.ft.fill(T::zero());
    }

    /// The number of elements the tree holds.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.ft.len()
    }
}

impl<T: AdditionMonoidWithSub + Copy + Ord> FenwickTree<T> {
    /// Smallest index `i` with `get_to(i + 1) >= k`, or [`len`](Self::len) if
    /// no prefix reaches `k`.
    ///
    /// For a tree of counts this is the element holding the `k`-th unit,
    /// counting from 1 — the operation that turns a Fenwick tree into an
    /// order-statistic structure. `O(log n)`: a single descent over the powers
    /// of two, rather than a binary search calling
    /// [`get_to`](Self::get_to) at every step, which would be `O(log² n)`.
    ///
    /// Requires every element to be non-negative. The descent assumes prefix
    /// sums are non-decreasing, and returns a meaningless index otherwise.
    ///
    /// ```
    /// use algo_lib::collections::fenwick_tree::FenwickTree;
    ///
    /// let ft = FenwickTree::from([0, 1, 0, 1, 2].as_slice());
    /// assert_eq!(ft.select(1), 1); // the first unit sits at index 1
    /// assert_eq!(ft.select(3), 4);
    /// assert_eq!(ft.select(5), 5); // past the total, so `len()`
    /// ```
    pub fn select(&self, mut k: T) -> usize {
        let size = self.len();
        let mut p2 = size.next_power_of_two();
        let mut at = 0;
        while p2 > 0 {
            if at + p2 <= size {
                let c = self.ft[at + p2 - 1];
                if c < k {
                    k -= c;
                    at += p2;
                }
            }
            p2 >>= 1;
        }
        at
    }
}

impl<T: AdditionMonoidWithSub + Copy> FenwickTree<T>
where
    usize: Cast<T>,
{
    /// A tree of `n` elements, every one of them equal to one. `O(n)`.
    ///
    /// Equivalent to building from `vec![T::one(); n]`, but without
    /// materialising that vector — worth a separate constructor because the
    /// temporary is as large as the tree itself, so a domain of `2e6` would
    /// peak at twice the memory with both live at once.
    ///
    /// No walk is needed: over all-ones data, cell `i` holds nothing but the
    /// length of its own block `[i & (i + 1), i]`, which is the lowest set bit
    /// of `i + 1`.
    ///
    /// This is the entry point for algorithms that start from "every key is
    /// present" and only ever delete — the Josephus and lucky-number families
    /// of sieves, where the survivors fall out of repeatedly removing the
    /// `k`-th remaining element. For a domain starting at one rather than
    /// zero, follow with `add(0, -one)`.
    pub fn ones(n: usize) -> Self {
        Self {
            ft: (0..n)
                .map(|i| ((i + 1) & (i + 1).wrapping_neg()).to())
                .collect(),
        }
    }
}

/// Resolves any [`RangeBounds`] into the half-open pair `(from, to)`, taking
/// `len` as the end of an unbounded range.
///
/// Shared with the 2D tree so both read a range the same way: `a..b` gives
/// `(a, b)`, `a..=b` gives `(a, b + 1)`, and `..` gives `(0, len)`. The pair is
/// deliberately *not* clamped — callers spot an empty range with `from >= to`
/// and leave anything past the end to the walks, which stop at the array
/// boundary anyway.
pub(crate) fn resolve_bounds(bounds: impl RangeBounds<usize>, len: usize) -> (usize, usize) {
    let from = match bounds.start_bound() {
        std::ops::Bound::Included(&x) => x,
        std::ops::Bound::Excluded(&x) => x + 1,
        std::ops::Bound::Unbounded => 0,
    };
    let to = match bounds.end_bound() {
        std::ops::Bound::Included(&x) => x.saturating_add(1),
        std::ops::Bound::Excluded(&x) => x,
        std::ops::Bound::Unbounded => len,
    };
    (from, to)
}

/// Inverse of the bottom-up build in `From<&[T]> for FenwickTree<T>`: the same
/// walk into each cell's parent, run right to left and subtracting instead of
/// adding, which restores every cell to its original element.
///
/// The reversed order is what makes this exact. A cell is only ever
/// decremented by children at smaller indices, which are visited later, so at
/// the moment cell `i` is read it still holds the block sum that the forward
/// pass added to its parent — each subtraction undoes exactly one addition.
fn undo_parent_walk<T: AdditionMonoidWithSub + Copy>(mut ft: Vec<T>) -> Vec<T> {
    for i in ft.indices().rev() {
        let p = i | (i + 1);
        if p < ft.len() {
            let t = ft[i];
            ft[p] -= t;
        }
    }
    ft
}

/// Recovers the original array, consuming the tree and reusing its storage.
///
/// The allocation-free counterpart to [`to_vec`](FenwickTree::to_vec), and the
/// exact inverse of building from a slice. `O(n)`.
impl<T: AdditionMonoidWithSub + Copy> From<FenwickTree<T>> for Vec<T> {
    fn from(tree: FenwickTree<T>) -> Self {
        undo_parent_walk(tree.ft)
    }
}

/// Builds a tree from an array in `O(n)`, using `n - 1` additions.
///
/// Each cell starts as its own element and is then folded into its parent, so
/// one left-to-right pass leaves every cell holding its block's sum. Prefer
/// this to [`new`](FenwickTree::new) followed by a call to
/// [`add`](FenwickTree::add) per element, which costs `O(n log n)`.
impl<T: AdditionMonoidWithSub + Copy> From<&[T]> for FenwickTree<T> {
    fn from(slice: &[T]) -> Self {
        let n = slice.len();
        let mut ft = vec![T::zero(); n];
        for i in slice.indices() {
            ft[i] += slice[i];
            let p = i | (i + 1);
            if p < n {
                let t = ft[i];
                ft[p] += t;
            }
        }
        Self { ft }
    }
}

#[cfg(test)]
mod test {
    use crate::collections::{fenwick_tree::FenwickTree, slice_ext::indices::Indices};

    const F: [i32; 10] = [0, 1, 0, 1, 2, 3, 2, 1, 1, 0];

    fn tree() -> FenwickTree<i32> {
        FenwickTree::from(F.as_slice())
    }

    /// Sum of a half-open slice range, as the reference answer.
    fn sum(from: usize, to: usize) -> i32 {
        F[from..to].iter().sum()
    }

    #[test]
    fn fenwick_tree_get() {
        let mut ft = tree();
        assert_eq!(ft.get(0..=5), 7);
        assert_eq!(ft.select(7), 5);
        ft.add(4, 1);
        assert_eq!(ft.get(0..=9), 12);
    }

    #[test]
    fn fenwick_tree_rev() {
        let ft = tree();
        assert_eq!(ft.to_vec(), F);
    }

    #[test]
    fn to_vec_reflects_updates() {
        let mut ft = tree();
        ft.add(4, 1);
        ft.add(0, -3);
        let mut want = F;
        want[4] += 1;
        want[0] -= 3;
        assert_eq!(ft.to_vec(), want);
        // The consuming conversion agrees, and leaves nothing behind to clone.
        assert_eq!(Vec::from(ft), want);
    }

    #[test]
    fn ones_is_an_all_ones_array() {
        for n in 0..=33usize {
            let ft = FenwickTree::<i32>::ones(n);
            assert_eq!(ft.to_vec(), vec![1; n], "ones({n})");
            for to in 0..=n {
                assert_eq!(ft.get_to(to), to as i32, "ones({n}).get_to({to})");
            }
        }
    }

    #[test]
    fn get_to_is_exclusive() {
        let ft = tree();
        for i in F.indices() {
            assert_eq!(ft.get_to(i), sum(0, i), "get_to({i})");
        }
    }

    #[test]
    fn select_finds_the_element_holding_the_kth_unit() {
        let ft = tree();
        let total = sum(0, F.len());
        for k in 1..=total {
            let at = ft.select(k);
            // `at` is the unique index whose element spans the k-th unit.
            assert!(
                ft.get_to(at) < k,
                "select({k}) = {at}: prefix before it is not < k"
            );
            assert!(
                k <= ft.get_to(at + 1),
                "select({k}) = {at}: prefix through it is < k"
            );
            assert!(F[at] > 0, "select({k}) = {at}: landed on an empty slot");
        }
    }

    #[test]
    fn select_edge_cases() {
        let ft = tree();
        let total = sum(0, F.len());
        assert_eq!(ft.select(0), 0, "select(0)");
        assert_eq!(ft.select(1), 1, "select(1) skips the leading zero");
        assert_eq!(
            ft.select(total),
            8,
            "select(total) is the last non-empty slot"
        );
        assert_eq!(
            ft.select(total + 1),
            F.len(),
            "k past the total returns len()"
        );
    }

    #[test]
    fn single_element_ranges() {
        let ft = tree();
        for i in F.indices() {
            assert_eq!(ft.get(i..=i), F[i], "get({i}..={i})");
            assert_eq!(ft.get(i..i + 1), F[i], "get({i}..{})", i + 1);
        }
    }

    #[test]
    fn empty_ranges() {
        let ft = tree();
        assert_eq!(ft.get(3..3), 0, "get(3..3)");
        assert_eq!(ft.get(0..0), 0, "get(0..0)");
        assert_eq!(ft.get(..0), 0, "get(..0)");
    }

    #[test]
    fn end_past_the_array_is_clamped() {
        let ft = tree();
        assert_eq!(ft.get(0..1000), sum(0, F.len()), "get(0..1000)");
        assert_eq!(ft.get(1..=99), sum(1, F.len()), "get(1..=99)");
        assert_eq!(ft.get_to(99), sum(0, F.len()), "get_to(99)");
    }

    #[test]
    fn start_past_the_array_is_empty() {
        let ft = tree();
        assert_eq!(ft.get(10..), 0, "get(10..)");
        assert_eq!(ft.get(10..=20), 0, "get(10..=20)");
    }

    #[test]
    fn every_in_bounds_range_matches_a_slice_sum() {
        let ft = tree();
        for from in 0..=F.len() {
            for to in from..=F.len() {
                assert_eq!(ft.get(from..to), sum(from, to), "get({from}..{to})");
                if to < F.len() {
                    assert_eq!(ft.get(from..=to), sum(from, to + 1), "get({from}..={to})");
                }
            }
            assert_eq!(ft.get(from..), sum(from, F.len()), "get({from}..)");
            assert_eq!(ft.get(..from), sum(0, from), "get(..{from})");
        }
        assert_eq!(ft.get(..), sum(0, F.len()), "get(..)");
    }
}
