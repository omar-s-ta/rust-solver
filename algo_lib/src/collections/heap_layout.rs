//! Where each element of a segment tree lives in the tree.
//!
//! A segment tree over `n` elements is stored like a heap (see
//! [`heap_point`](crate::collections::heap_point)): nodes `1..2n` in one
//! array. Nodes `n..2n` are the *leaves*, and each leaf holds one element.
//! The other nodes, `1..n`, each combine their two children.
//!
//! A **layout** answers two questions:
//!
//! - Element `i` lives in which leaf? ([`point`](HeapLayout::point))
//! - Node `p` is in charge of which elements? ([`node_bounds`](HeapLayout::node_bounds))
//!
//! # Why this needs thought
//!
//! When `n` is a power of two, the leaves fill the bottom row, and both
//! questions are easy. Otherwise the leaves are spread over the bottom two
//! rows. Here is `n = 5`, leaves `5..10`:
//!
//! ```text
//!                     1
//!                  /     \
//!                2         3
//!               / \       / \
//!              4   5     6   7
//!             / \
//!            8   9
//!
//! the leaves, as you read the tree from left to right:
//!
//!            8   9   5     6   7
//! ```
//!
//! The two layouts here put the elements into those leaves in two different
//! ways.
//!
//! # The two layouts
//!
//! ```text
//! leaf (left to right):   8   9   5   6   7
//! InOrderLayout element:  0   1   2   3   4    <- in order
//! CircularLayout element: 3   4   0   1   2    <- in order, but starts in the middle
//! ```
//!
//! - [`InOrderLayout`]: reading the leaves left to right gives the elements
//!   in order. So every node is in charge of a plain run like `1..4`.
//! - [`CircularLayout`]: element `i` is simply leaf `n + i`. The math is the
//!   simplest, but a node's run can go off the right end and continue from `0`,
//!   like a clock: node `2` above is in charge of `3, 4, 0`.
//!
//! # Which one to use
//!
//! - **Only going up the tree?** Use [`CircularLayout`]. Point updates and
//!   range queries only climb from leaves to the root, so they never care
//!   where a node's run starts. It is also the cheapest: `point` and `range`
//!   are just `+ n`.
//! - **Walking down the tree in order?** Use [`InOrderLayout`]. Searches like
//!   "first position where the prefix sum reaches `k`" (`max_right` /
//!   `min_left`) go left or right at each node, and count on this rule:
//!   the left child holds the smaller elements. In [`CircularLayout`] that
//!   rule breaks: node `2`'s left child holds `3, 4`, and its right child
//!   holds `0`.
//! - **Lazy tags that need a node's size** (like "add `x` to every element",
//!   which adds `x * size` to a sum): get it from
//!   [`node_size`](HeapLayout::node_size), not from `end - start`.
//!   In [`CircularLayout`] a run can wrap, and then `end - start` is wrong:
//!   for `(3, 1)` with `n = 5` the size is `3`, not `1 - 3`. (A
//!   [`SegmentTree`](crate::collections::segment_tree::SegmentTree) does this
//!   for you: it passes `node_size` to the tag as `size`.)
//!
//! # Example
//!
//! ```
//! use algo_lib::collections::heap_layout::{CircularLayout, HeapLayout, InOrderLayout};
//! use algo_lib::collections::heap_point::HeapPoint;
//!
//! let node = |i| HeapPoint::new(i).unwrap();
//!
//! let in_order = InOrderLayout::new(5);
//! assert_eq!(in_order.point(2), node(5));
//! assert_eq!(in_order.node_bounds(node(2)), (0, 3)); // elements 0, 1, 2
//!
//! let circular = CircularLayout::new(5);
//! assert_eq!(circular.point(2), node(7));
//! assert_eq!(circular.node_bounds(node(2)), (3, 1)); // elements 3, 4, then 0
//! ```
//!
//! # Limits
//!
//! `n` is at most `2^30`. That keeps every index the layouts compute inside
//! a `u32`.

use std::ops::Range;

use crate::collections::{heap_point::HeapPoint, heap_range::HeapRange};

