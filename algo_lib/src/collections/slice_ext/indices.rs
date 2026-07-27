use std::ops::Range;

/// Extension trait providing a range over the valid indices of a slice.
///
/// Useful for indexed iteration without writing `0..slice.len()`:
///
/// ```
/// use algo_lib::collections::slice_ext::indices::Indices;
///
/// let v = [10, 20, 30];
/// assert_eq!(v.indices(), 0..3);
/// ```
pub trait Indices {
    /// Returns the range of valid indices, i.e. `0..len`.
    fn indices(&self) -> Range<usize>;
}

impl<T> Indices for [T] {
    fn indices(&self) -> Range<usize> {
        0..self.len()
    }
}
