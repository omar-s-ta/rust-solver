//! A dynamic multiset over a fixed integer key domain, with order statistics.
//!
//! [`OrderStatSet::new(size)`](OrderStatSet::new) fixes the key domain to
//! `0..size` — the integers `0` through `size - 1` inclusive, so `size` is a
//! *count of keys*, not the largest one. Within that domain the set holds any
//! multiset of keys, duplicates allowed, and answers the two questions a sorted
//! container is usually wanted for: how many stored keys fall below a value,
//! and which key sits at a given rank. Both in `O(log domain)`.
//!
//! It is a [`FenwickTree`] read as a frequency table: cell `v` counts the
//! copies of key `v` currently present. Everything follows from that. Insertion
//! and removal are `+1` and `-1` point updates, every counting query is a
//! prefix sum, and [`kth`](OrderStatSet::kth) is the tree's own descent.
//!
//! | operation                                | cost            |
//! | ---------------------------------------- | --------------- |
//! | `insert`, `remove`, `count`, `contains`  | `O(log domain)` |
//! | `rank`, `count_lte`, `count_in`, `kth`   | `O(log domain)` |
//! | `len`, `domain`                          | `O(1)`          |
//! | memory                                   | `O(domain)`     |
//!
//! # Assumptions
//!
//! These are the substance of the structure, not boilerplate:
//!
//! - **The key domain is fixed, integral, and known up front.** Memory is
//!   proportional to the *domain*, not to the number of stored elements, and a
//!   key outside `0..domain` cannot be stored at all. When the domain is
//!   unknown or huge, coordinate-compress the keys first (which requires having
//!   them offline), or use a dynamic segment tree or an order-statistic BST.
//! - **A key is an index.** That is precisely why insertion is cheap: storing
//!   one key never disturbs the position of another. The flip side is that
//!   *positional* insertion — placing a value between two others and
//!   renumbering the rest — is impossible here at any complexity, and needs a
//!   balanced BST carrying subtree sizes.
//! - **[`remove`](OrderStatSet::remove) must not run on an absent key**, which
//!   is why it is guarded and returns `bool` rather than `()`. A negative
//!   counter would destroy the monotonicity of the prefix sums that
//!   [`kth`](OrderStatSet::kth) descends, and the damage is silent: removing an
//!   absent `0` from `{2, 3}` leaves `kth(1) == 3` and `len() == 1`, both wrong
//!   and neither one a panic.
//!
//! Duplicates are allowed. For strict set semantics, guard
//! [`insert`](OrderStatSet::insert) with `count(v) == 0` exactly as
//! [`remove`](OrderStatSet::remove) is guarded.
//!
//! # Example
//!
//! ```
//! use algo_lib::collections::order_stat_set::OrderStatSet;
//!
//! let mut set = OrderStatSet::new(10);
//! for v in [3, 1, 4, 1, 5] {
//!     set.insert(v);
//! }
//!
//! assert_eq!(set.len(), 5);
//! assert_eq!(set.count(1), 2);
//! assert_eq!(set.rank(4), 3); // three keys below 4: 1, 1, 3
//! assert_eq!(set.kth(1), 1);  // ranks are 1-based, so this is the minimum
//! assert_eq!(set.kth(5), 5);
//!
//! assert!(set.remove(1));
//! assert_eq!(set.count(1), 1);
//! assert!(!set.remove(9)); // absent, so nothing changes
//! assert_eq!(set.len(), 4);
//! ```

use crate::{collections::fenwick_tree::FenwickTree, math::cast::Cast};

/// A multiset over the keys `0..domain` supporting rank and select in
/// `O(log domain)`.
///
/// See the [module documentation](self) for what the domain costs you and what
/// it buys.
pub struct OrderStatSet {
    /// Cell `v` counts the stored copies of key `v`.
    ft: FenwickTree<i32>,
    /// Running total, so [`len`](OrderStatSet::len) need not walk the tree.
    n: usize,
}

impl OrderStatSet {
    /// Creates an empty set whose keys are `0..size`, i.e. `0` through
    /// `size - 1` inclusive. `O(size)`.
    ///
    /// `size` is the number of representable keys, so a set built to hold keys
    /// up to and including `m` needs `new(m + 1)`. It fixes the domain for the
    /// set's whole life and bounds its memory; the set itself starts empty, so
    /// [`len`](Self::len) is `0` until something is inserted.
    pub fn new(size: usize) -> Self {
        Self {
            ft: FenwickTree::new(size),
            n: 0,
        }
    }

