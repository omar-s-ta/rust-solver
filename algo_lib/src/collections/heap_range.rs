//! A half-open range of nodes in a 1-based implicit binary tree, and its
//! decomposition into whole subtrees.
//!
//! This is the index arithmetic of a bottom-up (iterative) segment tree,
//! separated from any storage. A [`HeapRange`] `[a, b)` names a run of
//! consecutive [`HeapPoint`]s; its methods return the nodes a segment tree
//! touches when answering a query or applying an update on that run:
//!
//! - [`cover`](HeapRange::cover) / [`cover_with_side`](HeapRange::cover_with_side) /
//!   [`cover_l_to_r`](HeapRange::cover_l_to_r): the fewest nodes whose
//!   subtrees together hold exactly the range. Fold over these to answer a
//!   query, or apply an update to them.
//! - [`boundary_down`](HeapRange::boundary_down) /
//!   [`boundary_up`](HeapRange::boundary_up): the strict ancestors of those
//!   nodes. Push lazy tags down along these before touching the cover, and
//!   recompute them bottom-up afterwards.
//!
//! # The range invariant
//!
//! A range requires `1 <= a <= b <= 2a`. The reason is that the window
//! `[a, 2a)` is a *frontier*: every downward path from the root passes
//! through exactly one index in it. (Going down maps `i` to `2i` or
//! `2i + 1`, so a path cannot jump from below `a` to `2a` or above.) So the
//! window behaves like one row of leaves, and `[a, b)` is a contiguous run
//! of that row. "Covering the range" means covering those leaves.
//!
//! When `a` is not a power of two, the row spans two depths. For `a = 5` it
//! is `5 6 7 | 8 9`: index order lists the shallow part first, while
//! left-to-right in the drawing the deep part comes first. Index order is
//! the one that matters, and it is the order of the elements in a segment
//! tree, as shown below.
//!
//! # Index arithmetic
//!
//! The code is bit tricks on the layout rules in the
//! [`heap_point` docs](crate::collections::heap_point#layout): the ancestor
//! `l` levels up is `i >> l`, the nodes `l` levels below a node form one run,
//! and `i` is the leftmost node below `i >> l` when its last `l` bits are `0`.
//!
//! # Segment tree mapping
//!
//! A segment tree over `n` elements keeps element `i` at node `n + i`, for any
//! `n` (not only powers of two). The query `[l, r)` over elements is the range
//! `HeapRange::new(n + l, n + r)`. It is valid because `n <= n + l` and
//! `n + r <= 2n`.
//!
//! # Example
//!
//! A range sum over five elements, with a tree that is not a power of two:
//!
//! ```
//! use algo_lib::collections::heap_range::HeapRange;
//!
//! let xs = [3, 1, 4, 1, 5];
//! let n = xs.len();
//! let mut t = vec![0; 2 * n];
//! t[n..].copy_from_slice(&xs);
//! for i in (1..n).rev() {
//!     t[i] = t[2 * i] + t[2 * i + 1];
//! }
//!
//! // Sum of xs[1..4] = 1 + 4 + 1.
//! let r = HeapRange::new((n + 1) as u32, (n + 4) as u32);
//! let sum: i32 = r.cover().map(|p| t[p]).sum();
//! assert_eq!(sum, 6);
//! ```

use std::iter::FusedIterator;

use crate::collections::heap_point::HeapPoint;

/// The node at index `a`. Every caller derives `a >= 1` from the range
/// invariant, so this never panics.
fn point(a: u32) -> HeapPoint {
    HeapPoint::new(a).expect("HeapRange: index 0 is not a node")
}

/// Error from [`HeapRange::try_new`]: the bounds break
/// `1 <= a <= b <= 2a`. It keeps the rejected bounds for the message.
#[derive(Debug)]
pub struct InvalidHeapRange {
    a: u32,
    b: u32,
}

impl std::fmt::Display for InvalidHeapRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid heap range [{}, {}): need 1 <= a <= b <= 2a",
            self.a, self.b
        )
    }
}

impl std::error::Error for InvalidHeapRange {}

