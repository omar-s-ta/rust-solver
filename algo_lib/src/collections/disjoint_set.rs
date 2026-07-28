use std::cell::Cell;

use crate::{collections::slice_ext::indices::Indices, math::cast::Cast};

/// Disjoint-set (union-find): maintains a partition of the elements `0..n`
/// into sets, supporting merge and same-set queries.
///
/// Two implementations are provided: [`CompressedDisjointSet`], which uses
/// path compression for near-constant amortized queries, and
/// [`RollbackDisjointSet`], which trades compression away in exchange for
/// undoable unions (see its docs for the resulting bounds).
pub trait DisjointSet {
    /// Creates a partition of `n` elements, each in its own singleton set.
    fn new(n: usize) -> Self;

    /// Returns the representative (root) of the set containing `i`. Two
    /// elements are in the same set iff their representatives are equal.
    fn find(&self, i: usize) -> usize;

    /// Merges the sets containing `a` and `b`, returning `false` if they were
    /// already in the same set.
    fn union(&mut self, a: usize, b: usize) -> bool;

    /// Returns the number of elements in the set containing `i`.
    fn size(&self, i: usize) -> usize;

    /// Returns the number of disjoint sets.
    fn sets_count(&self) -> usize;

    /// Returns the total number of elements across all sets.
    fn len(&self) -> usize;

    /// Returns `true` if there are no elements.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Disjoint-set using union by size and path halving, giving O(α(n))
/// amortized per operation.
///
/// # Encoding
///
/// The `parent` array carries both roles at once, so no separate size array
/// is needed:
///
/// - `parent[i] < 0` → `i` is a root, and its set holds `-parent[i]` elements.
/// - `parent[i] >= 0` → `parent[i]` is the parent of `i`.
#[derive(Clone)]
pub struct CompressedDisjointSet {
    parent: Vec<Cell<i32>>,
    count: usize,
}

/// Disjoint-set (union-find) supporting rollback to previous states.
///
/// Unlike [`CompressedDisjointSet`], `find` performs **no path compression**:
/// compression would mutate O(path length) parent pointers per query, and
/// every mutation would have to be recorded to stay undoable. Instead all
/// mutations happen in `union`, which appends a single entry to a history
/// log, so [`rollback`](Self::rollback) can undo unions one entry at a time.
///
/// Union by size keeps trees at depth O(log n), giving:
/// - `find`, `union`: O(log n)
/// - [`rollback`](Self::rollback): O(k) to undo k unions
pub struct RollbackDisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
    history: Vec<(usize, usize)>,
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
        loop {
            let p = self.parent[node].get();
            if p < 0 {
                return node;
            }
            let p: usize = p.to();
            let gp = self.parent[p].get();
            if gp < 0 {
                return p;
            }
            self.parent[node].set(gp);
            node = gp.to();
        }
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
    /// Returns an iterator over the representatives (roots) of all sets, in
    /// ascending index order.
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.parent
            .iter()
            .enumerate()
            .filter_map(|(i, id)| (id.get() < 0).then_some(i))
    }

    /// Resets every element to its own singleton set.
    pub fn clear(&mut self) {
        self.count = self.parent.len();
        self.parent.fill(Cell::new(-1));
    }

    /// Returns the current partition as a vec of sets, each holding its
    /// member indices in ascending order. Buckets are created in
    /// first-encounter order of their root during a `0..n` scan.
    ///
    /// O(n·α(n)).
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

impl DisjointSet for RollbackDisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            count: n,
            history: Vec::new(),
        }
    }

    fn find(&self, i: usize) -> usize {
        let mut node = i;
        while self.parent[node] != node {
            node = self.parent[node];
        }
        node
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a == b {
            return false;
        }
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.history.push((a, b));
        self.parent[b] = a;
        self.size[a] += self.size[b];
        self.count -= 1;
        true
    }

    fn size(&self, i: usize) -> usize {
        self.size[self.find(i)]
    }

    fn sets_count(&self) -> usize {
        self.count
    }

    fn len(&self) -> usize {
        self.parent.len()
    }
}