    /// Creates a set over the keys `0..size` already holding every key exactly
    /// once, so [`len`](Self::len) is `size` and `kth(k)` is `k - 1`. `O(size)`.
    ///
    /// Costs no more than [`new`](Self::new) and allocates no temporary — see
    /// [`FenwickTree::ones`]. This is what the deletion sieves want: start
    /// full, then [`remove`](Self::remove).
    pub fn full(size: usize) -> Self {
        Self {
            ft: FenwickTree::ones(size),
            n: size,
        }
    }

    /// The size of the key domain — the `size` given to [`new`](Self::new), so
    /// the valid keys are `0..domain()` and the largest is `domain() - 1`.
    /// `O(1)`.
    ///
    /// Fixed for the set's whole life, unlike [`len`](Self::len).
    pub fn domain(&self) -> usize {
        self.ft.len()
    }

    /// How many elements are stored, counting duplicates separately. `O(1)`.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.n
    }

    /// Adds one copy of `value`. `O(log domain)`.
    ///
    /// `value` must be in `0..domain()`, i.e. at most `domain() - 1`. Outside
    /// that range the key is silently dropped by the underlying tree while
    /// [`len`](Self::len) still advances, leaving the two permanently out of
    /// step — so debug builds assert instead, since nothing downstream can
    /// detect it afterwards.
    pub fn insert(&mut self, value: usize) {
        debug_assert!(
            value < self.domain(),
            "key {value} outside the domain 0..{}",
            self.domain()
        );
        self.ft.add(value, 1);
        self.n += 1;
    }

    /// Removes one copy of `value`, returning `false` and changing nothing if
    /// it is absent. `O(log domain)`.
    ///
    /// The guard is not politeness: see the third assumption in the [module
    /// documentation](self) for what an unguarded removal would do.
    pub fn remove(&mut self, value: usize) -> bool {
        if self.contains(value) {
            self.ft.add(value, -1);
            self.n -= 1;
            return true;
        }
        false
    }

    /// How many stored elements have keys strictly less than `value`, i.e. the
    /// rank `value` would take. `O(log domain)`.
    ///
    /// `rank(0)` is `0`, and a `value` past the domain gives
    /// [`len`](Self::len).
    pub fn rank(&self, value: usize) -> i32 {
        self.ft.get_to(value)
    }

    /// How many copies of `value` are stored; `0` if it is absent or outside
    /// the domain. `O(log domain)`.
    pub fn count(&self, value: usize) -> i32 {
        self.ft.get(value..=value)
    }

    /// How many stored elements have keys less than or equal to `value`.
    /// `O(log domain)`.
    pub fn count_lte(&self, value: usize) -> i32 {
        self.ft.get_to(value.saturating_add(1))
    }

    /// How many stored elements have keys in `a..=b`; `0` when `a > b`.
    /// `O(log domain)`.
    pub fn count_in(&self, a: usize, b: usize) -> i32 {
        self.ft.get(a..=b)
    }

    /// Whether `value` is stored at all. `O(log domain)`.
    pub fn contains(&self, value: usize) -> bool {
        self.count(value) > 0
    }

    /// The `k`-th smallest key, with `k` **1-based**: `kth(1)` is the minimum
    /// and `kth(len())` the maximum. `O(log domain)`.
    ///
    /// `k` is a rank, not an index — keys are 0-based, but ranks are counts, so
    /// they start at one. Returns [`domain()`](Self::domain), one past the last
    /// valid key, when `k` exceeds [`len`](Self::len), and `0` when `k` is `0`.
    pub fn kth(&self, k: usize) -> usize {
        self.ft.select(k.to())
    }
}

#[cfg(test)]
mod test {
    use crate::collections::order_stat_set::OrderStatSet;

    const DOMAIN: usize = 12;
    const KEYS: [usize; 9] = [3, 0, 7, 3, 11, 5, 7, 7, 0];