/// The half-open run of nodes `[a, b)`, with `1 <= a <= b <= 2a`; see the
/// [module docs](self).
///
/// Construction checks the invariant, so every method can rely on it.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct HeapRange {
    a: u32,
    b: u32,
}

impl HeapRange {
    /// The range `[a, b)`. Fails with [`InvalidHeapRange`] unless
    /// `1 <= a <= b <= 2a`.
    pub fn try_new(a: u32, b: u32) -> Result<Self, InvalidHeapRange> {
        if 1 <= a && a <= b && u64::from(b) <= 2 * u64::from(a) {
            Ok(HeapRange { a, b })
        } else {
            Err(InvalidHeapRange { a, b })
        }
    }

    /// The range `[a, b)`.
    ///
    /// # Panics
    ///
    /// Panics unless `1 <= a <= b <= 2a`; use [`try_new`](Self::try_new) to
    /// handle that case.
    pub fn new(a: u32, b: u32) -> Self {
        Self::try_new(a, b).unwrap_or_else(|e| panic!("{e}"))
    }

    /// The inclusive start.
    pub const fn a(self) -> u32 {
        self.a
    }

    /// The exclusive end.
    pub const fn b(self) -> u32 {
        self.b
    }

    /// `true` if the range has no nodes (`a == b`).
    pub const fn is_empty(self) -> bool {
        self.a == self.b
    }

    /// The nodes a segment tree reads for a query on this range, or tags for
    /// an update.
    ///
    /// Each returned node stands in for the range nodes in its subtree. When
    /// two neighbouring range nodes share a parent, the parent is returned
    /// instead of both, and so on up the tree. Each range node is covered by
    /// exactly one returned node, and nothing outside the range is covered.
    /// "Outside" means other nodes of the range's row `[a, 2a)`, see the
    /// [module docs](self).
    ///
    /// For `[5, 9)`, the range is `5 6 7 8`, and `6, 7` merge into their
    /// parent `3`:
    ///
    /// ```text
    ///               1
    ///          ┌────┴────┐
    ///          2         3 ◀
    ///        ┌─┴─┐     ┌─┴─┐
    ///        4   5 ◀   6   7
    ///      ┌─┴─┐
    ///      8 ◀ 9
    /// ```
    ///
    /// At most two nodes per level, so O(log b) in total. An empty range
    /// yields nothing.
    ///
    /// **Order:** bottom-up, which is *not* left to right (`5, 8, 3` above,
    /// while left to right is `5, 3, 8`). That's fine for commutative
    /// operations like sum, min or xor. When order matters, use
    /// [`cover_l_to_r`](Self::cover_l_to_r) or
    /// [`cover_with_side`](Self::cover_with_side).
    ///
    /// ```
    /// use algo_lib::collections::{heap_point::HeapPoint, heap_range::HeapRange};
    ///
    /// let cover: Vec<u32> = HeapRange::new(5, 9).cover().map(HeapPoint::get).collect();
    /// assert_eq!(cover, [5, 8, 3]);
    /// ```
    pub fn cover(self) -> impl Iterator<Item = HeapPoint> {
        self.cover_with_side().map(|(p, _)| p)
    }

