//! A segment tree: ask "what do elements `l..r` add up to?" and change
//! elements, each in `O(log n)` instead of `O(n)`.
//!
//! "Add up" can be anything that is a [`Monoid`]: sum, min, max, gcd, matrix
//! product, ... The operation does not have to be commutative; answers always
//! combine the elements in order.
//!
//! # The idea
//!
//! The tree keeps the answer for a few well-chosen groups of elements. Here
//! is a sum tree over the elements `[3, 1, 4, 1]`:
//!
//! ```text
//!                  9              node 1: all four elements
//!             /         \
//!          4               5      node 2: elements 0, 1    node 3: elements 2, 3
//!        /   \           /   \
//!       3     1         4     1   leaves: one element each
//! ```
//!
//! (The nodes are numbered like a heap, see
//! [`heap_point`](crate::collections::heap_point). Where each element sits in
//! the leaves is decided by a layout, see
//! [`heap_layout`](crate::collections::heap_layout).)
//!
//! **Query.** To add up elements `1..4` we don't visit every element. We take
//! the fewest nodes that hold exactly those elements, marked `◀`, and combine
//! their values: `1 + 5 = 6`. That is
//! [`HeapRange::cover`](crate::collections::heap_range::HeapRange::cover).
//!
//! ```text
//!                  9
//!             /         \
//!          4               5 ◀
//!        /   \           /   \
//!       3     1 ◀       4     1
//! ```
//!
//! **Change one element.** Overwrite its leaf, then recompute every node above
//! it from its two children, up to the root. That is `log n` nodes.
//!
//! # Three type parameters
//!
//! `SegmentTree<M, T, L>`:
//!
//! - `M`: *what* to combine, a [`Monoid`].
//! - `T`: a [`Tag`], needed only for **range updates** ("add `x` to every
//!   element in `l..r`"). The default, [`NoTag`], means no range updates, only
//!   [`set`](SegmentTree::set).
//! - `L`: the [`HeapLayout`]. The default, [`InOrderLayout`], is the one to
//!   use, see [Which layout](#which-layout).
//!
//! # Range updates: lazy tags
//!
//! Changing every element of a range one by one is slow. Instead, we leave a
//! **note** (a tag) on the few nodes that cover the range, and fix their
//! values right away. The nodes below are *not* told yet.
//!
//! Add `10` to elements `1..4` of the tree above. The cover is leaf `1` and
//! node `3`:
//!
//! ```text
//!                 39              9 + 10 * 3
//!             /         \
//!         14               25 [+10]    <- the note waits here
//!        /   \            /   \
//!       3     11         4     1       <- still the old values, they
//!                                         haven't heard about +10
//! ```
//!
//! The nodes above the cover are recomputed from their children, so queries
//! that stop at them are right. A node with a note is right too, because
//! its own value already includes the note.
//!
//! The note is **pushed down** later, only when something needs to look
//! *inside* that node: the node hands the note to both children and throws
//! it away. So a query or update first pushes along the nodes above its
//! cover, top-down
//! ([`boundary_down`](crate::collections::heap_range::HeapRange::boundary_down)),
//! and an update recomputes them afterwards, bottom-up
//! ([`boundary_up`](crate::collections::heap_range::HeapRange::boundary_up)):
//!
//! ```text
//! query:  push(boundary_down); fold over cover
//! modify: push(boundary_down); apply to cover; pull(boundary_up)
//! ```
//!
//! This is why almost every method takes `&mut self`: even a read may push
//! notes down. How to write a tag is explained on [`Tag`].
//!
//! # Example
//!
//! Range add, range sum, over five elements:
//!
//! ```
//! use algo_lib::collections::segment_tree::{SegmentTree, Tag};
//! use algo_lib::math::type_classes::Monoid;
//!
//! struct Sum;
//! impl Monoid for Sum {
//!     type T = i64;
//!     fn empty() -> i64 {
//!         0
//!     }
//!     fn combine(a: i64, b: i64) -> i64 {
//!         a + b
//!     }
//! }
//!
//! /// "Add this to every element."
//! #[derive(Clone, Default)]
//! struct Add(i64);
//! impl Tag<Sum> for Add {
//!     fn is_empty(&self) -> bool {
//!         self.0 == 0
//!     }
//!     fn apply_to(&self, sum: i64, _l: usize, _r: usize, size: usize) -> Option<i64> {
//!         // The node holds `size` elements, each grows by `self.0`.
//!         Some(sum + self.0 * size as i64)
//!     }
//!     fn compose(&self, other: &Self) -> Self {
//!         Add(self.0 + other.0)
//!     }
//! }
//!
//! let xs = [3, 1, 4, 1, 5];
//! let mut t: SegmentTree<Sum, Add> = SegmentTree::from(&xs[..]);
//! assert_eq!(t.query(1..4), 6);
//!
//! t.modify(1..4, &Add(10)); // [3, 11, 14, 11, 5]
//! assert_eq!(t.query(1..4), 36);
//! assert_eq!(t.query(..), 44);
//! assert_eq!(t.get(2), 14);
//!
//! t.set(0, 100);
//! assert_eq!(t.query(..2), 111);
//! ```
//!
//! With the default [`NoTag`] you get a plain segment tree:
//! `SegmentTree<Sum>` has everything above except [`modify`](SegmentTree::modify).
//!
//! # Searching: `max_right` and `min_left`
//!
//! "How far can I go from `l` before the sum gets too big?" The tree answers
//! this in `O(log n)` by walking down, instead of trying every end. See
//! [`max_right`](SegmentTree::max_right) and [`min_left`](SegmentTree::min_left).
//! They need [`InOrder`], that is [`InOrderLayout`] (the default).
//!
//! # Which layout
//!
//! Use the default, [`InOrderLayout`].
//! [`CircularLayout`](crate::collections::heap_layout::CircularLayout) is
//! slightly cheaper but a node can hold a run that wraps from the last
//! element to `0`. Then there is no `max_right` / `min_left`, and a tag's
//! `apply_to(value, l, r, size)` can see `r <= l` for such a node. Lazy tags
//! still work: count a node's elements with `size`, never `r - l`. See
//! [`heap_layout`](crate::collections::heap_layout).
//!
//! # Cost
//!
//! `O(log n)` calls to `combine` (and to the tag, for lazy trees) per
//! operation, and `O(n)` to build. Memory is `2n` values, plus `n` tags if lazy.
//!
//! # Limits
//!
//! `n` is at most `2^30`, a limit of the layout.

