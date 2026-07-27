use std::cell::Cell;

use crate::{collections::slice_ext::indices::Indices, math::cast::Cast};

pub trait DisjointSet {
    fn new(n: usize) -> Self;
    fn find(&self, i: usize) -> usize;
    fn union(&mut self, a: usize, b: usize) -> bool;
    fn size(&self, i: usize) -> usize;
    fn sets_count(&self) -> usize;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct CompressedDisjointSet {
    parent: Vec<Cell<i32>>,
    count: usize,
}

impl DisjointSet for CompressedDisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: vec![Cell::new(-1); n],
            count: n,
        }
    }

    fn find(&self, i: usize) -> usize {
        let mut node = i;
        while let p = self.parent[node].get()
            && p >= 0
        {
            let p: usize = p.to();
            let gp = self.parent[p].get();
            if gp < 0 {
                return p;
            }
            self.parent[node].set(gp);
            node = gp.to();
        }
        node
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a == b {
            return false;
        }
        if self.parent[a].get() > self.parent[b].get() {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[a].set(self.parent[a].get() + self.parent[b].get());
        self.parent[b].set(a.to());
        self.count -= 1;
        true
    }

    fn size(&self, i: usize) -> usize {
        (-self.parent[self.find(i)].get()).to()
    }

    fn sets_count(&self) -> usize {
        self.count
    }

    fn len(&self) -> usize {
        self.parent.len()
    }
}

impl CompressedDisjointSet {
    pub fn iter(&self) -> impl Iterator<Item = usize> {
        self.parent
            .iter()
            .enumerate()
            .filter_map(|(i, id)| (id.get() < 0).then_some(i))
    }

    pub fn clear(&mut self) {
        self.count = self.parent.len();
        self.parent.fill(Cell::new(-1));
    }

    pub fn sets(&self) -> Vec<Vec<usize>> {
        let mut slot = vec![usize::MAX; self.len()];
        let mut res = Vec::<Vec<usize>>::with_capacity(self.count);
        for i in self.parent.indices() {
            let root = self.find(i);
            if slot[root] == usize::MAX {
                slot[root] = res.len();
                res.push(Vec::with_capacity((-self.parent[root].get()).to()));
            }
            res[slot[root]].push(i);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::CompressedDisjointSet;
    use super::DisjointSet;

    #[test]
    fn compressed_union_find() {
        let mut ds = CompressedDisjointSet::new(5);
        assert_eq!(ds.sets_count(), 5);
        assert!(ds.union(0, 1));
        assert!(ds.union(1, 3));
        assert!(ds.union(2, 4));
        assert_eq!(ds.sets_count(), 2);
        assert_eq!(ds.find(0), ds.find(3));
        assert_eq!(ds.find(1), ds.find(3));
        assert_eq!(ds.find(1), ds.find(0));
        assert_eq!(ds.find(2), ds.find(4));
        assert_ne!(ds.find(1), ds.find(4));
    }

    #[test]
    fn compressed_union_same_set() {
        let mut ds = CompressedDisjointSet::new(5);
        assert!(ds.union(0, 1));
        assert!(!ds.union(0, 1));
        assert_eq!(ds.sets_count(), 4);
    }

    #[test]
    fn compressed_sizes() {
        let mut ds = CompressedDisjointSet::new(5);
        assert!(ds.union(0, 1));
        assert!(ds.union(1, 3));
        assert!(ds.union(2, 4));
        assert_eq!(ds.size(1), ds.size(0));
        assert_eq!(ds.size(0), ds.size(3));
        assert_eq!(ds.size(3), 3);
        assert_eq!(ds.size(2), ds.size(4));
        assert_eq!(ds.size(4), 2);
    }

    #[test]
    fn compressed_sets() {
        let mut ds = CompressedDisjointSet::new(5);
        assert!(ds.union(0, 1));
        assert!(ds.union(1, 3));
        assert!(ds.union(2, 4));
        let sets = ds.sets();
        assert!(sets.contains(&vec![0, 1, 3]));
        assert!(sets.contains(&vec![2, 4]));
    }
}