/// Maps elements `0..n` to the leaves of a heap-shaped tree, and back; see
/// the [module docs](self).
///
/// All runs of elements are half-open: `[start, end)` means `start` up to,
/// but not including, `end`.
pub trait HeapLayout {
    /// A layout for `n` elements.
    ///
    /// # Panics
    ///
    /// If `n > 2^30`.
    fn new(n: usize) -> Self
    where
        Self: Sized;

    /// The number of elements, `n`.
    fn len(&self) -> usize;

    /// The leaf that holds element `i`.
    ///
    /// # Panics
    ///
    /// If `i >= n`.
    fn point(&self, i: usize) -> HeapPoint;

    /// The tree range for elements `r`, ready for
    /// [`HeapRange::cover`] and the `boundary_*` walks.
    ///
    /// # Panics
    ///
    /// If `r` is not inside `0..n`.
    fn range(&self, r: Range<usize>) -> HeapRange;

    /// The element held by leaf `p`. The opposite of [`point`](Self::point).
    ///
    /// # Panics
    ///
    /// If `p` is not a leaf, that is, not in `n..2n`.
    fn leaf_index(&self, p: HeapPoint) -> usize;

    /// The elements node `p` is in charge of, as `(start, end)`.
    ///
    /// In [`InOrderLayout`] this is always a plain run, `start < end`.
    /// In [`CircularLayout`] the run may go past the last element and
    /// continue from `0`; then `start >= end`.
    ///
    /// # Panics
    ///
    /// If `p` is not a node, that is, not in `1..2n`.
    fn node_bounds(&self, p: HeapPoint) -> (usize, usize);

    /// The first element of `p`'s right child: where `p`'s run splits in
    /// two.
    ///
    /// In [`CircularLayout`] this can be `n`, which means the right child
    /// starts at element `0`.
    ///
    /// Debug builds panic if `p` is a leaf, since a leaf has no children.
    fn node_split(&self, p: HeapPoint) -> usize {
        debug_assert!(p.i() < self.len(), "node_split of a leaf");
        self.node_bounds(p.left()).1
    }

    /// How many elements node `p` is in charge of.
    fn node_size(&self, p: HeapPoint) -> usize {
        let (l, r) = self.node_bounds(p);
        r - l
    }

    /// `true` if there are no elements.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A [`HeapLayout`] in which every node is in charge of a plain run
/// `start < end`, and its left child holds the smaller elements.
///
/// Marker for algorithms that walk down the tree in element order, such as
/// a segment tree's `max_right` / `min_left`.
pub trait InOrder: HeapLayout {}

impl InOrder for InOrderLayout {}

/// Reading the leaves from left to right gives elements `0, 1, 2, ...` in
/// order.
///
/// # How it works
///
/// Let `s` be `n` rounded up to a power of two. Think of a full bottom row
/// of `s` spots, nodes `s..2s`. With `n = 5`, `s = 8`:
///
/// ```text
///              4   5     6   7
///             / \ / \   / \ / \
/// spot:      8  9 10 11 12 13 14 15
/// element:   0  1  2  3  4
///                  \_______/
///                  spots 10.. don't exist (they are >= 2n)
/// ```
///
/// Element `i` wants spot `s + i`. The first few elements get real spots
/// (`8, 9`). The others would land on spots that don't exist, so they move
/// up one row to the nodes right after: element `2` goes to `5`, `3` to `6`,
/// `4` to `7`. In numbers, that is `s + i - n`.
///
/// ```text
///                     1
///                  /     \
///                2         3
///               / \       / \
///              4  [5]   [6] [7]       <- elements 2, 3, 4
///             / \
///           [8] [9]                   <- elements 0, 1
/// ```
///
/// The other methods undo or extend the same trick: anything that falls on
/// a spot that doesn't exist moves up a row.
#[derive(Clone, Copy, Debug, Default)]
pub struct InOrderLayout {
    /// The number of elements.
    n: u32,
    /// `n` rounded up to a power of two (`0` when `n` is `0`): the bottom
    /// row is nodes `s..2s`.
    s: u32,
}

impl HeapLayout for InOrderLayout {
    fn new(n: usize) -> Self {
        let n = u32::try_from(n)
            .ok()
            .filter(|&n| n <= 1 << 30)
            .expect("InOrderLayout: n must be at most 2^30");
        Self {
            n,
            s: if n == 0 { 0 } else { n.next_power_of_two() },
        }
    }

