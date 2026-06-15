use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Bound,
};

pub trait BoundedLookup<T> {
    type Output<'a>
    where
        Self: 'a;

    fn next_inclusive<'a>(&'a self, elem: &T) -> Option<Self::Output<'a>>;
    fn next_exclusive<'a>(&'a self, elem: &T) -> Option<Self::Output<'a>>;

    fn prev_inclusive<'a>(&'a self, elem: &T) -> Option<Self::Output<'a>>;
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