use std::ops::{Range, RangeBounds};

use crate::{
    collections::{
        clamp::clamp,
        heap_layout::{HeapLayout, InOrder, InOrderLayout},
        heap_point::HeapPoint,
        heap_range::HeapRange,
    },
    math::type_classes::Monoid,
};

/// A **tag** is a to-do note that waits on a node, for that node's two
/// children. These docs use the words *tag* and *note* for the same thing.
///
/// A *node* is one spot in the tree. It stores one value, such as the sum of
/// the elements under it. Say you change a whole range: "add 10 to elements
/// `1..4`". The tree does not visit every element. It fixes the value of a few
/// nodes right away, and on each of them it writes a note: "my value is
/// already right, but my children have not heard about this yet". That note is
/// the tag. It is handed down to the children later, and only if something
/// needs to look inside the node.
///
/// ```text
/// node 3: sum 25   [note: +10]     <- value already fixed; the note is the tag
///    /        \
///  leaf 4    leaf 1                <- still the old values, they haven't heard yet
/// ```
///
/// See the [module docs](self#range-updates-lazy-tags) for the whole picture.
///
/// You write one small type for each kind of change, for example `Add(i64)`
/// for "add this number to every element". Then you teach the tree three
/// things about it:
///
/// # The three methods
///
/// | method                       | question it answers             | for "add `x`"    |
/// | ---------------------------- | ------------------------------- | ---------------- |
/// | [`is_empty`](Self::is_empty) | is this note "do nothing"?      | `x == 0`         |
/// | [`apply_to`](Self::apply_to) | what is the node's new value?   | `sum + x * size` |
/// | [`compose`](Self::compose)   | two notes in a row, as one note | `x + y`          |
///
/// [`apply_to`](Self::apply_to) is also told how many elements the node holds
/// (`size`), because many changes need it: adding `x` to every element adds
/// `x * size` to a sum.
///
/// # Some tags you might write
///
/// | the change                     | each node stores | can `apply_to` say `None`? |
/// | ------------------------------ | ---------------- | -------------------------- |
/// | add `x` to every element       | the sum          | never                      |
/// | set every element to `x`       | the sum          | never                      |
/// | cap every element at `x`       | sum and maximum  | yes (example 1)            |
/// | replace each element by `f(e)` | sum and maximum  | yes (example 2)            |
///
/// The first two are simple: the new sum can always be worked out from the old
/// sum and `size`, so `apply_to` always says `Some`. The last two cannot: the
/// new sum depends on the individual elements. They are explained below, in
/// [Tags that can say no](#tags-that-can-say-no):
/// [example 1](#example-1-cap-every-element-at-x) and
/// [example 2](#example-2-a-tag-that-never-leaves-a-note).
///
/// # Laws
///
/// These are the rules the tree relies on. The compiler can't check them. If
/// you break one, answers come out wrong, and only sometimes (it depends on
/// when a note happens to be pushed down), so a few tests may not notice.
///
/// Write `t(v)` for "what `t.apply_to(v, l, r, size)` returns" on a node. The
/// three rules below are about the case where it returns `Some`. For `None`,
/// see [Tags that can say no](#tags-that-can-say-no).
///
/// 1. **The empty note.** Every node starts with `T::default()` and nothing
///    waiting. So `T::default().is_empty()` must be `true`. And putting a new
///    tag `t` on top of the empty note must give back `t`:
///    `t.compose(&T::default())` does the same as `t`. For "add" the empty
///    note is `Add(0)`: it adds nothing, and `Add(x)` on top of it is
///    `Add(x + 0)`.
/// 2. **Two notes in a row.** A node holds only one note, so when a second
///    one arrives the tree merges them with [`compose`](Self::compose).
///    `a.compose(&b)` means "first `b`, then `a`":
///    `a.compose(&b)(v) == a(b(v))`. The tree always calls it as
///    `new_note.compose(&old_note)`, because the old note was there first.
///    For "add" the order doesn't matter (`x + y == y + x`). For "set to `x`"
///    it does: the later one wins, so `a.compose(&b)` is just `a`.
/// 3. **A node is the sum of its parts.** Say a node holds `s` elements, its
///    left child holds `s1`, its right child holds `s2` (so `s = s1 + s2`),
///    and the children's values are `a` and `b`. The node's value is
///    `combine(a, b)`. Applying `t` to the node must give the same result as
///    applying `t` to each child (each with its own size) and then combining:
///    `t(combine(a, b)) == combine(t(a), t(b))`. This matters because the node
///    is fixed *now* but its children only *later*, when the note is pushed
///    down. Both ways must agree. For "add `x`" and sum:
///    `(a + b) + x * s == (a + x * s1) + (b + x * s2)`, which is true because
///    `s = s1 + s2`.
///
/// About [`is_empty`](Self::is_empty): the tree asks it before pushing a note
/// down. `true` means "throw this note away". So for an ordinary tag, say
/// `true` only when the note really does nothing (like `Add(0)`). Saying
/// `false` for a note that does nothing is safe, just a little slower. Saying
/// `true` for a note that does something loses that change. (One kind of tag
/// is allowed to always say `true`, see
/// [example 2](#example-2-a-tag-that-never-leaves-a-note).)
///
/// # Tags that can say no
///
/// Some changes can't be done on a node's value alone. "Cap every element at
/// 4" on a node whose sum is 20: how much is the new sum? It depends on how
/// many elements are above 4, and the sum can't tell you.
///
/// For these, [`apply_to`](Self::apply_to) returns `None`, which means "I can't
/// tell from this node's value, look inside". Then the tree does this:
///
/// ```text
/// apply_to says None for node P:
///   1. push P's old note down      (so P's children are up to date)
///   2. give the new tag to P's left child and to P's right child
///      (if a child says None too, it does steps 1 to 3 itself)
///   3. recompute P from its children
/// ```
///
/// A leaf has nothing inside to look at, so a leaf must **never** say `None`:
/// the tree panics. On a leaf (`size == 1`) the value is the single element,
/// so the change can always be done.
///
/// To avoid looking inside when it isn't needed, `apply_to` should return
/// `Some` for a node whenever it can: for example, when the change would do
/// nothing to the elements under it.
///
/// ## Example 1: cap every element at `x`
///
/// "Cap at `x`" turns each element `e` into `min(e, x)`. Each node stores the
/// biggest element (`mx`) and the sum. `apply_to` has three cases:
///
/// | node                    | answer                 | why                                 |
/// | ----------------------- | ---------------------- | ----------------------------------- |
/// | a leaf (`size == 1`)    | `Some`, capped element | the one element is known            |
/// | has children, `mx <= x` | `Some`, unchanged      | no element above `x`: no change     |
/// | has children, `mx > x`  | `None`                 | some element above `x`: look inside |
///
/// Here is `Cap(4)` on the elements `[3, 1, 9, 4]`:
///
/// ```text
///                       root: max 9 > 4 -> None, look inside
///                    /                          \
///   node 2: max 3 <= 4                  node 3: max 9 > 4 -> None, look inside
///   -> Some, unchanged, stop            /                      \
///                                   leaf 9                    leaf 4
///                                   size 1                    size 1
///                                   min(9, 4) = 4             min(4, 4) = 4
///
/// result: [3, 1, 4, 4]
/// ```
///
/// This tag is ordinary: it keeps its note, and it follows all three laws.
/// The note `Cap(4)` stays on node 2, saying "nobody under you is above 4".
/// That is why [`set`](SegmentTree::set) pushes notes down before it writes a
/// leaf: otherwise the old `Cap(4)` would later land on a new, bigger value.
/// Its empty note is `Cap(u64::MAX)`, a cap that no element reaches, and two
/// caps compose into the smaller one.
///
/// **Cost.** This version is correct, but it can be slow: if every element
/// under a node equals the maximum, one cap visits every one of them. The full
/// trick (often called *segment tree beats*) also stores the second-biggest
/// value in each node, so a node whose biggest elements are all above the cap
/// can be fixed in one step. That needs a bigger `Node`; the idea is the same
/// as above.
///
/// ```
/// # use algo_lib::collections::segment_tree::{SegmentTree, Tag};
/// # use algo_lib::math::type_classes::Monoid;
/// /// What each node stores: the biggest element and the sum.
/// #[derive(Clone, Copy)]
/// struct Node {
///     mx: u64,
///     sum: u64,
/// }
///
/// struct MaxSum;
/// impl Monoid for MaxSum {
///     type T = Node;
///     fn empty() -> Node {
///         Node { mx: 0, sum: 0 }
///     }
///     fn combine(a: Node, b: Node) -> Node {
///         Node { mx: a.mx.max(b.mx), sum: a.sum + b.sum }
///     }
/// }
///
/// /// "Cap every element at this number."
/// #[derive(Clone)]
/// struct Cap(u64);
/// impl Default for Cap {
///     fn default() -> Self {
///         Cap(u64::MAX) // the empty note: a cap nobody reaches
///     }
/// }
/// impl Tag<MaxSum> for Cap {
///     fn is_empty(&self) -> bool {
///         self.0 == u64::MAX
///     }
///     fn apply_to(&self, x: Node, _l: usize, _r: usize, size: usize) -> Option<Node> {
///         if size == 1 {
///             let e = x.mx.min(self.0); // a leaf: the one element, capped
///             Some(Node { mx: e, sum: e })
///         } else if x.mx <= self.0 {
///             Some(x) // nobody is above the cap: nothing changes
///         } else {
///             None // somebody is above the cap: look inside
///         }
///     }
///     fn compose(&self, other: &Self) -> Self {
///         Cap(self.0.min(other.0)) // two caps in a row: the smaller one wins
///     }
/// }
///
/// let xs = [3, 1, 9, 4];
/// let mut t: SegmentTree<MaxSum, Cap> =
///     SegmentTree::from_fn(4, |i| Node { mx: xs[i], sum: xs[i] });
///
/// t.modify(.., &Cap(4)); // [3, 1, 4, 4]
/// let all = t.query(..);
/// assert_eq!((all.mx, all.sum), (4, 12));
///
/// t.set(0, Node { mx: 100, sum: 100 }); // [100, 1, 4, 4]
/// let first = t.get(0);
/// assert_eq!((first.mx, first.sum), (100, 100)); // the old `Cap(4)` did not touch it
///
/// t.modify(..2, &Cap(50)); // [50, 1, 4, 4]
/// let all = t.query(..);
/// assert_eq!((all.mx, all.sum), (50, 59));
/// ```
///
/// ## Example 2: a tag that never leaves a note
///
/// "Replace every element by its number of divisors." Write `d(e)` for that:
/// `d(1) = 1`, `d(2) = 2`, `d(6) = 4`. Take elements that are at least 1, and
/// the same `Node` and `MaxSum` as in example 1. `apply_to` again has three
/// cases:
///
/// | node                    | answer               | why                                     |
/// | ----------------------- | -------------------- | --------------------------------------- |
/// | a leaf (`size == 1`)    | `Some`, `d(element)` | the one element is known                |
/// | has children, `mx <= 2` | `Some`, unchanged    | all elements are 1 or 2: `d` keeps them |
/// | has children, `mx > 2`  | `None`               | some element changes: look inside       |
///
/// Every `modify` goes down to the leaves that really change, and does all the
/// work on the way. Nothing is left waiting, so **no note is ever needed**.
/// That has a clear consequence for the methods:
///
/// - [`is_empty`](Self::is_empty) always returns `true`. This is allowed here,
///   and it does not lose any change. The tree only keeps a note on a node
///   after `apply_to` said `Some` for it. For this tag, `Some` on a node with
///   children happens only when the change does nothing to anything under it
///   (`mx <= 2`). So the note could be thrown away without anyone noticing.
/// - [`compose`](Self::compose) just returns `self.clone()`. It is never
///   needed, since no note is ever pushed.
/// - `T::default()` is only a placeholder here. It is never applied, so it does
///   not have to be "do nothing".
///
/// The rule for this kind of tag is: **`apply_to` may say `Some` for a node
/// with children only if the change would do nothing to anything under that
/// node.** Everything else must either be a leaf (do the change) or `None`.
///
/// After each round every element `e` becomes `d(e)`, and numbers fall fast.
/// Starting from any number up to 1,000,000, it takes at most 6 rounds to
/// reach 1 or 2 (5040 is the slowest, with 6). A node whose maximum is 1 or 2
/// is then skipped at once, so `modify` gets cheap after a few rounds.
///
/// ```
/// # use algo_lib::collections::segment_tree::{SegmentTree, Tag};
/// # use algo_lib::math::type_classes::Monoid;
/// # #[derive(Clone, Copy)]
/// # struct Node {
/// #     mx: u64,
/// #     sum: u64,
/// # }
/// # struct MaxSum;
/// # impl Monoid for MaxSum {
/// #     type T = Node;
/// #     fn empty() -> Node {
/// #         Node { mx: 0, sum: 0 }
/// #     }
/// #     fn combine(a: Node, b: Node) -> Node {
/// #         Node { mx: a.mx.max(b.mx), sum: a.sum + b.sum }
/// #     }
/// # }
/// /// The number of divisors of `e`, by trying every number up to `e`.
/// fn d(e: u64) -> u64 {
///     (1..=e).filter(|k| e % k == 0).count() as u64
/// }
///
/// /// "Replace every element by its number of divisors."
/// #[derive(Clone, Default)]
/// struct ReplaceByD;
/// impl Tag<MaxSum> for ReplaceByD {
///     fn is_empty(&self) -> bool {
///         true // no note is ever needed
///     }
///     fn apply_to(&self, x: Node, _l: usize, _r: usize, size: usize) -> Option<Node> {
///         if size == 1 {
///             let e = d(x.mx); // a leaf: `mx` is the one element
///             Some(Node { mx: e, sum: e })
///         } else if x.mx <= 2 {
///             Some(x) // all 1s and 2s: nothing changes
///         } else {
///             None // some element changes: look inside
///         }
///     }
///     fn compose(&self, _: &Self) -> Self {
///         ReplaceByD // never needed
///     }
/// }
///
/// let xs = [2, 1, 6, 4];
/// let mut t: SegmentTree<MaxSum, ReplaceByD> =
///     SegmentTree::from_fn(4, |i| Node { mx: xs[i], sum: xs[i] });
///
/// t.modify(.., &ReplaceByD); // [2, 1, 4, 3]: d(6) = 4, d(4) = 3
/// let all = t.query(..);
/// assert_eq!((all.mx, all.sum), (4, 10));
///
/// t.modify(2.., &ReplaceByD); // [2, 1, 3, 2]: d(4) = 3, d(3) = 2
/// let all = t.query(..);
/// assert_eq!((all.mx, all.sum), (3, 8));
/// ```
pub trait Tag<M: Monoid>: Clone + Default {
    /// `true` for a real tag, so the tree keeps a note slot for every node.
    ///
    /// Leave it at the default. [`NoTag`] sets it to `false`, which makes the
    /// tree skip all note work and not allocate the note slots.
    const LAZY: bool = true;