    /// The nodes of [`cover`](Self::cover), in the same order, each paired
    /// with the [`Side`] of the range it was found from.
    ///
    /// The walk climbs from both ends of the range towards the middle, so the
    /// two kinds of nodes come out in opposite orders:
    ///
    /// - **[`Side::Left`]** nodes come in left-to-right order, so *append*
    ///   each to a left accumulator: `acc_l = acc_l · node`.
    /// - **[`Side::Right`]** nodes come in right-to-left order, so *prepend*
    ///   each to a right accumulator: `acc_r = node · acc_r`.
    /// - The answer is `acc_l · acc_r`.
    ///
    /// That gives a correct fold for operations where order matters (matrix
    /// product, string concatenation, ...). For `[5, 9)` (the picture on
    /// [`cover`](Self::cover)) the items are `(5, Left)`, `(8, Right)`,
    /// `(3, Left)`. That gives `acc_l = 5 · 3` and `acc_r = 8`, so the
    /// answer is `5 · 3 · 8`.
    ///
    /// If one accumulator is enough for you,
    /// [`cover_l_to_r`](Self::cover_l_to_r) is simpler: it already yields
    /// `5, 3, 8`.
    ///
    /// Concatenating `"abcd"` from a tree of strings:
    ///
    /// ```
    /// use algo_lib::collections::heap_range::{HeapRange, Side};
    ///
    /// let xs = ["a", "b", "c", "d", "e"];
    /// let n = xs.len();
    /// let mut t = vec![String::new(); 2 * n];
    /// for (i, x) in xs.iter().enumerate() {
    ///     t[n + i] = x.to_string();
    /// }
    /// for i in (1..n).rev() {
    ///     t[i] = format!("{}{}", t[2 * i], t[2 * i + 1]);
    /// }
    ///
    /// let (mut acc_l, mut acc_r) = (String::new(), String::new());
    /// for (p, side) in HeapRange::new(n as u32, (n + 4) as u32).cover_with_side() {
    ///     match side {
    ///         Side::Left => acc_l = format!("{acc_l}{}", t[p]),
    ///         Side::Right => acc_r = format!("{}{acc_r}", t[p]),
    ///     }
    /// }
    /// assert_eq!(acc_l + &acc_r, "abcd");
    /// ```
    pub fn cover_with_side(self) -> CoverWithSide {
        CoverWithSide {
            x: self.a,
            y: self.b,
            pending: None,
        }
    }

    /// The nodes of [`cover`](Self::cover) in left-to-right order: by the
    /// range nodes they hold, in increasing index order.
    ///
    /// This is the order for a non-commutative fold done with a single
    /// accumulator. It is double-ended, so `.rev()` gives right to left.
    /// Each node is computed directly from the bits of `a` and `b`, with no
    /// buffering. It yields the same left/right split as
    /// [`cover_with_side`](Self::cover_with_side): the left nodes in their
    /// order, then the right nodes reversed.
    ///
    /// ```
    /// use algo_lib::collections::{heap_point::HeapPoint, heap_range::HeapRange};
    ///
    /// let r = HeapRange::new(5, 9); // 5 | 6 7 | 8
    /// let l2r: Vec<u32> = r.cover_l_to_r().map(HeapPoint::get).collect();
    /// assert_eq!(l2r, [5, 3, 8]);
    /// let r2l: Vec<u32> = r.cover_l_to_r().rev().map(HeapPoint::get).collect();
    /// assert_eq!(r2l, [8, 3, 5]);
    /// ```
    pub fn cover_l_to_r(self) -> impl DoubleEndedIterator<Item = HeapPoint> + FusedIterator {
        let (a, b, mask) = (self.a, self.b, self.anc_mask());
        let left = Bits(a.wrapping_neg() & mask).map(move |lvl| point(((a - 1) >> lvl) + 1));
        let right = Bits(b & mask).rev().map(move |lvl| point((b >> lvl) - 1));
        left.chain(right)
    }

