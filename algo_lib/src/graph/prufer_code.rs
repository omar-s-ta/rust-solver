//! Prüfer code: a bijection between labeled trees on `n` nodes (`n >= 2`) and
//! integer sequences of length `n - 2` over `0..n`.
//!
//! - [`PruferCode::encode`] turns a tree (adjacency list) into its sequence.
//! - [`PruferCode::decode`] turns a sequence back into the tree's edge list.
//!
//! Both directions run in `O(n)`: the next leaf is always either the freshly
//! created one (when smaller than the scan pointer) or the next node the pointer
//! advances to, so the pointer sweeps `0..n` only once and never backtracks.
//!
//! The two operations are exposed as associated functions on the [`PruferCode`]
//! type rather than free functions so the submission bundler has a named item to
//! anchor on when inlining this module.
//!
//! ```
//! use algo_lib::graph::prufer_code::PruferCode;
//!
//! // path 0 - 1 - 2 - 3 - 4
//! let adj = vec![vec![1], vec![0, 2], vec![1, 3], vec![2, 4], vec![3]];
//! assert_eq!(PruferCode::encode(&adj), vec![1, 2, 3]);
//! assert_eq!(PruferCode::decode(&[1, 2, 3]), vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
//! ```

/// Namespace for the Prüfer [`encode`](PruferCode::encode) /
/// [`decode`](PruferCode::decode) operations.
pub struct PruferCode;

impl PruferCode {
    /// Encodes a tree, given as an adjacency list (`adj[u]` lists the neighbors
    /// of `u`), into its Prüfer sequence of length `n - 2`.
    ///
    /// The sequence is built by repeatedly removing the smallest-labeled leaf and
    /// recording its single surviving neighbor. `degree[v]` doubles as a liveness
    /// flag (`0` once `v` is removed); `ptr` sweeps labels in increasing order
    /// while `leaf` jumps backwards only when a removal creates a smaller leaf.
    ///
    /// Returns an empty sequence for `n < 2` (where `n - 2` would underflow); a
    /// 2-node tree falls through with a correct length-0 sequence.
    pub fn encode(adj: &[Vec<usize>]) -> Vec<usize> {
        let n = adj.len();
        if n < 2 {
            return Vec::new();
        }

        let mut degree = adj.iter().map(|a| a.len()).collect::<Vec<_>>();
        let mut ptr = (0..n).find(|&i| degree[i] == 1).expect("At least one leaf");
        let mut leaf = ptr;
        let mut code = Vec::with_capacity(n - 2);

        for _ in 0..n - 2 {
            // A leaf has exactly one neighbor still in the tree (degree > 0);
            // since the reduced tree stays connected to the root, that neighbor
            // is its parent. The scan is amortized O(n) across all removals.
            let parent = adj[leaf]
                .iter()
                .copied()
                .find(|&i| degree[i] > 0)
                .expect("A parent for the leaf");

            code.push(parent);
            degree[leaf] = 0; // mark the leaf removed so later scans skip it
            degree[parent] -= 1;

            // The parent becomes the next leaf only if it is now a leaf and sits
            // behind the sweep; otherwise advance the pointer to the next leaf.
            if degree[parent] == 1 && parent < ptr {
                leaf = parent;
            } else {
                ptr += 1;
                while degree[ptr] != 1 {
                    ptr += 1;
                }
                leaf = ptr;
            }
        }
        code
    }

    /// Decodes a Prüfer sequence into the tree's `n - 1` edges, where `n` is
    /// `code.len() + 2`.
    ///
    /// In the reconstructed tree `degree[v] == (occurrences of v in code) + 1`,
    /// so the array is seeded with `1`s and bumped per occurrence. Each code
    /// entry is joined to the current smallest leaf (`degree == 1`); decrementing
    /// a degree to `1` turns that vertex into the next leaf. After consuming the
    /// code, exactly two vertices remain — the last leaf and `n - 1` — forming
    /// the final edge.
    pub fn decode(code: &[usize]) -> Vec<(usize, usize)> {
        let n = code.len() + 2;
        let mut degree = vec![1usize; n];
        code.iter().for_each(|&v| degree[v] += 1);

        let mut ptr = (0..n).find(|&i| degree[i] == 1).expect("At least one leaf");
        let mut leaf = ptr;

        let mut edges = Vec::with_capacity(n - 1);
        for &vertex in code {
            edges.push((leaf, vertex));
            degree[vertex] -= 1;

            if degree[vertex] == 1 && vertex < ptr {
                leaf = vertex;
            } else {
                ptr += 1;
                while ptr < n && degree[ptr] != 1 {
                    ptr += 1;
                }
                leaf = ptr;
            }
        }
        // The two survivors are the current leaf and the highest label, n - 1,
        // which is never removed during the process.
        edges.push((leaf, n - 1));
        edges
    }
}