    /// `true` means "this note can be thrown away": the tree then skips
    /// pushing it down.
    ///
    /// Must be `true` for `Self::default()`. For an ordinary tag, return `true`
    /// only if the note does nothing (like `Add(0)`); see the
    /// [laws](Tag#laws).
    fn is_empty(&self) -> bool;

    /// The new value of a node whose old value is `value`, after this change.
    ///
    /// The node holds `size` elements, the run `[l, r)`.
    ///
    /// Use `size` to count the node's elements. Do **not** compute it as
    /// `r - l`: with [`CircularLayout`](crate::collections::heap_layout::CircularLayout)
    /// a run can wrap from the last element to `0`, and then `r <= l`.
    /// `size` comes from the layout, so it is right for every layout. Use `l`
    /// and `r` only if your change depends on where the elements are.
    ///
    /// Return `None` if the new value can't be worked out from `value` alone:
    /// the tree then looks inside the node, see
    /// [Tags that can say no](Tag#tags-that-can-say-no). A leaf (`size == 1`)
    /// must never return `None`: the tree panics.
    fn apply_to(&self, value: M::T, l: usize, r: usize, size: usize) -> Option<M::T>;

    /// The one tag that means "first `other`, then `self`".
    ///
    /// Used when a node already waits on a note `other` and gets a newer note
    /// `self`: they are merged into one, so a node never holds more than one.
    /// See the [laws](Tag#laws).
    fn compose(&self, other: &Self) -> Self;
}

