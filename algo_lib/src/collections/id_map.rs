//! Interning: hand out a small dense `usize` for each distinct value, once.
//!
//! [`IdMap`] is the hash-based half of "replace awkward keys with array
//! indices". Feed it strings, pairs, whatever hashes, and it answers with
//! `0, 1, 2, …` — the same id every time for an equal value — while keeping the
//! values so you can go back. That id is what makes the rest of a solution
//! cheap: it indexes a `Vec` adjacency list, a `FenwickTree`, a `MdArray` row,
//! instead of hashing a `String` on every touch.
//!
//! ```
//! use algo_lib::collections::id_map::IdMap;
//!
//! let mut ids = IdMap::default();
//! assert_eq!(ids.intern(&"pear".to_string()), 0);
//! assert_eq!(ids.intern(&"apple".to_string()), 1);
//! assert_eq!(ids.intern(&"pear".to_string()), 0); // already seen, same id
//! assert_eq!(ids.len(), 2);
//!
//! // Both directions.
//! assert_eq!(ids.value(1), "apple");
//! assert_eq!(ids.lookup(&"apple".to_string()), Some(1));
//! assert_eq!(ids.lookup(&"plum".to_string()), None);
//!
//! // Same operation, one clone cheaper, for a value you are done with.
//! assert_eq!(ids.intern_owned("plum".to_string()), 2);
//! assert_eq!(ids.lookup(&"plum".to_string()), Some(2));
//!
//! // Ids are handed out in *first-seen* order, so `values()` is not sorted.
//! assert_eq!(
//!     ids.values(),
//!     &["pear".to_string(), "apple".to_string(), "plum".to_string()]
//! );
//! ```
//!
//! # This is not coordinate compression
//!
//! The trap worth naming: ids follow **first-seen order**, not value order, so
//! an id is *not* a rank. One value having a smaller id than another says
//! nothing about which value is smaller, and [`values`](IdMap::values) comes
//! back unsorted.
//!
//! When you need order — "how many values are less than this one", a Fenwick
//! tree over compressed coordinates, binary search — use the sorting route
//! instead: [`Dedup::sorted_deduped`](crate::collections::vec_ext::dedup::Dedup::sorted_deduped)
//! to get the distinct values ascending, then
//! [`SortedOps::lower_bound`](crate::collections::slice_ext::sorted_ops::SortedOps::lower_bound)
//! for the rank. That route needs `T: Ord` and every value up front; `IdMap`
//! needs only `Hash + Eq` and works online, one value at a time. Pick by
//! whether you need order or streaming.
//!
//! Ids are also permanent: nothing here removes, so an id stays valid for the
//! life of the map and [`values`](IdMap::values) is always dense over
//! `0..len()`.

use crate::collections::hash_map::FxHashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

/// A two-way map between values and dense ids `0..len()`.
///
/// Holds each distinct value twice — once as a hash key for value → id, once in
/// a `Vec` for id → value — which is what buys O(1) in both directions.
#[derive(Clone)]
pub struct IdMap<T> {
    ids: FxHashMap<T, usize>,
    values: Vec<T>,
}

/// The only constructor: `IdMap::default()` starts empty and allocates nothing
/// until the first value is interned.
///
/// Hand-written rather than derived, and it has to stay that way:
/// `#[derive(Default)]` bounds every type parameter, so it would demand
/// `T: Default` for no reason — an empty `FxHashMap` and an empty `Vec` exist
/// for every `T`. Do not put the derive back.
impl<T> Default for IdMap<T> {
    fn default() -> Self {
        Self {
            ids: Default::default(),
            values: Vec::new(),
        }
    }
}

impl<T: Hash + Eq> IdMap<T> {
    /// Returns the value that owns `id`.
    ///
    /// # Panics
    ///
    /// If `id >= self.len()`. Ids handed out by [`intern`](IdMap::intern) are
    /// always in range and stay that way, so this only fires on an id that came
    /// from somewhere else.
    pub fn value(&self, id: usize) -> &T {
        &self.values[id]
    }

    /// Returns every value, indexed by its own id — `values()[i]` is the value
    /// with id `i`.
    ///
    /// First-seen order, not sorted. `values().iter().enumerate()` is the way to
    /// walk every `(id, value)` pair.
    pub fn values(&self) -> &[T] {
        &self.values
    }

    /// Returns the id of `el` if it has been seen, without assigning one.
    ///
    /// The read-only counterpart to [`intern`](IdMap::intern); use it when a
    /// missing value means something other than "add it".
    pub fn lookup(&self, el: &T) -> Option<usize> {
        self.ids.get(el).copied()
    }

    /// Returns the number of distinct values, which is also the next id to be
    /// handed out.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns `true` if nothing has been interned yet.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// Interning is split out because it is the only operation that needs
/// [`Clone`]: it stores `el` on both sides of the map. Keeping the bound off the
/// block above means a non-cloneable `T` can still be queried — do not merge
/// these two impls back together.
impl<T: Hash + Eq + Clone> IdMap<T> {
    /// Returns the id of `el`, assigning the next free one if it is new.
    ///
    /// Idempotent: calling it again with an equal value returns the same id and
    /// changes nothing, so it doubles as "id, creating on demand".
    ///
    /// An already-present value costs one hash lookup and no allocation. A new
    /// one costs two lookups — one to find it missing, one to place it — and two
    /// clones of `el`, one for each side of the map. When you own the value and
    /// do not need it afterwards, [`intern_owned`](IdMap::intern_owned) beats
    /// that on both counts.
    ///
    /// The body is spelled out rather than delegating to
    /// [`intern_owned`](IdMap::intern_owned) on purpose: routing through it
    /// would re-probe the table a third time on a miss.
    pub fn intern(&mut self, el: &T) -> usize {
        if let Some(&i) = self.ids.get(el) {
            return i;
        }
        let id = self.values.len();
        self.values.push(el.clone());
        self.ids.insert(el.clone(), id);
        id
    }

    /// Returns the id of `el`, assigning the next free one if it is new, taking
    /// `el` by value.
    ///
    /// The cheaper half of [`intern`](IdMap::intern), on both axes. Going
    /// through the entry API finds the slot once and fills it in place, so this
    /// is **one** hash lookup whether the value is new or not, against two for a
    /// miss via [`intern`](IdMap::intern). And a new value is cloned once for the
    /// hash key then *moved* into the id → value side, so interning a `String`
    /// costs one allocation instead of two.
    ///
    /// The trade is that `el` is consumed either way. A value that turns out to
    /// be a duplicate is dropped and only its id comes back, so reach for this
    /// when you were going to discard `el` regardless — and for a value you
    /// still need, use [`intern`](IdMap::intern) rather than cloning to call
    /// this, which would put you back at two clones.
    pub fn intern_owned(&mut self, el: T) -> usize {
        let id = self.values.len();
        match self.ids.entry(el) {
            Entry::Occupied(seen) => *seen.get(),
            Entry::Vacant(slot) => {
                self.values.push(slot.key().clone());
                slot.insert(id);
                id
            }
        }
    }
}