    fn len(&self) -> usize {
        self.n as usize
    }

    /// Spot `s + i`, moved up a row if that spot doesn't exist (`- n`).
    fn point(&self, i: usize) -> HeapPoint {
        assert!(i < self.len());
        let a = i as u32 + self.s;
        HeapPoint::new(if a >= 2 * self.n { a - self.n } else { a }).unwrap()
    }

    /// Both ends are turned into spots, like [`point`](Self::point) does.
    ///
    /// A [`HeapRange`] needs both ends on one row, so an end that would sit
    /// one row up is written as the spot below it instead (`* 2`). That spot
    /// doesn't exist, but that is fine: the cover walk climbs straight past
    /// it. With `n = 5`, elements `1..4` become `[9, 14)`:
    ///
    /// ```text
    /// spot:      8  9 10 11 12 13 14 15
    /// element:   0  1  2  3  4
    ///              [9 ...........)14
    ///                              ^ end: element 4 sits up at node 7,
    ///                                the spot below 7 is 14
    /// ```
    fn range(&self, r: Range<usize>) -> HeapRange {
        assert!(r.start <= r.end && r.end <= self.len());
        if self.is_empty() {
            HeapRange::default()
        } else {
            let fold = |x: usize| {
                let x = x as u32 + self.s;
                if x >= 2 * self.n { 2 * (x - self.n) } else { x }
            };
            HeapRange::new(fold(r.start), fold(r.end))
        }
    }

    /// The opposite of [`point`](Self::point): a leaf in the bottom row
    /// (`p >= s`) is element `p - s`; a leaf one row up is element
    /// `p + n - s`.
    fn leaf_index(&self, p: HeapPoint) -> usize {
        let p = p.get();
        assert!(self.n <= p && p < 2 * self.n);
        (if p < self.s { p + self.n } else { p } - self.s) as usize
    }

    /// Push `p` down to the bottom row and read off the spots under it
    /// ([`HeapPoint::descendants_at_depth`]). Then turn the first spot
    /// and the end spot into elements. A spot that doesn't exist moves up
    /// to its parent first (`>> 1`).
    ///
    /// ```text
    /// spot:      8  9 10 11 12 13 14 15
    /// node 2:   [8          )12            -> elements 0..3
    /// node 3:               [12         )16 -> elements 3..5
    /// ```
    ///
    /// Spot `12` doesn't exist. Its parent is node `6`, which holds
    /// element `3`.
    fn node_bounds(&self, p: HeapPoint) -> (usize, usize) {
        let n = self.n;
        assert!(p.get() < 2 * n, "node {p} outside a tree of {n} elements");
        let deepest = (2 * n - 1).ilog2();
        let Range { start: x, end: y } = p.descendants_at_depth(deepest);
        debug_assert!(self.s <= x && x < y && y <= 2 * self.s);
        let fold = |v: u32| {
            let v = if v >= 2 * n { (v >> 1) + n } else { v };
            (v - self.s) as usize
        };
        (fold(x), fold(y))
    }
}

/// Element `i` is leaf `n + i`. Simple, but nodes can be in charge of a run
/// that wraps around from the last element to `0`.
///
/// # How it works
///
/// The leaves are nodes `n..2n`, and they hold the elements in number
/// order. With `n = 5`:
///
/// ```text
///                     1
///                  /     \
///                2         3
///               / \       / \
///              4  [5]   [6] [7]       <- elements 0, 1, 2
///             / \
///           [8] [9]                   <- elements 3, 4
/// ```
///
/// Reading the leaves from left to right gives `3, 4, 0, 1, 2`: the bottom
/// row comes first, but it holds the *last* elements. So node `2` is in
/// charge of `3, 4, 0`, a run that goes past the end and wraps to `0`, like
/// the hand of a clock going past 12. The root is in charge of all five,
/// starting at `3`.
///
/// Going up the tree (point updates, range queries) never cares about this.
/// Going down to search does, so use [`InOrderLayout`] for that.
#[derive(Clone, Copy, Debug, Default)]
pub struct CircularLayout {
    /// The number of elements.
    n: u32,
}

impl HeapLayout for CircularLayout {
    fn new(n: usize) -> Self {
        let n = u32::try_from(n)
            .ok()
            .filter(|&n| n <= 1 << 30)
            .expect("CircularLayout: n must be at most 2^30");
        Self { n }
    }