/// "No tag": a segment tree with only point updates ([`SegmentTree::set`]) and
/// no range updates.
///
/// It is the default `T` of [`SegmentTree`]. It has [`Tag::LAZY`] `= false`,
/// so the tree stores no notes and skips every push. Calling
/// [`modify`](SegmentTree::modify) on such a tree does not compile.
#[derive(Clone, Copy, Default)]
pub struct NoTag;

impl<M: Monoid> Tag<M> for NoTag {
    const LAZY: bool = false;

    fn is_empty(&self) -> bool {
        true
    }

    fn apply_to(&self, value: M::T, _: usize, _: usize, _: usize) -> Option<M::T> {
        Some(value)
    }

    fn compose(&self, _: &Self) -> Self {
        NoTag
    }
}

/// Answers "what do elements `l..r` combine to?" and updates elements, in
/// `O(log n)`; see the [module docs](self).
///
/// - `M`: what is combined, a [`Monoid`].
/// - `T`: the range-update [`Tag`], or [`NoTag`] (default) for none.
/// - `L`: the [`HeapLayout`], default [`InOrderLayout`].
///
/// Ranges are any Rust range of `usize`: `a..b`, `a..=b`, `..b`, `a..`, `..`.
/// An end past the last element is cut to `n`.
///
/// Reads take `&mut self` because a lazy tree may push notes down while
/// reading. For a tree without a tag nothing is pushed, but the signature is
/// the same.
pub struct SegmentTree<M: Monoid, T: Tag<M> = NoTag, L: HeapLayout = InOrderLayout> {
    /// Where element `i` lives and which elements a node is in charge of.
    layout: L,
    /// The value of every node, indexed by [`HeapPoint`]. Slot `0` is unused.
    /// Nodes `n..2n` are the leaves, one per element; nodes `1..n` combine
    /// their two children.
    values: Box<[M::T]>,
    /// The note waiting on each node `1..n`, meaning "my two children have not
    /// heard of this yet". Slot `0` is unused. The node's own value already
    /// includes the note. Empty when the tag is not lazy.
    tags: Box<[T]>,
}

