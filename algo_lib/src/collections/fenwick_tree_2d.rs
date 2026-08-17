//! A 2D Fenwick tree: point updates and rectangle sums, both in
//! `O(log n · log m)`.
//!
//! [`FenwickTree2D<T>`] is a Fenwick tree *of* Fenwick trees. The row index gets
//! the same block decomposition as the column index, and that is the entire
//! difference between this and a flat `Vec<FenwickTree<T>>` holding one tree per
//! row.
//!
//! # Layout
//!
//! Inner tree `i` is **not** row `i`. It is a column tree over the row-block
//! `[i & (i + 1), i]`, so the cell `(r, c)` lives in every inner tree whose
//! block contains `r` — exactly `log n` of them, mirroring the 1D fact that an
//! index belongs to `log n` cells. Composing both axes, cell `j` of inner tree
//! `i` holds the sum over the product range
//!
//! ```text
//! [i & (i + 1) ..= i]  ×  [j & (j + 1) ..= j]
//! ```
//!
//! Every method is then the corresponding 1D walk with a whole inner tree
//! standing where the 1D version has a single value: [`add`](FenwickTree2D::add)
//! climbs the row blocks and lets each inner tree climb its own columns, and a
//! query walks the row blocks and asks each inner tree for a column range.
//!
//! # Memory is the binding constraint
//!
//! `O(n · m)`, held as `n` separate row vectors. Time is rarely what stops you
//! here; space is. A `1e5 × 1e5` grid is `1e10` cells and cannot be allocated at
//! all. For large sparse data, coordinate-compress the keys first, or drop a
//! dimension by sweeping one axis offline with the 1D [`FenwickTree`].
//!
//! # Why not one tree per row?
//!
//! A flat `Vec<FenwickTree<T>>` with one tree per row is not wrong, just a
//! different trade. It updates in `O(log m)` and answers a single row's range in
//! `O(log m)` — both better than here — but a *rectangle* costs
//! `O(rows · log m)`, because it has to visit every row in range. Decomposing
//! the row axis as well is exactly what buys the logarithmic rectangle.
//!
//! # Example
//!
//! ```
//! use algo_lib::collections::fenwick_tree_2d::FenwickTree2D;
//!
//! let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
//! let mut ft = FenwickTree2D::from(grid.as_slice());
//!
//! assert_eq!(ft.get(.., ..), 45);
//! assert_eq!(ft.get(0..2, 1..3), 16); // 2 + 3 + 5 + 6
//!
//! ft.add(1, 1, 10);
//! assert_eq!(ft.get(1..=1, 1..=1), 15);
//! ```

use std::ops::RangeBounds;

use crate::{
    collections::{
        fenwick_tree::{FenwickTree, resolve_bounds},
        slice_ext::indices::Indices,
    },
    math::algebra::AdditionMonoidWithSub,
};

/// A grid supporting point addition and rectangle sum, each in
/// `O(log n · log m)`.
///
/// See the [module documentation](self) for the block layout and for what the
/// `O(n · m)` memory rules out.
pub struct FenwickTree2D<T> {
    /// Inner tree `i` covers the row-block `[i & (i + 1), i]`, not row `i`.
    ft: Vec<FenwickTree<T>>,
}

