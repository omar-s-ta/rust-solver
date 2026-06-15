use std::{hash::Hash, iter::repeat_n, ops::Deref};

use crate::collections::hash_map::FxHashMap;

#[derive(Default)]
pub struct MultiHashSet<T> {
    map: FxHashMap<T, usize>,
    len: usize,
}

impl<T: Eq + Hash> MultiHashSet<T> {
    pub fn new() -> Self {
        Self {
            map: FxHashMap::default(),
            len: 0,
        }
    }

    pub fn insert(&mut self, elem: T) {
        *self.map.entry(elem).or_insert(0) += 1;
        self.len += 1;
    }

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

    pub fn remove_all(&mut self, elem: &T) -> bool {
        match self.map.remove(elem) {
            Some(count) => {
                self.len -= count;
                true
            }
            None => false,
        }
    }

    pub fn cardinality(&self) -> usize {
        self.map.len()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn contains(&self, elem: &T) -> bool {
        self.map.contains_key(elem)
    }

    pub fn count(&self, elem: &T) -> usize {
        self.map.get(elem).copied().unwrap_or_default()
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.len = 0;
    }

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