    fn len(&self) -> usize {
        self.n as usize
    }

    fn point(&self, i: usize) -> HeapPoint {
        assert!(i < self.len());
        HeapPoint::new(self.n + i as u32).unwrap()
    }

    /// Elements `l..r` are leaves `n + l .. n + r`.
    fn range(&self, r: Range<usize>) -> HeapRange {
        assert!(r.start <= r.end && r.end <= self.len());
        if self.is_empty() {
            HeapRange::default()
        } else {
            HeapRange::new(self.n + r.start as u32, self.n + r.end as u32)
        }
    }

    fn leaf_index(&self, p: HeapPoint) -> usize {
        let p = p.get();
        assert!(self.n <= p && p < 2 * self.n);
        (p - self.n) as usize
    }

    /// Like [`InOrderLayout::node_bounds`]: push `p` down to the bottom row,
    /// move spots that don't exist up to their parent (`>> 1`), and read off
    /// the elements (`- n`). The returned `end` is in `1..=n`, never `0`.
    ///
    /// ```text
    /// spot:      8  9 10 11 12 13 14 15
    /// node 2:   [8          )12            -> start 8 - 5 = 3,
    ///                                         end   12 -> 6 -> 6 - 5 = 1
    ///                                         so elements 3, 4, 0
    /// ```
    fn node_bounds(&self, p: HeapPoint) -> (usize, usize) {
        let n = self.n;
        assert!(p.get() < 2 * n, "node {p} outside a tree of {n} elements");
        let deepest = (2 * n - 1).ilog2();
        let Range { start: x, end: y } = p.descendants_at_depth(deepest);
        let x = if x >= 2 * n { x >> 1 } else { x };
        // `>`, not `>=`: an end of exactly `2n` stays, so it becomes `n`, not `0`.
        let y = if y > 2 * n { y >> 1 } else { y };
        ((x - n) as usize, (y - n) as usize)
    }

    /// Counts the wrap-around too: a run from `3` to `1` with `n = 5` is
    /// `3, 4, 0`, so `3`.
    fn node_size(&self, p: HeapPoint) -> usize {
        let (l, r) = self.node_bounds(p);
        if r > l { r - l } else { r + self.len() - l }
    }
}

#[cfg(test)]
mod test {
    use super::{CircularLayout, HeapLayout, InOrderLayout};
    use crate::collections::heap_point::HeapPoint;

    fn node(i: u32) -> HeapPoint {
        HeapPoint::new(i).unwrap()
    }

    /// The elements under `p`, read from left to right, using only `point`
    /// and a plain walk down the tree.
    fn elements_under(layout: &impl HeapLayout, p: u32, out: &mut Vec<usize>) {
        let n = layout.len() as u32;
        if p >= 2 * n {
            return;
        }
        if p >= n {
            out.push(layout.leaf_index(node(p)));
        } else {
            elements_under(layout, 2 * p, out);
            elements_under(layout, 2 * p + 1, out);
        }
    }

    /// Checks every method of `layout` against a brute-force model.
    /// `in_order` says whether runs must never wrap.
    fn check(layout: &impl HeapLayout, in_order: bool) {
        let n = layout.len();
        assert_eq!(layout.is_empty(), n == 0);

        // point and leaf_index are opposites, and use each leaf once.
        let mut seen = vec![false; 2 * n];
        for i in 0..n {
            let p = layout.point(i);
            assert!(n <= p.i() && p.i() < 2 * n, "n={n} i={i} p={p}");
            assert!(!seen[p.i()]);
            seen[p.i()] = true;
            assert_eq!(layout.leaf_index(p), i);
        }

        // node_bounds and node_size match the elements actually under the node.
        for p in 1..2 * n as u32 {
            let mut under = Vec::new();
            elements_under(layout, p, &mut under);
            let (start, end) = layout.node_bounds(node(p));
            let size = layout.node_size(node(p));
            assert_eq!(size, under.len(), "n={n} p={p}");
            let expected: Vec<usize> = (0..size).map(|k| (start + k) % n).collect();
            assert_eq!(under, expected, "n={n} p={p}");
            assert_eq!(end, (start + size - 1) % n + 1, "n={n} p={p}");
            if in_order {
                assert!(start < end, "n={n} p={p}");
            }

            // The split is where the right child starts.
            if (p as usize) < n {
                let split = layout.node_split(node(p));
                assert!(1 <= split && split <= n);
                assert_eq!(split % n, layout.node_bounds(node(2 * p + 1)).0);
            }
        }

        // range: the cover holds exactly the elements l..r, left to right.
        for l in 0..=n {
            for r in l..=n {
                let mut got = Vec::new();
                for p in layout.range(l..r).cover_l_to_r() {
                    elements_under(layout, p.get(), &mut got);
                }
                assert_eq!(got, (l..r).collect::<Vec<_>>(), "n={n} range={l}..{r}");
            }
        }
    }

