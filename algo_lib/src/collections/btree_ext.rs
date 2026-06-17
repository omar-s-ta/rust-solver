use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Bound,
};

use crate::collections::multi_tree_set::MultiTreeSet;

/// Neighbor lookups relative to a query element on an ordered collection.
///
/// Each method finds the closest stored element on one side of `elem`,
/// optionally including a match equal to `elem` itself, in O(log n) time. This
/// is implemented for [`BTreeSet`], [`BTreeMap`] (where `Output` is a key/value
/// pair), and [`MultiTreeSet`].
pub trait BoundedLookup<T> {
    /// What a lookup yields: a reference to the element for set-like types, or a
    /// key/value pair for map-like types.
    type Output<'a>
    where
        Self: 'a;

    /// Returns the smallest stored element `>= elem`, or `None` if none exists.
    fn next_inclusive<'a>(&'a self, elem: &T) -> Option<Self::Output<'a>>;
    /// Returns the smallest stored element strictly `> elem`, or `None` if none exists.
    fn next_exclusive<'a>(&'a self, elem: &T) -> Option<Self::Output<'a>>;

    /// Returns the largest stored element `<= elem`, or `None` if none exists.
    fn prev_inclusive<'a>(&'a self, elem: &T) -> Option<Self::Output<'a>>;
    /// Returns the largest stored element strictly `< elem`, or `None` if none exists.
    fn prev_exclusive<'a>(&'a self, elem: &T) -> Option<Self::Output<'a>>;
}

impl<T: Ord> BoundedLookup<T> for BTreeSet<T> {
    type Output<'a>
        = &'a T
    where
        T: 'a;

    fn next_inclusive<'a>(&'a self, elem: &T) -> Option<Self::Output<'a>> {
        self.range(elem..).next()
    }

    fn next_exclusive<'a>(&'a self, elem: &T) -> Option<Self::Output<'a>> {
        self.range((Bound::Excluded(elem), Bound::Unbounded)).next()
    }

    fn prev_inclusive<'a>(&'a self, elem: &T) -> Option<Self::Output<'a>> {
        self.range(..=elem).next_back()
    }

    fn prev_exclusive<'a>(&'a self, elem: &T) -> Option<Self::Output<'a>> {
        self.range(..elem).next_back()
    }
}

impl<T: Ord> BoundedLookup<T> for MultiTreeSet<T> {
    type Output<'a>
        = &'a T
    where
        Self: 'a;

    fn next_inclusive<'a>(&'a self, elem: &T) -> Option<Self::Output<'a>> {
        self.range(elem..).next()
    }

    fn next_exclusive<'a>(&'a self, elem: &T) -> Option<Self::Output<'a>> {
        self.range((Bound::Excluded(elem), Bound::Unbounded)).next()
    }

    fn prev_inclusive<'a>(&'a self, elem: &T) -> Option<Self::Output<'a>> {
        self.range_rev(..=elem).next()
    }

    fn prev_exclusive<'a>(&'a self, elem: &T) -> Option<Self::Output<'a>> {
        self.range_rev(..elem).next()
    }
}

impl<K: Ord, V> BoundedLookup<K> for BTreeMap<K, V> {
    type Output<'a>
        = (&'a K, &'a V)
    where
        K: 'a,
        V: 'a;

    fn next_inclusive<'a>(&'a self, elem: &K) -> Option<Self::Output<'a>> {
        self.range(elem..).next()
    }

    fn next_exclusive<'a>(&'a self, elem: &K) -> Option<Self::Output<'a>> {
        self.range((Bound::Excluded(elem), Bound::Unbounded)).next()
    }

    fn prev_inclusive<'a>(&'a self, elem: &K) -> Option<Self::Output<'a>> {
        self.range(..=elem).next_back()
    }

    fn prev_exclusive<'a>(&'a self, elem: &K) -> Option<Self::Output<'a>> {
        self.range(..elem).next_back()
    }
}
