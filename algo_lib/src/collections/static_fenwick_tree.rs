//! A Fenwick tree whose length is a const parameter and whose cells live in
//! the struct rather than on the heap.
//!
//! [`StaticFenwickTree<T, N>`] is [`FenwickTree<T>`] with the backing `Vec<T>`
//! replaced by `[T; N]`. Identical cell layout, identical bit walks, identical
//! clamping rules — the
//! [`fenwick_tree`](crate::collections::fenwick_tree) module documentation
//! explains all of it and is not repeated here. The only thing that differs is
//! where the cells live.
//!
//! # When to prefer it
//!
//! Reach for this when the domain is a compile-time constant *and* you need
//! many trees rather than one: a counter per trie node, per DP state, per
//! bucket of a sqrt decomposition. At that shape a `Vec` costs more than it
//! looks:
//!
//! - **No allocation.** One `malloc` per tree stops being noise when there are
//!   millions of trees; construction becomes a memset the compiler can often
//!   fold into the parent's own initialisation.
//! - **No indirection.** The cells sit inside the owning struct, so reaching
//!   them is an offset rather than a pointer chase into a block the prefetcher
//!   never saw.
//! - **No handle.** `Vec<T>` spends 24 bytes on pointer, length and capacity
//!   before storing a single cell; `[T; N]` spends none, and the length is
//!   known to the optimiser at every call site.
//!
//! For one tree, or a length only known at run time, use [`FenwickTree`].
//!
//! # Example
//!
//! ```
//! use algo_lib::collections::static_fenwick_tree::StaticFenwickTree;
//!
//! let mut ft = StaticFenwickTree::from([1, 2, 3, 4, 5]);
//! assert_eq!(ft.get(1..4), 9);
//! ft.add(2, 10);
//! assert_eq!(ft.get(1..4), 19);
//! assert_eq!(ft.get(..), 25);
//! ```
//!
//! [`FenwickTree`]: crate::collections::fenwick_tree::FenwickTree
//! [`FenwickTree<T>`]: crate::collections::fenwick_tree::FenwickTree

use std::ops::RangeBounds;

use crate::{
    collections::fenwick_tree::{build_parent_walk, resolve_bounds, undo_parent_walk},
    math::{algebra::AdditionMonoidWithSub, cast::Cast},
};

/// `N` values supporting point addition and range sum, each in `O(log N)`,
/// stored inline.
///
/// The heap-allocated equivalent is
/// [`FenwickTree`](crate::collections::fenwick_tree::FenwickTree), whose module
/// documentation covers the cell layout and the two bit walks behind every
/// method.
///
/// Deliberately not [`Copy`], even though `[T; N]` would allow it: at a
/// realistic `N` this is hundreds of bytes, and implicit copies in a hot loop
/// are harder to spot than an explicit [`clone`](Clone::clone).
#[derive(Clone)]
pub struct StaticFenwickTree<T, const N: usize> {
    ft: [T; N],
}

impl<T: AdditionMonoidWithSub + Copy, const N: usize> StaticFenwickTree<T, N> {
    /// Creates a tree of `N` zeros. `O(N)`.
    ///
    /// To load existing data, prefer `StaticFenwickTree::from(array)`: it
    /// builds in `O(N)`, where `N` calls to [`add`](Self::add) would cost
    /// `O(N log N)`.
    pub fn new() -> Self {
        Self { ft: [T::zero(); N] }
    }

    /// Sum over `[0, to)` — the first `to` elements, mirroring `slice[..to]`.
    ///
    /// `to` is clamped to `N`, so `get_to(0)` is zero and any `to` at or past
    /// the end gives the total. `O(log N)`.
    pub fn get_to(&self, mut to: usize) -> T {
        to = to.min(N).wrapping_sub(1);
        let mut result = T::zero();
        while let Some(t) = self.ft.get(to) {
            result += *t;
            to = (to & (to + 1)).wrapping_sub(1);
        }
        result
    }

