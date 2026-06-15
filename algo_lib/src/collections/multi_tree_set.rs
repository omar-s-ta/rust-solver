use std::{
    collections::BTreeMap,
    iter::repeat_n,
    ops::{Deref, RangeBounds},
};

#[derive(Default)]
pub struct MultiTreeSet<T> {
    map: BTreeMap<T, usize>,
    len: usize,
}

impl<T: Ord> MultiTreeSet<T> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
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

    pub fn range(&self, range: impl RangeBounds<T>) -> impl Iterator<Item = &T> {
        self.map
            .range(range)
            .flat_map(|(value, count)| repeat_n(value, *count))
    }

    pub fn first(&self) -> Option<&T> {
        self.map.iter().next().map(|(elem, _)| elem)
    }

    pub fn last(&self) -> Option<&T> {
        self.map.iter().next_back().map(|(elem, _)| elem)
    }
}

impl<T: Ord + Clone> MultiTreeSet<T> {
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