    /// The nodes *above* the [`cover`](Self::cover): every strict ancestor of
    /// a cover node, each listed once, top-down (parents before children).
    ///
    /// For `[5, 9)`, with the cover marked `◀` and the boundary marked `●`:
    ///
    /// ```text
    ///                1 ●
    ///           ┌────┴─────┐
    ///           2 ●        3 ◀
    ///         ┌─┴──┐     ┌─┴─┐
    ///         4 ●  5 ◀   6   7
    ///       ┌─┴─┐
    ///       8 ◀ 9
    /// ```
    ///
    /// These are the nodes that are only *partly* inside the range. They are
    /// why a lazy segment tree needs extra work around the cover:
    ///
    /// - **Before** reading or updating the cover, push pending tags down
    ///   from these nodes, top-down. That is this method. Otherwise a cover
    ///   node could miss an update stored higher up.
    /// - **After** updating the cover, recompute these nodes from their
    ///   children, bottom-up. That is [`boundary_up`](Self::boundary_up).
    ///   Otherwise they would keep their stale values.
    ///
    /// ```text
    /// query:  push(boundary_down); fold(cover)
    /// update: push(boundary_down); apply(cover); pull(boundary_up)
    /// ```
    ///
    /// The textbook loop pushes along every ancestor of `a` and `b - 1`.
    /// This method skips those that are already wholly inside the range,
    /// because they are cover nodes or lie below one, and are never read on
    /// their own. That leaves at most two paths to the root, O(log b) nodes.
    /// An empty range has an empty cover, so its boundary is empty too.
    ///
    /// ```
    /// use algo_lib::collections::{heap_point::HeapPoint, heap_range::HeapRange};
    ///
    /// let down: Vec<u32> = HeapRange::new(5, 9).boundary_down().map(HeapPoint::get).collect();
    /// assert_eq!(down, [1, 2, 4]);
    /// assert_eq!(HeapRange::new(5, 5).boundary_down().count(), 0);
    /// ```
    pub fn boundary_down(self) -> impl Iterator<Item = HeapPoint> {
        let (l, r) = self.boundary();
        let right = r.top.ancestors_up().take(r.below_join).rev();
        l.top.ancestors_down().take(l.all).chain(right)
    }

    /// The same nodes as [`boundary_down`](Self::boundary_down), bottom-up
    /// (children before parents).
    ///
    /// Use it after an update to recompute each node from its children. The
    /// bottom-up order guarantees that a node's children are already up to
    /// date when it is recomputed. See `boundary_down` for the picture and
    /// the lazy segment tree recipe.
    ///
    /// ```
    /// use algo_lib::collections::{heap_point::HeapPoint, heap_range::HeapRange};
    ///
    /// let up: Vec<u32> = HeapRange::new(5, 9).boundary_up().map(HeapPoint::get).collect();
    /// assert_eq!(up, [4, 2, 1]);
    /// ```
    pub fn boundary_up(self) -> impl Iterator<Item = HeapPoint> {
        let (l, r) = self.boundary();
        let left = l.top.ancestors_up().take(l.below_join);
        left.chain(r.top.ancestors_up().take(r.all))
    }

    /// The levels at which the bottom-up cover walk can still take nodes, as
    /// a bit mask: bit `l` set means level `l`. Level `l` is `l` steps up
    /// from the range's row, so level 0 is the row itself. The bit rules
    /// used here are in the [`heap_point` docs](crate::collections::heap_point#layout).
    ///
    /// **The walk.** At level `l`, what is left to cover is `[x_l, y_l)`,
    /// with
    ///
    /// ```text
    /// x_l = ((a - 1) >> l) + 1    first node whose row nodes all lie at or after a
    /// y_l = b >> l                end of the nodes whose row nodes all lie before b
    /// ```
    ///
    /// (Each level-`l` node holds one run of row nodes, so this rounds `a` up
    /// and `b` down to whole level-`l` nodes.)
    ///
    /// - If `x_l` is a right child (odd), its parent would also cover
    ///   `x_l - 1`, which lies before `a`. So `x_l` cannot be merged
    ///   upwards, and it becomes a **left cover node**. In bits: `x_l` is odd
    ///   ⇔ bit `l` of `a - 1` is `0` ⇔ bit `l` of `-a` is `1`, because
    ///   `-a == !(a - 1)`.
    /// - If `y_l` is odd, `y_l - 1` is a left child whose parent would also
    ///   cover `y_l`, which lies past `b`. So `y_l - 1` becomes a **right
    ///   cover node**. In bits: bit `l` of `b` is `1`.
    ///
    /// **When the walk stops.** `(a - 1) >> l` and `b >> l` are the ancestors
    /// `l` levels up, and they are equal above `h`, the highest bit where
    /// `a - 1` and `b` differ (`h = ((a - 1) ^ b).ilog2()`). At level `h`
    /// they are `2k` and `2k + 1`, so `x_h == y_h` and nothing is left. So
    /// only the levels below `h` can contribute, and the mask is `(1 << h) - 1`.
    ///
    /// The left cover nodes are then at the levels in `-a & mask`, and the
    /// right ones at the levels in `b & mask`. This is what
    /// [`cover_l_to_r`](Self::cover_l_to_r) iterates.
    ///
    /// Example `[5, 9)`: `a - 1 = 0b100`, `b = 0b1001`, so `h = 3` and
    /// `mask = 0b111`. `-5 & 0b111 = 0b011` gives left nodes at levels 0 and
    /// 1, which are `5` and `3`. `9 & 0b111 = 0b001` gives a right node at
    /// level 0, which is `8`.
    #[inline]
    fn anc_mask(self) -> u32 {
        // b >= a, so (a - 1) ^ b > 0 and the shift is in [0, 31].
        let anc_depth = ((self.a - 1) ^ self.b).ilog2();
        (1u32 << anc_depth) - 1
    }