impl RollbackDisjointSet {
    /// Returns a checkpoint that can later be passed to
    /// [`rollback`](Self::rollback) to undo every union made after this point.
    ///
    /// O(1).
    pub fn checkpoint(&self) -> usize {
        self.history.len()
    }

    /// Undoes all unions made after `checkpoint` was taken, restoring the
    /// exact partition (and set sizes) from that point. Checkpoints may be
    /// rolled back in any order as long as the checkpoint is not ahead of
    /// the current history; a checkpoint ahead of the history is a no-op.
    ///
    /// O(k) where k is the number of unions undone.
    pub fn rollback(&mut self, checkpoint: usize) {
        while self.history.len() > checkpoint {
            let (a, b) = self.history.pop().expect("bigger than checkpoint");
            self.parent[b] = b;
            self.size[a] -= self.size[b];
            self.count += 1;
        }
    }

    /// Resets every element to its own singleton set and clears the history.
    /// Previously taken checkpoints become no-ops.
    pub fn clear(&mut self) {
        for i in self.parent.indices() {
            self.parent[i] = i;
            self.size[i] = 1;
        }
        self.count = self.parent.len();
        self.history.clear();
    }
}

#[cfg(test)]
mod test {
    use crate::collections::disjoint_set::RollbackDisjointSet;

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

    #[test]
    fn rollback_union_find() {
        let mut ds = RollbackDisjointSet::new(5);
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
    fn rollback_union_same_set() {
        let mut ds = RollbackDisjointSet::new(5);
        assert!(ds.union(0, 1));
        assert!(!ds.union(0, 1));
        assert_eq!(ds.sets_count(), 4);
    }

    #[test]
    fn rollback_sizes() {
        let mut ds = RollbackDisjointSet::new(5);
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
    fn rollback_restore_states() {
        let mut ds = RollbackDisjointSet::new(5);
        assert!(ds.union(0, 1));
        let cp = ds.checkpoint();
        assert_eq!(ds.sets_count(), 4);
        assert!(ds.union(1, 3));
        assert!(ds.union(2, 4));
        assert_eq!(ds.sets_count(), 2);
        ds.rollback(cp);
        assert_eq!(ds.sets_count(), 4);
        assert_eq!(ds.find(0), ds.find(1));
        assert_ne!(ds.find(1), ds.find(3));
        assert_ne!(ds.find(2), ds.find(4));
    }

    #[test]
    fn rollback_to_empty() {
        let mut ds = RollbackDisjointSet::new(3);
        let cp = ds.checkpoint();
        ds.union(0, 1);
        ds.union(1, 2);
        assert_eq!(ds.sets_count(), 1);
        ds.rollback(cp);
        assert_eq!(ds.sets_count(), 3);
        assert_ne!(ds.find(0), ds.find(1));
        assert_ne!(ds.find(1), ds.find(2));
    }

    #[test]
    fn multiple_checkpoints() {
        let mut ds = RollbackDisjointSet::new(5);
        ds.union(0, 1);
        let cp1 = ds.checkpoint();
        ds.union(2, 3);
        let cp2 = ds.checkpoint();
        ds.union(0, 4);
        assert_eq!(ds.sets_count(), 2);
        ds.rollback(cp2);
        assert_eq!(ds.sets_count(), 3);
        ds.rollback(cp1);
        assert_eq!(ds.sets_count(), 4);
    }

    #[test]
    fn rollback_no_history_on_failed_union() {
        let mut ds = RollbackDisjointSet::new(5);
        assert!(ds.union(0, 1));
        let cp = ds.checkpoint();
        assert!(!ds.union(0, 1));
        assert_eq!(ds.checkpoint(), cp);
    }
}