/// A tree with element `i` equal to `values[i]`. Takes `O(n)`.
///
/// # Panics
///
/// If `values.len() > 2^30`.
impl<M: Monoid, T: Tag<M>, L: HeapLayout> From<&[M::T]> for SegmentTree<M, T, L> {
    fn from(values: &[M::T]) -> Self {
        Self::from_fn(values.len(), |i| values[i])
    }
}

impl<M: Monoid, T: Tag<M>, L: HeapLayout> SegmentTree<M, T, L> {
    /// A tree of `n` elements, all [`M::empty()`](Monoid::empty).
    ///
    /// # Panics
    ///
    /// If `n > 2^30`.
    pub fn new(n: usize) -> Self {
        Self::from_fn(n, |_| M::empty())
    }

    /// A tree of `n` elements, where element `i` is `f(i)`. Takes `O(n)`,
    /// faster than `n` calls to [`set`](Self::set).
    ///
    /// # Panics
    ///
    /// If `n > 2^30`.
    pub fn from_fn(n: usize, mut f: impl FnMut(usize) -> M::T) -> Self {
        let layout = L::new(n);
        let mut values = vec![M::empty(); 2 * n].into_boxed_slice();
        for i in 0..n {
            values[layout.point(i)] = f(i);
        }
        let tags = vec![T::default(); n].into_boxed_slice();
        let mut tree = Self {
            layout,
            values,
            tags,
        };
        // Every node needs its children's values, so go from the bottom up.
        for i in (1..n).rev() {
            tree.pull(HeapPoint::new(i as u32).unwrap());
        }
        tree
    }

    /// The number of elements, `n`.
    pub fn len(&self) -> usize {
        self.layout.len()
    }

    /// `true` if there are no elements.
    pub fn is_empty(&self) -> bool {
        self.layout.is_empty()
    }

    /// Element `i`.
    ///
    /// Pushes the notes above leaf `i` down first, so the leaf is up to date.
    ///
    /// # Panics
    ///
    /// If `i >= n`.
    pub fn get(&mut self, i: usize) -> M::T {
        let p = self.layout.point(i);
        self.push_along(p.ancestors_down());
        self.values[p]
    }