    /// Finds the boundary nodes for [`boundary_down`](Self::boundary_down)
    /// and [`boundary_up`](Self::boundary_up).
    ///
    /// Think of every node as a **box** that holds all the nodes below it.
    /// Take the range `[10, 13)`, which is `10 11 12`:
    ///
    /// ```text
    ///                1
    ///          ┌─────┴─────┐
    ///          2           3
    ///       ┌──┴──┐     ┌──┴──┐
    ///       4     5     6     7
    ///      ┌┴┐   ┌┴─┐  ┌┴─┐  ┌┴─┐
    ///      8 9  10 11 12 13 14 15
    /// ```
    ///
    /// **A cut is a fence.** Put one fence where the range starts (before
    /// `10`) and one where it ends (before `13`). **Boundary nodes are the
    /// boxes a fence goes through**: they hold some nodes inside the range
    /// and some outside it. Each `[===]` below is one box, drawn over the
    /// bottom-row nodes it holds:
    ///
    /// ```text
    ///            fence 10     fence 13
    ///               |            |
    ///         8   9 | 10  11  12 | 13  14  15
    /// box 1: [======|============|===========]  both fences go through
    /// box 2: [======|=======]    |              fence 10 goes through
    /// box 3:        |        [===|===========]  fence 13 goes through
    /// box 4: [=====]|            |              no fence inside
    /// box 5:        |[======]    |              fence 10 only touches its edge
    /// box 6:        |        [===|===]          fence 13 goes through
    /// box 7:        |            |    [======]  no fence inside
    /// ```
    ///
    /// So the boundary is `1, 2, 3, 6`.
    ///
    /// **Finding the boxes for one fence.** Start at the node just right of
    /// the fence and climb up while you are a *left* child. Those boxes
    /// start exactly at the fence, so the fence only touches their edge.
    /// Every box above where you stop has the fence going through it.
    ///
    /// - Fence 10: `10` is a left child, so climb to `5`. `5` is a right
    ///   child, so stop. The boxes above `5` are `2, 1`.
    /// - Fence 13: `13` is a right child, so stop right away. The boxes
    ///   above `13` are `6, 3, 1`.
    ///
    /// In bits, "climb while you are a left child" means "drop the trailing
    /// zeros": `10 = 0b1010` becomes `0b101 = 5`.
    ///
    /// **The two lists meet at the top.** Both reach `1`. To list each box
    /// once, take one list fully and cut the other off where they meet.
    ///
    /// **Returns** one [`Path`] per fence, left fence first:
    ///
    /// ```text
    ///              top   all   below_join
    /// fence 10:     5     2     1          boxes 1, 2  (only 2 is below the meeting point)
    /// fence 13:    13     3     2          boxes 1, 3, 6  (6, 3 are below it)
    /// ```
    ///
    /// The callers take one list whole and the other only up to where they
    /// meet, so `1` is listed once:
    ///
    /// - `boundary_down`: all of fence 10, then fence 13 up to the meeting
    ///   point. That gives `1, 2` + `3, 6`.
    /// - `boundary_up`: fence 10 up to the meeting point, then all of fence 13.
    ///   That gives `2` + `6, 3, 1`.
    ///
    /// The meeting point is `((x - 1) ^ y).ilog2()`, the highest bit where the
    /// two paths still differ. Here it is `2`, and each `below_join` is that
    /// minus how far its fence climbed.
    ///
    /// For an empty range both fences are in the same place, so `below_join`
    /// is already `0` on both sides. `all` is set to `0` as well, so the
    /// callers return nothing.
    #[inline]
    fn boundary(self) -> (Path, Path) {
        let (mut x, mut y) = (self.a, self.b);
        // Both fence nodes must be on the same row. If b is one row below a,
        // use 2a instead of a. 2a is a's left child, so it climbs to the same
        // box and marks the same fence, and it is on b's row. Since b <= 2a,
        // b is now the left fence and 2a the right one.
        // No overflow: b is on a lower row than a, so a < 2^31.
        if x.ilog2() < y.ilog2() {
            x <<= 1;
            std::mem::swap(&mut x, &mut y);
        }
        let join = ((x - 1) ^ y).ilog2();
        let path = |c: u32| {
            let cx = c.trailing_zeros();
            let top = c >> cx;
            Path {
                top: point(top),
                all: if self.is_empty() {
                    0
                } else {
                    top.ilog2() as usize
                },
                below_join: join.saturating_sub(cx) as usize,
            }
        };
        (path(x), path(y))
    }
}