    /// The same multiset as a plain count-per-key array, as the reference.
    fn model() -> (OrderStatSet, [i32; DOMAIN]) {
        let mut set = OrderStatSet::new(DOMAIN);
        let mut counts = [0; DOMAIN];
        for &k in &KEYS {
            set.insert(k);
            counts[k] += 1;
        }
        (set, counts)
    }

    #[test]
    fn counting_queries_match_the_model() {
        let (set, counts) = model();
        assert_eq!(set.len(), KEYS.len());
        assert_eq!(set.domain(), DOMAIN);
        for v in 0..DOMAIN {
            assert_eq!(set.count(v), counts[v], "count({v})");
            assert_eq!(set.contains(v), counts[v] > 0, "contains({v})");
            assert_eq!(set.rank(v), counts[..v].iter().sum::<i32>(), "rank({v})");
            assert_eq!(
                set.count_lte(v),
                counts[..=v].iter().sum::<i32>(),
                "count_lte({v})"
            );
            for w in 0..DOMAIN {
                let want = if v > w { 0 } else { counts[v..=w].iter().sum() };
                assert_eq!(set.count_in(v, w), want, "count_in({v}, {w})");
            }
        }
    }

    #[test]
    fn kth_walks_the_multiset_in_order() {
        let (set, counts) = model();
        let sorted: Vec<usize> = (0..DOMAIN)
            .flat_map(|v| std::iter::repeat_n(v, counts[v] as usize))
            .collect();
        for (i, &want) in sorted.iter().enumerate() {
            assert_eq!(set.kth(i + 1), want, "kth({})", i + 1);
        }
        assert_eq!(set.kth(set.len() + 1), DOMAIN, "k past the size");
    }

    #[test]
    fn remove_is_guarded_and_keeps_len_in_step() {
        let (mut set, _) = model();
        assert!(!set.remove(1), "removing an absent key reports failure");
        assert_eq!(set.len(), KEYS.len(), "a failed remove changes nothing");
        assert!(set.remove(7));
        assert_eq!(set.count(7), 2);
        assert_eq!(set.len(), KEYS.len() - 1);
        for _ in 0..2 {
            assert!(set.remove(7));
        }
        assert!(!set.remove(7), "the last copy is gone");
        assert!(!set.contains(7));
        // [0, 0, 3, 3, 5, 11] remains once every 7 is gone.
        assert_eq!(set.kth(5), 5, "the 5th smallest skips the removed 7s");
        assert_eq!(set.kth(6), 11);
    }

    /// Release builds skip the assert, so this only holds with debug
    /// assertions on.
    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "outside the domain")]
    fn inserting_outside_the_domain_is_caught() {
        let mut set = OrderStatSet::new(4);
        set.insert(4);
    }

    #[test]
    fn full_holds_every_key_once() {
        for size in 0..=17usize {
            let set = OrderStatSet::full(size);
            assert_eq!(set.len(), size, "full({size}).len()");
            assert_eq!(set.domain(), size, "full({size}).domain()");
            for v in 0..size {
                assert_eq!(set.count(v), 1, "full({size}).count({v})");
                assert_eq!(set.rank(v), v as i32, "full({size}).rank({v})");
                assert_eq!(set.kth(v + 1), v, "full({size}).kth({})", v + 1);
            }
        }
    }

    /// The deletion-sieve shape: start full, then repeatedly drop a survivor.
    #[test]
    fn full_then_remove_renumbers_the_survivors() {
        let mut set = OrderStatSet::full(6);
        for v in [0, 3, 4] {
            assert!(set.remove(v));
        }
        assert_eq!(set.len(), 3);
        // 1, 2, 5 remain.
        assert_eq!(set.kth(1), 1);
        assert_eq!(set.kth(2), 2);
        assert_eq!(set.kth(3), 5);
        assert_eq!(set.kth(4), set.domain(), "no fourth survivor");
        assert_eq!(set.rank(5), 2);
    }

    #[test]
    fn queries_outside_the_domain_saturate() {
        let (set, _) = model();
        assert_eq!(set.count(DOMAIN + 5), 0);
        assert!(!set.contains(DOMAIN + 5));
        assert_eq!(set.rank(DOMAIN + 5), set.len() as i32);
        assert_eq!(set.count_in(DOMAIN, DOMAIN + 5), 0);
    }
}