    /// Sum over `bounds`, interpreted exactly as slice indexing is:
    /// `get(a..b)` matches `array[a..b].iter().sum()` and `get(a..=b)` includes
    /// `b`.
    ///
    /// Empty and reversed ranges give zero, and both endpoints are clamped to
    /// `N`, so out-of-range bounds saturate instead of panicking. `O(log N)`,
    /// as the difference of two prefix sums.
    pub fn get(&self, bounds: impl RangeBounds<usize>) -> T {
        let (from, to) = resolve_bounds(bounds, N);
        if from >= to {
            T::zero()
        } else {
            self.get_to(to) - self.get_to(from)
        }
    }

    /// Adds `value` to the element at `at`, walking up to every cell whose
    /// block covers it. `O(log N)`.
    ///
    /// An out-of-range `at` is ignored: the walk never enters the array, so
    /// adding past the end is a silent no-op.
    pub fn add(&mut self, mut at: usize, value: T) {
        while at < N {
            self.ft[at] += value;
            at |= at + 1;
        }
    }

    /// The original array, recovered by inverting the build's parent walk.
    ///
    /// Copies the cells; `<[T; N]>::from(tree)` consumes the tree instead.
    /// `O(N)`.
    pub fn to_array(&self) -> [T; N] {
        let mut ft = self.ft;
        undo_parent_walk(&mut ft);
        ft
    }

    /// Resets every element to zero. `O(N)`.
    pub fn clear(&mut self) {
        self.ft.fill(T::zero());
    }

    /// The number of elements the tree holds, i.e. `N`.
    ///
    /// A `const fn`, so it also serves as the length of an array declared
    /// alongside the tree.
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize {
        N
    }
}