    /// Overwrites element `i` with `value`.
    ///
    /// Pushes the notes above leaf `i` down first (otherwise they would later
    /// land on the new value), writes the leaf, then recomputes its ancestors.
    ///
    /// # Panics
    ///
    /// If `i >= n`.
    pub fn set(&mut self, i: usize, value: M::T) {
        let p = self.layout.point(i);
        self.push_along(p.ancestors_down());
        self.values[p] = value;
        p.ancestors_up().for_each(|q| self.pull(q))
    }

    /// Walks over the pieces of `bounds`, left to right, with your own
    /// accumulator.
    ///
    /// The range is cut into the fewest nodes that hold exactly its elements
    /// (at most `2 log n`, see
    /// [`HeapRange::cover`](crate::collections::heap_range::HeapRange::cover)).
    /// `f` gets the accumulator and the **combined value of one piece**, not
    /// of one element. For plain answers use [`query`](Self::query), which is
    /// this with `combine`.
    ///
    /// ```text
    /// elements:  [3  1  4  1]          fold(1..4, ..): the pieces are
    ///             0  1  2  3           the leaf of element 1 (value 1)
    ///               [-------]          and the node of 2, 3 (value 5)
    /// ```
    ///
    /// ```
    /// use algo_lib::collections::segment_tree::SegmentTree;
    /// use algo_lib::math::type_classes::Monoid;
    ///
    /// struct Sum;
    /// impl Monoid for Sum {
    ///     type T = i64;
    ///     fn empty() -> i64 { 0 }
    ///     fn combine(a: i64, b: i64) -> i64 { a + b }
    /// }
    ///
    /// let xs = [3, 1, 4, 1];
    /// let mut t: SegmentTree<Sum> = SegmentTree::from(&xs[..]);
    /// let pieces = t.fold(1..4, Vec::new(), |mut v, x| { v.push(x); v });
    /// assert_eq!(pieces, [1, 5]);
    /// ```
    ///
    /// # Panics
    ///
    /// If the start of `bounds` is after its end. An end past `n` is cut to
    /// `n` first, so a start past `n` panics.
    pub fn fold<S>(
        &mut self,
        bounds: impl RangeBounds<usize>,
        init: S,
        mut f: impl FnMut(S, M::T) -> S,
    ) -> S {
        let rng = self.prepare(bounds);
        rng.cover_l_to_r()
            .fold(init, |acc, p| f(acc, self.values[p]))
    }

    /// Like [`fold`](Self::fold), but the pieces come right to left.
    ///
    /// For the example on `fold`, `f` sees `5` first, then `1`.
    ///
    /// # Panics
    ///
    /// Same as [`fold`](Self::fold).
    pub fn rfold<S>(
        &mut self,
        bounds: impl RangeBounds<usize>,
        init: S,
        mut f: impl FnMut(S, M::T) -> S,
    ) -> S {
        let rng = self.prepare(bounds);
        rng.cover_l_to_r()
            .rev()
            .fold(init, |acc, p| f(acc, self.values[p]))
    }

    /// The combination of the elements in `bounds`, in order:
    /// `e_l · e_{l+1} · ... · e_{r-1}`, where `·` is [`Monoid::combine`].
    ///
    /// An empty range gives [`Monoid::empty`].
    ///
    /// # Panics
    ///
    /// Same as [`fold`](Self::fold).
    pub fn query(&mut self, bounds: impl RangeBounds<usize>) -> M::T {
        self.fold(bounds, M::empty(), M::combine)
    }

    /// Applies the tag `t` to every element in `bounds`, for example "add `x`
    /// to all of them".
    ///
    /// Leaves the note on the nodes that cover the range and fixes the nodes
    /// above them; see [the module docs](self#range-updates-lazy-tags).
    ///
    /// Only for trees with a real tag. With [`NoTag`] this fails to compile.
    ///
    /// # Panics
    ///
    /// Same as [`fold`](Self::fold).
    pub fn modify(&mut self, bounds: impl RangeBounds<usize>, t: &T) {
        const { assert!(T::LAZY, "modify needs a lazy Tag, not NoTag.") };
        let rng = self.prepare(bounds);
        rng.cover().for_each(|p| self.apply(p, t));
        rng.boundary_up().for_each(|p| self.pull(p));
    }

    /// Turns `bounds` into a tree range, and pushes the notes above it down so
    /// that every node of its cover is up to date.
    fn prepare(&mut self, bounds: impl RangeBounds<usize>) -> HeapRange {
        let (l, r) = clamp(&bounds, self.len());
        debug_assert!(
            l <= r,
            "range start {l} is after its end {r} (length {})",
            self.len()
        );
        self.prepare_range(l..r)
    }

    /// [`prepare`](Self::prepare) for an already-resolved `l..r`.
    fn prepare_range(&mut self, r: Range<usize>) -> HeapRange {
        let rng = self.layout.range(r);
        self.push_along(rng.boundary_down());
        rng
    }

    /// `true` if `p` is an inner node (it has children), `false` for a leaf.
    fn is_inner(&self, p: HeapPoint) -> bool {
        p.i() < self.len()
    }

    /// Recomputes `p` from its two children (which must be up to date).
    fn pull(&mut self, p: HeapPoint) {
        self.values[p] = M::combine(self.values[p.left()], self.values[p.right()])
    }

