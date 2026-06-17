use std::{hash::Hash, iter::repeat_n, ops::Deref};

use crate::collections::hash_map::FxHashMap;

/// A multiset (bag) backed by a hash map, allowing duplicate elements.
///
/// Each distinct element is stored once together with its multiplicity, so the
/// memory used is proportional to the number of *distinct* elements rather than
/// the total number of insertions. Element lookups, insertions, and removals run
/// in expected O(1) time.
///
/// `Deref` exposes the underlying `FxHashMap<T, usize>` (element to count) for
/// read-only access to map methods.
#[derive(Default)]
pub struct MultiHashSet<T> {
    /// Maps each distinct element to its current multiplicity (always >= 1).
    map: FxHashMap<T, usize>,
    /// Total number of elements counted with multiplicity.
    len: usize,
}

impl<T: Eq + Hash> MultiHashSet<T> {
    /// Creates an empty `MultiHashSet`.
    pub fn new() -> Self {
        Self {
            map: FxHashMap::default(),
            len: 0,
        }
    }

    /// Inserts one occurrence of `elem`, incrementing its multiplicity.
    pub fn insert(&mut self, elem: T) {
        *self.map.entry(elem).or_insert(0) += 1;
        self.len += 1;
    }

    /// Removes a single occurrence of `elem`, dropping the entry entirely once
    /// its multiplicity reaches zero.
    ///
    /// Returns `true` if an occurrence was present and removed, `false` otherwise.
    pub fn remove(&mut self, elem: &T) -> bool {
        match self.map.get_mut(elem) {
            Some(count) => {
                *count -= 1;
                self.len -= 1;
                if *count == 0 {
                    self.map.remove(elem);
                }
                true
            }
            None => false,
        }
    }

    /// Removes all occurrences of `elem` at once.
    ///
    /// Returns `true` if the element was present, `false` otherwise.
    pub fn remove_all(&mut self, elem: &T) -> bool {
        match self.map.remove(elem) {
            Some(count) => {
                self.len -= count;
                true
            }
            None => false,
        }
    }

    /// Returns the number of *distinct* elements in the set.
    pub fn cardinality(&self) -> usize {
        self.map.len()
    }

    /// Returns the total number of elements counted with multiplicity.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns `true` if the set contains at least one occurrence of `elem`.
    pub fn contains(&self, elem: &T) -> bool {
        self.map.contains_key(elem)
    }

    /// Returns the multiplicity of `elem`, or `0` if it is absent.
    pub fn count(&self, elem: &T) -> usize {
        self.map.get(elem).copied().unwrap_or_default()
    }

    /// Removes all elements, resetting the set to empty.
    pub fn clear(&mut self) {
        self.map.clear();
        self.len = 0;
    }

    /// Iterates over all elements, yielding each one as many times as its
    /// multiplicity. The order is unspecified.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.map
            .iter()
            .flat_map(|(value, count)| repeat_n(value, *count))
    }
}

impl<T> Deref for MultiHashSet<T> {
    type Target = FxHashMap<T, usize>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<T: Eq + Hash> FromIterator<T> for MultiHashSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();
        for e in iter {
            set.insert(e);
        }
        set
    }
}