impl<T: AdditionMonoidWithSub + Copy + Ord, const N: usize> StaticFenwickTree<T, N> {
    /// Smallest index `i` with `get_to(i + 1) >= k`, or `N` if no prefix
    /// reaches `k`.
    ///
    /// For a tree of counts this is the element holding the `k`-th unit,
    /// counting from 1 — the operation that turns a Fenwick tree into an
    /// order-statistic structure. `O(log N)`: a single descent over the powers
    /// of two, rather than a binary search calling [`get_to`](Self::get_to) at
    /// every step, which would be `O(log² N)`.
    ///
    /// Requires every element to be non-negative. The descent assumes prefix
    /// sums are non-decreasing, and returns a meaningless index otherwise.
    ///
    /// ```
    /// use algo_lib::collections::static_fenwick_tree::StaticFenwickTree;
    ///
    /// let ft = StaticFenwickTree::from([0, 1, 0, 1, 2]);
    /// assert_eq!(ft.select(1), 1); // the first unit sits at index 1
    /// assert_eq!(ft.select(3), 4);
    /// assert_eq!(ft.select(5), 5); // past the total, so `N`
    /// ```
    pub fn select(&self, mut k: T) -> usize {
        let mut p2 = N.next_power_of_two();
        let mut at = 0;
        while p2 > 0 {
            if at + p2 <= N {
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

impl<T: AdditionMonoidWithSub + Copy, const N: usize> StaticFenwickTree<T, N>
where
    usize: Cast<T>,
{
    /// A tree of `N` elements, every one of them equal to one. `O(N)`.
    ///
    /// No walk is needed: over all-ones data, cell `i` holds nothing but the
    /// length of its own block `[i & (i + 1), i]`, which is the lowest set bit
    /// of `i + 1`.
    ///
    /// This is the entry point for algorithms that start from "every key is
    /// present" and only ever delete. For a domain starting at one rather than
    /// zero, follow with `add(0, -one)`.
    pub fn ones() -> Self {
        Self {
            ft: std::array::from_fn(|i| ((i + 1) & (i + 1).wrapping_neg()).to()),
        }
    }
}

impl<T: AdditionMonoidWithSub + Copy, const N: usize> Default for StaticFenwickTree<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Builds a tree from an array in `O(N)`, using `N - 1` additions.
///
/// Prefer this to [`new`](StaticFenwickTree::new) followed by a call to
/// [`add`](StaticFenwickTree::add) per element, which costs `O(N log N)`.
impl<T: AdditionMonoidWithSub + Copy, const N: usize> From<[T; N]> for StaticFenwickTree<T, N> {
    fn from(mut ft: [T; N]) -> Self {
        build_parent_walk(&mut ft);
        Self { ft }
    }
}

/// Recovers the original array, consuming the tree and reusing its cells.
///
/// The copy-free counterpart to [`to_array`](StaticFenwickTree::to_array), and
/// the exact inverse of building from an array. `O(N)`.
impl<T: AdditionMonoidWithSub + Copy, const N: usize> From<StaticFenwickTree<T, N>> for [T; N] {
    fn from(tree: StaticFenwickTree<T, N>) -> Self {
        let mut ft = tree.ft;
        undo_parent_walk(&mut ft);
        ft
    }
}

#[cfg(test)]
mod test {
    use crate::collections::{slice_ext::indices::Indices, static_fenwick_tree::StaticFenwickTree};

    const F: [i32; 10] = [0, 1, 0, 1, 2, 3, 2, 1, 1, 0];

    fn tree() -> StaticFenwickTree<i32, 10> {
        StaticFenwickTree::from(F)
    }

    /// Sum of a half-open slice range, as the reference answer.
    fn sum(from: usize, to: usize) -> i32 {
        F[from..to].iter().sum()
    }

    #[test]
    fn matches_the_heap_backed_tree() {
        use crate::collections::fenwick_tree::FenwickTree;

        let mut a = tree();
        let mut b = FenwickTree::from(F.as_slice());
        a.add(4, 7);
        b.add(4, 7);
        for from in 0..=F.len() {
            for to in from..=F.len() {
                assert_eq!(a.get(from..to), b.get(from..to), "get({from}..{to})");
            }
        }
        assert_eq!(a.to_array().as_slice(), b.to_vec().as_slice());
    }

    #[test]
    fn round_trips_through_an_array() {
        let mut ft = tree();
        ft.add(4, 1);
        ft.add(0, -3);
        let mut want = F;
        want[4] += 1;
        want[0] -= 3;
        assert_eq!(ft.to_array(), want);
        // The consuming conversion agrees, and leaves nothing behind to copy.
        assert_eq!(<[i32; 10]>::from(ft), want);
    }

    #[test]
    fn new_and_default_are_all_zeros() {
        assert_eq!(StaticFenwickTree::<i32, 10>::new().to_array(), [0; 10]);
        assert_eq!(StaticFenwickTree::<i32, 10>::default().to_array(), [0; 10]);
    }

    #[test]
    fn clear_resets_every_element() {
        let mut ft = tree();
        ft.clear();
        assert_eq!(ft.to_array(), [0; 10]);
        assert_eq!(ft.get(..), 0);
    }

    #[test]
    fn ones_is_an_all_ones_array() {
        let ft = StaticFenwickTree::<i32, 33>::ones();
        assert_eq!(ft.to_array(), [1; 33]);
        for to in 0..=33 {
            assert_eq!(ft.get_to(to), to as i32, "ones().get_to({to})");
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
        assert_eq!(ft.select(0), 0, "select(0)");
        assert_eq!(ft.select(total + 1), F.len(), "k past the total returns N");
    }

    #[test]
    fn out_of_range_bounds_are_clamped() {
        let mut ft = tree();
        assert_eq!(ft.get(0..1000), sum(0, F.len()), "get(0..1000)");
        assert_eq!(ft.get_to(99), sum(0, F.len()), "get_to(99)");
        assert_eq!(ft.get(10..), 0, "get(10..)");
        assert_eq!(ft.get(3..3), 0, "get(3..3)");
        // An add past the end is a silent no-op, not a panic.
        ft.add(10, 100);
        ft.add(usize::MAX, 100);
        assert_eq!(ft.get(..), sum(0, F.len()));
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
        }
        assert_eq!(ft.get(..), sum(0, F.len()), "get(..)");
    }

    #[test]
    fn a_zero_length_tree_is_usable() {
        let mut ft = StaticFenwickTree::<i32, 0>::new();
        ft.add(0, 5);
        assert_eq!(ft.len(), 0);
        assert_eq!(ft.get(..), 0);
        assert_eq!(ft.get_to(3), 0);
        assert_eq!(ft.select(1), 0);
    }
}
