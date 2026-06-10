/// In-place "keep the smaller / larger" updates.
///
/// These are the two relaxation steps that pervade competitive programming and
/// dynamic programming: `best.with_min(candidate)` overwrites `best` with
/// `candidate` only when `candidate` is smaller, and `with_max` is the
/// symmetric variant. Each method returns `true` when the value actually
/// changed, which is handy for tracking whether a fixed-point iteration is
/// still making progress.
///
/// ```
/// use algo_lib::collections::min_max::MinMax;
///
/// let mut best = 10;
/// assert!(best.with_min(7));   // 7 < 10, so `best` becomes 7
/// assert_eq!(best, 7);
/// assert!(!best.with_min(9));  // 9 is not smaller, `best` is unchanged
/// assert_eq!(best, 7);
/// ```
///
/// The `RHS` type parameter lets the right-hand side differ from `Self`; this
/// is what powers the [`Option`] implementation, where the comparison value is
/// a bare `T` rather than another `Option<T>`.
pub trait MinMax<RHS = Self>: PartialOrd {
    /// Replaces `self` with `rhs` when `rhs` is strictly smaller, returning
    /// whether the replacement happened.
    fn with_min(&mut self, rhs: RHS) -> bool;

    /// Replaces `self` with `rhs` when `rhs` is strictly larger, returning
    /// whether the replacement happened.
    fn with_max(&mut self, rhs: RHS) -> bool;
}

impl<T: PartialOrd> MinMax for T {
    fn with_min(&mut self, rhs: Self) -> bool {
        if *self > rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }

    fn with_max(&mut self, rhs: Self) -> bool {
        if *self < rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
}

/// Treats `None` as "no value seen yet": the first relaxation always takes the
/// incoming value, and later ones compare against the stored value. This makes
/// `Option<T>` a ready-made accumulator for the minimum/maximum of a stream
/// without needing a sentinel like `i64::MAX`.
///
/// ```
/// use algo_lib::collections::min_max::MinMax;
///
/// let mut best: Option<i32> = None;
/// assert!(best.with_max(3));  // None -> Some(3)
/// assert!(best.with_max(5));  // 5 > 3 -> Some(5)
/// assert!(!best.with_max(1)); // 1 is not larger -> unchanged
/// assert_eq!(best, Some(5));
/// ```
impl<T: PartialOrd> MinMax<T> for Option<T> {
    fn with_min(&mut self, rhs: T) -> bool {
        match self {
            Some(lhs) => lhs.with_min(rhs),
            None => {
                *self = Some(rhs);
                true
            }
        }
    }

    fn with_max(&mut self, rhs: T) -> bool {
        match self {
            Some(lhs) => lhs.with_max(rhs),
            None => {
                *self = Some(rhs);
                true
            }
        }
    }
}