    /// Hands the note on `p` to both its children, and clears it.
    /// Does nothing if `p` has no note. `p` must be an inner node.
    fn push(&mut self, p: HeapPoint) {
        if T::LAZY && !self.tags[p].is_empty() {
            let t = std::mem::take(&mut self.tags[p]);
            self.apply(p.left(), &t);
            self.apply(p.right(), &t);
        }
    }

    /// [`push`](Self::push) every node on `path`, in the order given. Pass the
    /// path top-down, so a note keeps moving down.
    fn push_along(&mut self, path: impl Iterator<Item = HeapPoint>) {
        if T::LAZY {
            path.for_each(|p| self.push(p));
        }
    }

    /// Applies tag `t` to node `p`: fixes its value now, and leaves `t` as a
    /// note for its children (merged with the note already there).
    ///
    /// If [`Tag::apply_to`] says `None`, the value can't be fixed from the
    /// node alone. Then the old note goes down first, `t` is applied to both
    /// children (this may recurse), and `p` is recomputed from them.
    fn apply(&mut self, p: HeapPoint, t: &T) {
        let (l, r) = self.layout.node_bounds(p);
        match t.apply_to(self.values[p], l, r, self.layout.node_size(p)) {
            Some(value) => {
                self.values[p] = value;
                if self.is_inner(p) {
                    self.tags[p] = t.compose(&self.tags[p]);
                }
            }
            None => {
                assert!(self.is_inner(p), "Tag::apply_to must succeed on a leaf");
                self.push(p);
                self.apply(p.left(), t);
                self.apply(p.right(), t);
                self.pull(p);
            }
        }
    }
}

/// Searches. These walk down the tree, left or right at each node, so they
/// need [`InOrder`]: the left child must hold the smaller elements. That is
/// [`InOrderLayout`], the default.
impl<M: Monoid, T: Tag<M>, L: InOrder> SegmentTree<M, T, L> {
    /// How far right can the range starting at `l` grow before `pred` turns
    /// false?
    ///
    /// Returns the largest `r` (`l <= r <= n`) such that
    /// `pred(self.query(l..r))` is true. When `r < n`, element `r` is the one
    /// that breaks it: `pred(self.query(l..r + 1))` is false. When `pred`
    /// never fails, the answer is `n`.
    ///
    /// ```text
    /// element:      0  1  2  3  4          pred = "sum <= 5", l = 0
    /// value:        3  1  4  1  5
    /// sum from 0:   3  4  8
    ///               ok ok no               -> r = 2
    /// ```
    ///
    /// `pred` must be **monotone**: once it is false for a range, it is false
    /// for every longer range that starts at `l`. And it must be true for
    /// [`Monoid::empty`], the range `l..l`. ("Sum is at most `k`" is monotone
    /// for non-negative elements; "sum is even" is not.)
    ///
    /// # How it works
    ///
    /// Take the pieces of `l..n` left to right, growing a running answer
    /// `agg`. At the first piece that breaks `pred`, go inside it: if the left
    /// child still fits, add it to `agg` and continue in the right child,
    /// otherwise continue in the left one. When you reach a leaf, that leaf is
    /// element `r`. That is one walk down: `O(log n)`.
    ///
    /// ```
    /// use algo_lib::collections::segment_tree::SegmentTree;
    /// use algo_lib::math::type_classes::Monoid;
    ///
    /// struct Sum;
    /// impl Monoid for Sum {
    ///     type T = i64;
    ///     fn empty() -> i64 { 0 }
    ///     fn combine(a: i64, b: i64) -> i64 { a + b }
    /// }
    ///
    /// let xs = [3, 1, 4, 1, 5];
    /// let mut t: SegmentTree<Sum> = SegmentTree::from(&xs[..]);
    /// assert_eq!(t.max_right(0, |s| s <= 5), 2); // 3 + 1 = 4, then 4 breaks it
    /// assert_eq!(t.max_right(2, |s| s <= 5), 4); // 4 + 1 = 5, then 5 breaks it
    /// assert_eq!(t.max_right(0, |s| s <= 100), 5); // never breaks: n
    /// assert_eq!(t.max_right(5, |s| s <= 0), 5); // nothing to the right: n
    /// ```
    ///
    /// # Panics
    ///
    /// If `l > n`.
    pub fn max_right(&mut self, l: usize, mut pred: impl FnMut(M::T) -> bool) -> usize {
        let n = self.len();
        debug_assert!(l <= n, "max_right: l = {l} out of bounds for length {n}");
        let rng = self.prepare_range(l..n);
        let mut agg = M::empty();
        for mut p in rng.cover_l_to_r() {
            let next = M::combine(agg, self.values[p]);
            if pred(next) {
                agg = next;
                continue;
            }
            // `p` breaks `pred`: the answer is inside it.
            while self.is_inner(p) {
                self.push(p);
                let next = M::combine(agg, self.values[p.left()]);
                if pred(next) {
                    agg = next;
                    p = p.right();
                } else {
                    p = p.left();
                }
            }
            return self.layout.node_bounds(p).0;
        }
        n
    }