    #[test]
    fn in_order_all_small() {
        for n in 0..=70 {
            check(&InOrderLayout::new(n), true);
        }
    }

    #[test]
    fn circular_all_small() {
        for n in 0..=70 {
            check(&CircularLayout::new(n), false);
        }
    }

    #[test]
    fn in_order_split_matches_original_formula() {
        for n in 1..=300usize {
            let layout = InOrderLayout::new(n);
            let (n32, s) = (n as u32, n.next_power_of_two() as u32);
            for a in 1..n32 {
                let l = (2 * a + 1).leading_zeros() - (2 * n32 - 1).leading_zeros();
                let x = (2 * a + 1) << l;
                let original = (if x >= 2 * n32 { (x >> 1) + n32 } else { x } - s) as usize;
                assert_eq!(layout.node_split(node(a)), original, "n={n} a={a}");
            }
        }
    }

    /// The `n = 5` pictures in the docs.
    #[test]
    fn exact_outputs_n5() {
        let in_order = InOrderLayout::new(5);
        let leaves: Vec<u32> = (0..5).map(|i| in_order.point(i).get()).collect();
        assert_eq!(leaves, [8, 9, 5, 6, 7]);
        assert_eq!(in_order.node_bounds(node(1)), (0, 5));
        assert_eq!(in_order.node_bounds(node(2)), (0, 3));
        assert_eq!(in_order.node_bounds(node(3)), (3, 5));
        let r = in_order.range(1..4);
        assert_eq!((r.a(), r.b()), (9, 14));

        let circular = CircularLayout::new(5);
        let leaves: Vec<u32> = (0..5).map(|i| circular.point(i).get()).collect();
        assert_eq!(leaves, [5, 6, 7, 8, 9]);
        assert_eq!(circular.node_bounds(node(1)), (3, 3));
        assert_eq!(circular.node_size(node(1)), 5);
        assert_eq!(circular.node_bounds(node(2)), (3, 1));
        assert_eq!(circular.node_size(node(2)), 3);
        assert_eq!(circular.node_bounds(node(3)), (1, 3));
        assert_eq!(circular.node_split(node(1)), 1);
    }

    #[test]
    fn largest_n() {
        let n = 1 << 30;
        for layout in [
            &InOrderLayout::new(n) as &dyn HeapLayout,
            &CircularLayout::new(n),
        ] {
            assert_eq!(layout.node_bounds(HeapPoint::ROOT), (0, n));
            assert_eq!(layout.leaf_index(layout.point(n - 1)), n - 1);
            assert_eq!(layout.range(0..n).cover().count(), 1);
        }
    }

    #[test]
    #[should_panic(expected = "at most 2^30")]
    fn in_order_too_big() {
        InOrderLayout::new((1 << 30) + 1);
    }

    #[test]
    #[should_panic(expected = "at most 2^30")]
    fn circular_too_big() {
        CircularLayout::new((1 << 30) + 1);
    }

    #[test]
    #[should_panic(expected = "outside a tree")]
    fn node_bounds_outside() {
        InOrderLayout::new(5).node_bounds(node(10));
    }

    #[test]
    #[should_panic(expected = "outside a tree")]
    fn node_bounds_empty() {
        CircularLayout::new(0).node_bounds(HeapPoint::ROOT);
    }
}