impl Default for HeapRange {
    /// The empty range [1, 1)
    fn default() -> Self {
        Self { a: 1, b: 1 }
    }
}

/// Formats as `[a..b)`.
impl std::fmt::Display for HeapRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}..{})", self.a, self.b)
    }
}

/// `[a, b]` to the range `[a, b)`, as [`HeapRange::try_new`].
impl TryFrom<[u32; 2]> for HeapRange {
    type Error = InvalidHeapRange;

    fn try_from([a, b]: [u32; 2]) -> Result<Self, Self::Error> {
        HeapRange::try_new(a, b)
    }
}

/// The bounds `[a, b]`.
impl From<HeapRange> for [u32; 2] {
    fn from(value: HeapRange) -> Self {
        [value.a, value.b]
    }
}

/// One fence's list of boundary boxes: the boxes above `top`, which are its
/// strict ancestors. See [`HeapRange::boundary`] for a worked example.
struct Path {
    /// Where the climb from the fence stops.
    top: HeapPoint,
    /// How many boxes are above `top`: the whole list. `0` for an empty
    /// range, which has no boundary.
    all: usize,
    /// How many of those come before this list meets the other fence's list.
    /// The boxes from the meeting point up are in both lists.
    below_join: usize,
}

/// Which end of the range a cover node was found from; see
/// [`HeapRange::cover_with_side`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Side {
    /// Found from the left end. These come in left-to-right order.
    Left,
    /// Found from the right end. These come in right-to-left order.
    Right,
}

/// Iterator returned by [`HeapRange::cover_with_side`].
#[derive(Clone, Debug)]
pub struct CoverWithSide {
    /// [x, y) what is still left to cover, at the current depth
    x: u32,
    y: u32,
    /// Right node found in the current step, still to be yielded after the
    /// step's left node.
    pending: Option<u32>,
}

impl Iterator for CoverWithSide {
    type Item = (HeapPoint, Side);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.pending.take() {
            return Some((point(e), Side::Right));
        }
        while self.x < self.y {
            let mut start = None;
            if self.x & 1 == 1 {
                start = Some(self.x);
                self.x += 1;
            }
            if self.y & 1 == 1 {
                self.y -= 1;
                self.pending = Some(self.y);
            }
            self.x >>= 1;
            self.y >>= 1;
            if let Some(s) = start {
                return Some((point(s), Side::Left));
            }
            if let Some(e) = self.pending.take() {
                return Some((point(e), Side::Right));
            }
        }
        None
    }
}

impl FusedIterator for CoverWithSide {}

/// The positions of the set bits of a mask: lowest first from the front,
/// highest first from the back.
#[derive(Clone, Debug)]
struct Bits(u32);

impl Iterator for Bits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        (self.0 != 0).then(|| {
            let i = self.0.trailing_zeros();
            self.0 &= self.0 - 1;
            i
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.0.count_ones() as usize;
        (n, Some(n))
    }
}

