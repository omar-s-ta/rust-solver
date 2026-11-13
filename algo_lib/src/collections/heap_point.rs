//! A typed index into a 1-based, implicit binary tree (heap layout).
//!
//! [`HeapPoint`] names a node of a complete binary tree stored in a flat array
//! the way a binary heap or an iterative segment tree stores it: no pointers,
//! just arithmetic on the index. Wrapping the index in its own type keeps tree
//! positions from being mixed up with ordinary element indices, and makes the
//! navigation (`left`, `right`, `parent`, ancestors) explicit at call sites.
//!
//! # Layout
//!
//! The root is `1`. Node `i` has children `2i` (left) and `2i + 1` (right),
//! and parent `i / 2`. In binary this is a path encoding: the leading `1` is
//! the root, and each following bit is one step down — `0` for left, `1` for
//! right. So `0b1101` is root → right → left → right, and:
//!
//! - **Children** append a bit: `(i << 1) | right`.
//! - **Parent** drops the last bit: `i >> 1`.
//! - **Depth** is the number of bits after the leading one: `ilog2(i)`.
//! - **The ancestor at distance `l`** is `i >> l`.
//!
//! ```text
//!               1
//!          ┌────┴────┐
//!          2         3
//!        ┌─┴─┐     ┌─┴─┐
//!        4   5     6   7
//! ```
//!
//! Index `0` is not a node — it is exactly what `parent` of the root would
//! compute. The index is stored as a [`NonZeroU32`], which turns that into the
//! `None` of [`parent`](HeapPoint::parent) and also gives
//! `Option<HeapPoint>` the same size as `u32`.
//!
//! # Indexing
//!
//! A `HeapPoint` indexes `[T]` and `Vec<T>` directly, at position
//! [`i()`](HeapPoint::i). Slot `0` of the backing storage is never addressed,
//! so a tree of `n` nodes needs a buffer of length `n + 1`. Out-of-bounds
//! points panic like any slice index.
//!
//! # Limits
//!
//! The index is a `u32`, so depth is at most `31`. Descending past that is a
//! bug: [`child`](HeapPoint::child) of a node at depth `31` shifts the root
//! bit out. Debug builds catch this with an assertion; release builds either
//! panic or silently return an unrelated shallow node.
//! Real arrays run out of memory long before then.
//!
//! # Example
//!
//! ```
//! use algo_lib::collections::heap_point::HeapPoint;
//!
//! let root = HeapPoint::ROOT;
//! let p = root.right().left(); // 0b110
//! assert_eq!(p.get(), 6);
//! assert_eq!(p.parent(), Some(root.right()));
//! assert_eq!(root.parent(), None);
//!
//! let down: Vec<u32> = p.ancestors_down().map(HeapPoint::get).collect();
//! assert_eq!(down, [1, 3]);
//!
//! let mut tree = vec![0; 8];
//! tree[p] = 42;
//! assert_eq!(tree[6], 42);
//! ```

use std::{
    fmt::Display,
    num::NonZeroU32,
    ops::{Index, IndexMut},
};

use crate::io::output::Writable;

/// A node of a 1-based implicit binary tree; see the [module docs](self).
///
/// The ordering is the numeric order of the index, which is breadth-first
/// order: every node sorts after all nodes at smaller depth.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct HeapPoint {
    i: NonZeroU32,
}

impl HeapPoint {
    /// The root of the tree, index `1`.
    pub const ROOT: HeapPoint = HeapPoint { i: NonZeroU32::MIN };

    /// The node at index `a`, or `None` if `a` is `0`.
    pub fn new(a: u32) -> Option<HeapPoint> {
        NonZeroU32::new(a).map(|i| HeapPoint { i })
    }

    /// The raw 1-based index.
    pub const fn get(self) -> u32 {
        self.i.get()
    }

    /// The raw 1-based index as a `usize`, for indexing storage.
    pub const fn i(self) -> usize {
        self.get() as usize
    }

