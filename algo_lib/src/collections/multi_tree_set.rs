use std::{
    collections::BTreeMap,
    iter::repeat_n,
    ops::{Deref, RangeBounds},
};

/// An ordered multiset (bag) backed by a `BTreeMap`, allowing duplicate elements.
///
/// Each distinct element is stored once together with its multiplicity, so the
/// memory used is proportional to the number of *distinct* elements rather than
/// the total number of insertions. Because the elements are kept sorted, this
/// set additionally supports ordered queries such as [`first`](Self::first),
/// [`last`](Self::last), [`range`](Self::range), and the popping operations,
/// while element lookups, insertions, and removals run in O(log n) time.
///
/// `Deref` exposes the underlying `BTreeMap<T, usize>` (element to count) for
/// read-only access to map methods.
#[derive(Default)]
pub struct MultiTreeSet<T> {
    /// Maps each distinct element to its current multiplicity (always >= 1).
    map: BTreeMap<T, usize>,
    /// Total number of elements counted with multiplicity.
    len: usize,
}

impl<T: Ord> MultiTreeSet<T> {
    /// Creates an empty `MultiTreeSet`.
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
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

    /// Iterates over all elements in ascending order, yielding each one as many
    /// times as its multiplicity.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.map
            .iter()
            .flat_map(|(value, count)| repeat_n(value, *count))
    }

    /// Iterates in ascending order over the elements within `range`, yielding
    /// each one as many times as its multiplicity.
    pub fn range(&self, range: impl RangeBounds<T>) -> impl Iterator<Item = &T> {
        self.map
            .range(range)
            .flat_map(|(value, count)| repeat_n(value, *count))
    }

    /// Iterates in descending order over the elements within `range`, yielding
    /// each one as many times as its multiplicity.
    pub fn range_rev(&self, range: impl RangeBounds<T>) -> impl Iterator<Item = &T> {
        self.map
            .range(range)
            .rev()
            .flat_map(|(value, count)| repeat_n(value, *count))
    }

    /// Returns the smallest element, or `None` if the set is empty.
    pub fn first(&self) -> Option<&T> {
        self.map.iter().next().map(|(elem, _)| elem)
    }

    /// Returns the largest element, or `None` if the set is empty.
    pub fn last(&self) -> Option<&T> {
        self.map.iter().next_back().map(|(elem, _)| elem)
    }
}

impl<T: Ord + Clone> MultiTreeSet<T> {
    /// Removes and returns the smallest element, or `None` if the set is empty.
    pub fn pop_first(&mut self) -> Option<T> {
        let (elem, count) = self.map.iter_mut().next()?;
        *count -= 1;
        self.len -= 1;
        let elem = elem.clone();
        if *count == 0 {
            self.map.remove(&elem);
        }
        Some(elem)
    }

    /// Removes and returns the largest element, or `None` if the set is empty.
    pub fn pop_last(&mut self) -> Option<T> {
        let (elem, count) = self.map.iter_mut().next_back()?;
        *count -= 1;
        self.len -= 1;
        let elem = elem.clone();
        if *count == 0 {
            self.map.remove(&elem);
        }
        Some(elem)
    }
}

impl<T> Deref for MultiTreeSet<T> {
    type Target = BTreeMap<T, usize>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<T: Ord> FromIterator<T> for MultiTreeSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();
        for e in iter {
            set.insert(e);
        }
        set
    }
}