impl DoubleEndedIterator for Bits {
    fn next_back(&mut self) -> Option<Self::Item> {
        (self.0 != 0).then(|| {
            let i = self.0.ilog2();
            self.0 ^= 1 << i;
            i
        })
    }
}

impl ExactSizeIterator for Bits {}

impl FusedIterator for Bits {}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    use super::{HeapRange, Side};
    use crate::collections::heap_point::HeapPoint;

    fn ids(it: impl Iterator<Item = HeapPoint>) -> Vec<u32> {
        it.map(HeapPoint::get).collect()
    }

    /// The row nodes `[a, 2a)` inside `p`'s subtree, as half-open intervals.
    /// The row spans at most two depths, so `p` holds at most one run on each.
    fn held(p: u32, a: u32) -> Vec<(u64, u64)> {
        let (p, a) = (u64::from(p), u64::from(a));
        let (dp, d) = (p.ilog2(), a.ilog2());
        (d..=d + 1)
            .filter(|&k| k >= dp)
            .map(|k| {
                let (lo, hi) = (p << (k - dp), (p + 1) << (k - dp));
                (lo.max(a), hi.min(2 * a))
            })
            .filter(|(lo, hi)| lo < hi)
            .collect()
    }

    fn strict_ancestors(p: u32) -> impl Iterator<Item = u32> {
        std::iter::successors(Some(p >> 1), |q| Some(q >> 1)).take_while(|&q| q > 0)
    }

    /// Checks every documented property of `[a, b)` against a brute-force
    /// model. Works for any valid range, including ones near `u32::MAX`.
    fn check(a: u32, b: u32) {
        let r = HeapRange::new(a, b);
        let ctx = format!("[{a}, {b})");
        let side: Vec<(u32, Side)> = r.cover_with_side().map(|(p, s)| (p.get(), s)).collect();
        let cover = ids(r.cover());
        let l2r = ids(r.cover_l_to_r());

        // `cover` is `cover_with_side` without the side.
        assert_eq!(
            cover,
            side.iter().map(|&(p, _)| p).collect::<Vec<_>>(),
            "{ctx}"
        );

        // The cover holds exactly the range, each node once.
        let mut runs: Vec<(u64, u64)> = cover.iter().flat_map(|&p| held(p, a)).collect();
        runs.sort();
        let mut end = u64::from(a);
        for (lo, hi) in runs {
            assert_eq!(lo, end, "{ctx}: gap or overlap in the cover");
            end = hi;
        }
        assert_eq!(end, u64::from(b), "{ctx}: cover does not end at b");
        assert!(cover.len() <= 2 * 32, "{ctx}");

        // `cover_l_to_r` is the cover, sorted by the range nodes it holds.
        let mut by_position = cover.clone();
        by_position.sort_by_key(|&p| held(p, a)[0].0);
        assert_eq!(l2r, by_position, "{ctx}");
        let r2l = ids(r.cover_l_to_r().rev());
        assert_eq!(r2l, l2r.iter().rev().copied().collect::<Vec<_>>(), "{ctx}");

        // Left nodes in order, then right nodes reversed, is left to right.
        type Tagged = Vec<(u32, Side)>;
        let (left, right): (Tagged, Tagged) = side.iter().partition(|s| s.1 == Side::Left);
        let folded: Vec<u32> = left.iter().chain(right.iter().rev()).map(|s| s.0).collect();
        assert_eq!(folded, l2r, "{ctx}");

        // The boundary is the strict ancestors of the cover, each once.
        let want: BTreeSet<u32> = cover.iter().flat_map(|&p| strict_ancestors(p)).collect();
        let down = ids(r.boundary_down());
        let up = ids(r.boundary_up());
        for (name, got) in [("down", &down), ("up", &up)] {
            let set: BTreeSet<u32> = got.iter().copied().collect();
            assert_eq!(
                set.len(),
                got.len(),
                "{ctx}: boundary_{name} repeats a node"
            );
            assert_eq!(set, want, "{ctx}: boundary_{name}");
        }
        // Down lists parents first, up lists children first.
        for (i, &p) in down.iter().enumerate() {
            assert!(p == 1 || down[..i].contains(&(p >> 1)), "{ctx}: down order");
        }
        for (i, &p) in up.iter().enumerate() {
            assert!(p == 1 || up[i + 1..].contains(&(p >> 1)), "{ctx}: up order");
        }
    }

    #[test]
    fn all_small_ranges() {
        for a in 1..200 {
            for b in a..=2 * a {
                check(a, b);
            }
        }
    }

    #[test]
    fn ranges_near_u32_max() {
        const TOP: u32 = 1 << 31;
        let cases = [
            (TOP, TOP),
            (TOP, TOP + 1),
            (TOP, u32::MAX),
            (TOP + 1, u32::MAX),
            (u32::MAX - 1, u32::MAX),
            (u32::MAX, u32::MAX),
            (TOP - 1, u32::MAX - 1),
            (TOP - 1, TOP + 5),
            ((TOP >> 1) + 3, TOP + 6),
            (TOP >> 1, TOP),
        ];
        for (a, b) in cases {
            check(a, b);
        }
    }

    /// Pins the exact output order, which the property checks allow to vary.
    #[test]
    fn exact_outputs() {
        // (a, b, cover, cover_l_to_r, boundary_down, boundary_up)
        #[rustfmt::skip]
        let cases: [(u32, u32, &[u32], &[u32], &[u32], &[u32]); 9] = [
            (5, 9, &[5, 8, 3], &[5, 3, 8], &[1, 2, 4], &[4, 2, 1]),
            (9, 15, &[9, 14, 5, 6], &[9, 5, 6, 14], &[1, 2, 4, 3, 7], &[4, 2, 7, 3, 1]),
            (10, 13, &[12, 5], &[5, 12], &[1, 2, 3, 6], &[2, 6, 3, 1]),
            (4, 7, &[6, 2], &[2, 6], &[1, 3], &[3, 1]),
            (7, 9, &[7, 8], &[7, 8], &[1, 2, 4, 3], &[4, 2, 3, 1]),
            (5, 10, &[5, 3, 4], &[5, 3, 4], &[1, 2], &[2, 1]),
            (8, 16, &[1], &[1], &[], &[]),
            (1, 2, &[1], &[1], &[], &[]),
            (5, 5, &[], &[], &[], &[]),
        ];
        for (a, b, cover, l2r, down, up) in cases {
            let r = HeapRange::new(a, b);
            assert_eq!(ids(r.cover()), cover, "[{a}, {b}) cover");
            assert_eq!(ids(r.cover_l_to_r()), l2r, "[{a}, {b}) cover_l_to_r");
            assert_eq!(ids(r.boundary_down()), down, "[{a}, {b}) boundary_down");
            assert_eq!(ids(r.boundary_up()), up, "[{a}, {b}) boundary_up");
        }
    }

    #[test]
    fn side_tags() {
        let side: Vec<(u32, Side)> = HeapRange::new(5, 9)
            .cover_with_side()
            .map(|(p, s)| (p.get(), s))
            .collect();
        assert_eq!(side, [(5, Side::Left), (8, Side::Right), (3, Side::Left)]);
    }

    #[test]
    fn validation() {
        assert!(HeapRange::try_new(0, 0).is_err());
        assert!(HeapRange::try_new(5, 4).is_err());
        assert!(HeapRange::try_new(5, 11).is_err());
        assert!(HeapRange::try_new(5, 10).is_ok());
        assert!(HeapRange::try_new(u32::MAX, u32::MAX).is_ok());
        assert!(HeapRange::try_from([3, 7]).is_err());
        assert_eq!(<[u32; 2]>::from(HeapRange::new(3, 6)), [3, 6]);
        assert!(HeapRange::default().is_empty());
        assert_eq!(HeapRange::new(5, 9).to_string(), "[5..9)");
    }

    #[test]
    #[should_panic(expected = "invalid heap range")]
    fn new_panics_on_invalid() {
        HeapRange::new(5, 11);
    }
}