    /// The left (`right == false`) or right (`right == true`) child.
    ///
    /// Branch-free form of [`left`](Self::left)/[`right`](Self::right), for
    /// walks driven by a bit, e.g. a key's bits in a binary trie.
    ///
    /// # Panics
    ///
    /// The index must be below `2^31` (depth at most `30`); debug builds
    /// assert this. Beyond that the root bit is shifted out, so in release
    /// builds the left child of `2^31` is `0`, which panics, and every other
    /// case silently yields a wrong, shallow node (the right child of `2^31`
    /// is the root).
    pub const fn child(self, right: bool) -> HeapPoint {
        debug_assert!(
            self.get() < 1 << 31,
            "HeapPoint::child: depth would exceed 31"
        );
        // i >= 1, so ((i << 1) | right) >= 2 as long as the shift does not
        // drop the top bit, i.e. for i < 2^31 (asserted above).
        HeapPoint {
            i: NonZeroU32::new((self.get() << 1) | right as u32).unwrap(),
        }
    }

    /// The left child, `2i`. Same limits as [`child`](Self::child).
    pub const fn left(self) -> HeapPoint {
        self.child(false)
    }

    /// The right child, `2i + 1`. Same limits as [`child`](Self::child).
    pub const fn right(self) -> HeapPoint {
        self.child(true)
    }

    /// The parent, `i / 2`, or `None` for the root.
    pub fn parent(self) -> Option<HeapPoint> {
        HeapPoint::new(self.get() >> 1)
    }

    /// `self`, then each ancestor up to and including the root.
    ///
    /// The usual path for bottom-up updates: change a leaf, then recompute
    /// every node above it. Yields `depth + 1` points.
    pub fn self_and_ancestors(self) -> impl Iterator<Item = HeapPoint> {
        std::iter::successors(Some(self), |p| p.parent())
    }

    /// The strict ancestors of `self`, from the parent up to the root.
    ///
    /// Excludes `self`; empty for the root. Unlike
    /// [`self_and_ancestors`](Self::self_and_ancestors), each step is computed
    /// directly as `i >> l` rather than from the previous one, so the iterator
    /// is double-ended and knows its length.
    pub fn ancestors_up(self) -> impl DoubleEndedIterator<Item = HeapPoint> {
        // l <= depth = ilog2(i), so i >> l >= 1: the unwrap can not fail.
        (1..=self.depth()).map(move |l| HeapPoint {
            i: NonZeroU32::new(self.get() >> l).unwrap(),
        })
    }

    /// The strict ancestors of `self`, from the root down to the parent.
    ///
    /// The order of [`ancestors_up`](Self::ancestors_up) reversed — the path
    /// for top-down work such as pushing lazy tags before touching a node.
    pub fn ancestors_down(self) -> impl DoubleEndedIterator<Item = HeapPoint> {
        self.ancestors_up().rev()
    }

    /// Distance from the root: `0` for the root, `floor(log2(i))` in general.
    const fn depth(self) -> u32 {
        self.i.ilog2()
    }
}

/// Formats the raw index.
impl Display for HeapPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.i.fmt(f)
    }
}

/// Writes the raw index.
impl Writable for HeapPoint {
    fn write(&self, output: &mut crate::io::output::Output) {
        self.get().write(output)
    }
}

impl From<HeapPoint> for usize {
    fn from(value: HeapPoint) -> Self {
        value.i()
    }
}

/// Indexes the slice at [`HeapPoint::i`]; panics if out of bounds.
impl<T> Index<HeapPoint> for [T] {
    type Output = T;

    fn index(&self, point: HeapPoint) -> &Self::Output {
        &self[point.i()]
    }
}

/// Indexes the slice at [`HeapPoint::i`]; panics if out of bounds.
impl<T> IndexMut<HeapPoint> for [T] {
    fn index_mut(&mut self, point: HeapPoint) -> &mut Self::Output {
        &mut self[point.i()]
    }
}

/// Indexes the vector at [`HeapPoint::i`]; panics if out of bounds.
impl<T> Index<HeapPoint> for Vec<T> {
    type Output = T;

    fn index(&self, point: HeapPoint) -> &Self::Output {
        &self.as_slice()[point]
    }
}

/// Indexes the vector at [`HeapPoint::i`]; panics if out of bounds.
impl<T> IndexMut<HeapPoint> for Vec<T> {
    fn index_mut(&mut self, point: HeapPoint) -> &mut Self::Output {
        &mut self.as_mut_slice()[point]
    }
}
