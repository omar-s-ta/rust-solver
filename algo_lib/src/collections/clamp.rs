use std::ops::RangeBounds;

/// Resolves `bounds` into a half-open index range `[from, to)` for a sequence
/// of length `len`.
///
/// Any [`RangeBounds`] is accepted (`a..b`, `a..=b`, `..b`, `a..`, `..`, or a
/// raw `(Bound, Bound)` pair). An unbounded start becomes `0`, an unbounded
/// end becomes `len`, and inclusive/exclusive bounds are normalised to the
/// half-open form. Only `to` is clamped to `len`; `from` is returned as is,
/// so `from > to` (an empty or out-of-range request) is possible and left to
/// the caller to handle.
///
/// # Panics
///
/// Panics in debug builds if the start is `Bound::Excluded(usize::MAX)`.
///
/// # Examples
///
/// ```
/// use algo_lib::collections::clamp::clamp;
///
/// assert_eq!(clamp(&(2..=4), 10), (2, 5));
/// assert_eq!(clamp(&(..), 10), (0, 10));
/// assert_eq!(clamp(&(3..100), 10), (3, 10));
/// assert_eq!(clamp(&(12..), 10), (12, 10));
/// ```
pub fn clamp(bounds: &impl RangeBounds<usize>, len: usize) -> (usize, usize) {
    let from = match bounds.start_bound() {
        std::ops::Bound::Included(&x) => x,
        std::ops::Bound::Excluded(&x) => x + 1,
        std::ops::Bound::Unbounded => 0,
    };
    let to = match bounds.end_bound() {
        std::ops::Bound::Included(&x) => x.saturating_add(1),
        std::ops::Bound::Excluded(&x) => x,
        std::ops::Bound::Unbounded => len,
    };
    (from, to.min(len))
}