impl<T: AdditionMonoidWithSub + Copy> FenwickTree2D<T> {
    /// Creates an all-zero grid over rows `0..n` and columns `0..m`.
    /// `O(n · m)`.
    ///
    /// To load a grid you already have, prefer `FenwickTree2D::from(&grid)`:
    /// it builds in `O(n · m)` where `n · m` calls to [`add`](Self::add) would
    /// cost `O(n · m · log n · log m)`.
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            ft: vec![FenwickTree::new(m); n],
        }
    }

    /// Sum over the prefix rectangle rows `[0, i)` × columns `[0, j)`,
    /// mirroring `grid[..i]` by `row[..j]`. `O(log n · log m)`.
    ///
    /// Both ends are clamped, so any `i` or `j` at or past the edge gives the
    /// whole extent.
    pub fn get_to(&self, mut i: usize, j: usize) -> T {
        i = i.min(self.rows()).wrapping_sub(1);
        let mut result = T::zero();
        while let Some(ft) = self.ft.get(i) {
            result += ft.get_to(j);
            i = (i & (i + 1)).wrapping_sub(1);
        }
        result
    }

    /// Sum over rows `[0, i)` × columns `[ja, jb)`: a prefix down the rows, an
    /// arbitrary range across the columns. `O(log n · log m)`.
    ///
    /// This is the primitive [`get`](Self::get) is built from, and the reason a
    /// rectangle costs two row walks rather than four. Asking each inner tree
    /// for the column *range* fuses the pairs of the four-corner
    /// inclusion–exclusion — `(i2, j2)` with `(i2, j1)` decompose the same row
    /// blocks — so half the row walks disappear and each inner vector is
    /// visited once while still cache-hot.
    pub fn get_rows_to(&self, mut i: usize, ja: usize, jb: usize) -> T {
        i = i.min(self.rows()).wrapping_sub(1);
        let mut result = T::zero();
        while let Some(ft) = self.ft.get(i) {
            result += ft.get(ja..jb);
            i = (i & (i + 1)).wrapping_sub(1);
        }
        result
    }

    /// Sum over the rectangle `rows` × `cols`, each bound read exactly as slice
    /// indexing reads it: `get(1..3, 2..=4)` is rows 1 and 2 crossed with
    /// columns 2 through 4. `O(log n · log m)`.
    ///
    /// The two axes take independent range types, so mixing kinds — `get(1..3,
    /// ..)`, `get(2.., ..3)` — is fine. An empty or reversed range on either
    /// axis gives zero, and bounds past the edge are clamped rather than
    /// panicking.
    pub fn get<R, C>(&self, rows: R, cols: C) -> T
    where
        R: RangeBounds<usize>,
        C: RangeBounds<usize>,
    {
        let (ia, ib) = resolve_bounds(rows, self.rows());
        let (ja, jb) = resolve_bounds(cols, self.cols());
        if ia >= ib || ja >= jb {
            T::zero()
        } else {
            self.get_rows_to(ib, ja, jb) - self.get_rows_to(ia, ja, jb)
        }
    }

    /// Adds `value` to the cell `(i, j)`. `O(log n · log m)`.
    ///
    /// Climbs the `log n` row-blocks containing row `i` and lets each inner tree
    /// climb its own `log m` column cells. An out-of-range `i` or `j` is a
    /// silent no-op — the row loop never runs, or the inner
    /// [`FenwickTree::add`] doesn't.
    pub fn add(&mut self, mut i: usize, j: usize, value: T) {
        while let Some(ft) = self.ft.get_mut(i) {
            ft.add(j, value);
            i |= i + 1;
        }
    }

    /// Resets every cell to zero, keeping the dimensions. `O(n · m)`.
    pub fn clear(&mut self) {
        self.ft.iter_mut().for_each(FenwickTree::clear);
    }

    /// The number of rows: valid row indices are `0..rows()`. `O(1)`.
    pub fn rows(&self) -> usize {
        self.ft.len()
    }

    /// The number of columns: valid column indices are `0..cols()`. `O(1)`.
    ///
    /// Read off the first inner tree, since every row has the same width; a
    /// grid with no rows reports `0`.
    pub fn cols(&self) -> usize {
        self.ft.first().map_or(0, FenwickTree::len)
    }
}