    /// The mirror of [`max_right`](Self::max_right): how far left can the
    /// range ending at `r` grow before `pred` turns false?
    ///
    /// Returns the smallest `l` (`0 <= l <= r`) such that
    /// `pred(self.query(l..r))` is true. When `l > 0`, element `l - 1` is the
    /// one that breaks it. When `pred` never fails, the answer is `0`.
    ///
    /// ```text
    /// element:      0  1  2  3  4          pred = "sum <= 6", r = 5
    /// value:        3  1  4  1  5
    /// sum up to 5:        10 6  5
    ///                     no ok ok         -> l = 3
    /// ```
    ///
    /// `pred` must be monotone (once false for a range, false for every longer
    /// range that ends at `r`) and true for [`Monoid::empty`].
    ///
    /// ```
    /// use algo_lib::collections::segment_tree::SegmentTree;
    /// use algo_lib::math::type_classes::Monoid;
    ///
    /// struct Sum;
    /// impl Monoid for Sum {
    ///     type T = i64;
    ///     fn empty() -> i64 { 0 }
    ///     fn combine(a: i64, b: i64) -> i64 { a + b }
    /// }
    ///
    /// let xs = [3, 1, 4, 1, 5];
    /// let mut t: SegmentTree<Sum> = SegmentTree::from(&xs[..]);
    /// assert_eq!(t.min_left(5, |s| s <= 6), 3); // 5 + 1 = 6, then 4 breaks it
    /// assert_eq!(t.min_left(5, |s| s <= 100), 0); // never breaks: 0
    /// assert_eq!(t.min_left(0, |s| s <= 0), 0); // nothing to the left: 0
    /// ```
    ///
    /// # Panics
    ///
    /// If `r > n`.
    pub fn min_left(&mut self, r: usize, mut pred: impl FnMut(M::T) -> bool) -> usize {
        debug_assert!(
            r <= self.len(),
            "min_left, r = {r} out of bounds for length {}",
            self.len()
        );
        let rng = self.prepare_range(0..r);
        let mut agg = M::empty();
        for mut p in rng.cover_l_to_r().rev() {
            let next = M::combine(self.values[p], agg);
            if pred(next) {
                agg = next;
                continue;
            }
            // `p` breaks `pred`: the answer is inside it.
            while self.is_inner(p) {
                self.push(p);
                let next = M::combine(self.values[p.right()], agg);
                if pred(next) {
                    agg = next;
                    p = p.left();
                } else {
                    p = p.right();
                }
            }
            return self.layout.node_bounds(p).1;
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::{SegmentTree, Tag};
    use crate::collections::heap_layout::{CircularLayout, HeapLayout, InOrderLayout};
    use crate::math::type_classes::Monoid;
    use crate::misc::random::{Random, Rng};

    struct Sum;

    impl Monoid for Sum {
        type T = i64;

        fn empty() -> i64 {
            0
        }

        fn combine(a: i64, b: i64) -> i64 {
            a + b
        }
    }

    /// "Add this to every element." Needs the node's size.
    #[derive(Clone, Default)]
    struct Add(i64);

    impl Tag<Sum> for Add {
        fn is_empty(&self) -> bool {
            self.0 == 0
        }

        fn apply_to(&self, sum: i64, _: usize, _: usize, size: usize) -> Option<i64> {
            Some(sum + self.0 * size as i64)
        }

        fn compose(&self, other: &Self) -> Self {
            Add(self.0 + other.0)
        }
    }

    /// Range add, range sum, get and set against a plain `Vec`.
    fn check<L: HeapLayout>() {
        // A fixed seed, so a failure can be replayed.
        let mut rng = Rng::from_seed(12345);
        for n in 0..=20usize {
            let mut model = vec![0i64; n];
            let mut tree: SegmentTree<Sum, Add, L> = SegmentTree::new(n);
            for _ in 0..200 {
                let a = rng.next_range(0..=n);
                let b = rng.next_range(0..=n);
                let (l, r) = (a.min(b), a.max(b));
                let x = rng.next_range(-10i64..=10);
                match rng.next_below(4usize) {
                    0 => {
                        tree.modify(l..r, &Add(x));
                        model[l..r].iter_mut().for_each(|e| *e += x);
                    }
                    1 if n > 0 => {
                        let i = rng.next_below(n);
                        tree.set(i, x);
                        model[i] = x;
                    }
                    2 if n > 0 => {
                        let i = rng.next_below(n);
                        assert_eq!(tree.get(i), model[i], "n={n} get({i})");
                    }
                    _ => {
                        let want: i64 = model[l..r].iter().sum();
                        assert_eq!(tree.query(l..r), want, "n={n} query({l}..{r})");
                    }
                }
            }
            assert_eq!(tree.query(..), model.iter().sum::<i64>(), "n={n} total");
        }
    }

    #[test]
    fn lazy_add_in_order() {
        check::<InOrderLayout>();
    }

    /// The nodes of `CircularLayout` can wrap (node 2 of `n = 5` holds
    /// elements 3, 4, 0), which is why the tree passes `size` to the tag.
    #[test]
    fn lazy_add_circular() {
        check::<CircularLayout>();
    }

    #[test]
    fn circular_wrapped_node_size() {
        let mut tree: SegmentTree<Sum, Add, CircularLayout> = SegmentTree::new(5);
        tree.modify(.., &Add(2));
        assert_eq!(tree.query(..), 10);
        assert_eq!(tree.query(3..5), 4);
        assert_eq!(tree.query(0..3), 6);
    }

    #[test]
    fn max_right_min_left_with_lazy_tags() {
        let xs = [3, 1, 4, 1, 5];
        let mut tree: SegmentTree<Sum, Add> = SegmentTree::from(&xs[..]);
        tree.modify(1..4, &Add(10)); // [3, 11, 14, 11, 5]
        assert_eq!(tree.max_right(0, |s| s <= 20), 2); // 3 + 11 = 14, then 14 breaks it
        assert_eq!(tree.min_left(5, |s| s <= 20), 3); // 5 + 11 = 16, then 14 breaks it
    }
}