/// Builds a grid in `O(n · m)`, rather than the `O(n · m · log n · log m)` that
/// a call to [`add`](FenwickTree2D::add) per cell would cost.
///
/// `vecs` must be rectangular — every row the same length, which becomes the
/// column width. A ragged input either panics or silently loses the columns past
/// a short row.
///
/// Two passes, and they separate because the column transform is *linear*: each
/// output cell is a plain sum of input cells, so transforming rows and then
/// adding them equals adding them and then transforming.
///
/// 1. Propagate whole rows into their row-block parents in raw space,
///    `g[i | (i + 1)] += g[i]` elementwise. This is the 1D build's own walk with
///    a row as the element type, and ascending order is safe for the same reason
///    as in 1D: the parent index `i | (i + 1)` is always greater than `i`.
/// 2. Hand each finished row to the 1D build, which applies the column
///    transform.
impl<T: AdditionMonoidWithSub + Copy> From<&[Vec<T>]> for FenwickTree2D<T> {
    fn from(vecs: &[Vec<T>]) -> Self {
        let mut g = vecs.to_vec();
        for i in vecs.indices() {
            let p = i | (i + 1);
            if p < vecs.len() {
                for j in vecs[i].indices() {
                    let v = g[i][j];
                    g[p][j] += v;
                }
            }
        }
        Self {
            ft: g.iter().map(|r| FenwickTree::from(r.as_slice())).collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::collections::{fenwick_tree_2d::FenwickTree2D, slice_ext::indices::Indices};

    const N: usize = 5;
    const M: usize = 7;

    /// A grid and the tree holding it, so queries can be checked against a
    /// plain nested loop.
    fn grid() -> (FenwickTree2D<i64>, [[i64; M]; N]) {
        let mut g = [[0; M]; N];
        let mut ft = FenwickTree2D::new(N, M);
        for i in g.indices() {
            for j in g[i].indices() {
                let v = (i * M + j) as i64 % 7 - 3;
                g[i][j] = v;
                ft.add(i, j, v);
            }
        }
        (ft, g)
    }

    fn sum(g: &[[i64; M]; N], r: std::ops::Range<usize>, c: std::ops::Range<usize>) -> i64 {
        g[r].iter()
            .map(|row| row[c.clone()].iter().sum::<i64>())
            .sum()
    }

    #[test]
    fn every_rectangle_matches_a_nested_loop() {
        let (ft, g) = grid();
        for r0 in 0..=N {
            for r1 in r0..=N {
                for c0 in 0..=M {
                    for c1 in c0..=M {
                        assert_eq!(
                            ft.get(r0..r1, c0..c1),
                            sum(&g, r0..r1, c0..c1),
                            "get({r0}..{r1}, {c0}..{c1})"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn prefixes_and_mixed_bound_kinds() {
        let (ft, g) = grid();
        assert_eq!(ft.get(.., ..), sum(&g, 0..N, 0..M));
        assert_eq!(ft.get(1..3, ..), sum(&g, 1..3, 0..M));
        assert_eq!(ft.get(.., 2..=4), sum(&g, 0..N, 2..5));
        assert_eq!(ft.get(2.., ..3), sum(&g, 2..N, 0..3));
        assert_eq!(ft.get(1..=3, 1..=1), sum(&g, 1..4, 1..2));
        for i in 0..=N {
            for j in 0..=M {
                assert_eq!(ft.get_to(i, j), sum(&g, 0..i, 0..j), "get_to({i}, {j})");
                assert_eq!(ft.get(..i, ..j), sum(&g, 0..i, 0..j), "get(..{i}, ..{j})");
            }
        }
    }

    #[test]
    fn from_grid_matches_a_pointwise_build() {
        let (built, g) = grid();
        let rows: Vec<Vec<i64>> = g.iter().map(|r| r.to_vec()).collect();
        let from = FenwickTree2D::from(rows.as_slice());
        for i in 0..=N {
            for j in 0..=M {
                assert_eq!(from.get_to(i, j), built.get_to(i, j), "get_to({i}, {j})");
            }
        }
    }

    #[test]
    fn empty_and_out_of_range_rectangles_are_zero() {
        let (ft, g) = grid();
        assert_eq!(ft.get(2..2, 0..M), 0, "empty row range");
        assert_eq!(ft.get(0..N, 3..3), 0, "empty column range");
        assert_eq!(ft.get(N.., ..), 0, "rows past the grid");
        assert_eq!(ft.get(.., M..), 0, "columns past the grid");
        assert_eq!(ft.get(0..100, 0..100), sum(&g, 0..N, 0..M), "clamped");
        assert_eq!(FenwickTree2D::<i64>::new(0, 0).get(.., ..), 0, "empty grid");
    }

    #[test]
    fn add_and_clear() {
        let (mut ft, g) = grid();
        ft.add(2, 3, 100);
        assert_eq!(ft.get(2..=2, 3..=3), g[2][3] + 100);
        assert_eq!(ft.get(.., ..), sum(&g, 0..N, 0..M) + 100);
        ft.add(N + 4, 0, 5); // out of range: silently ignored
        ft.add(0, M + 4, 5);
        assert_eq!(ft.get(.., ..), sum(&g, 0..N, 0..M) + 100);
        ft.clear();
        assert_eq!(ft.get(.., ..), 0);
        assert_eq!(ft.rows(), N);
        assert_eq!(ft.cols(), M);
    }
}
