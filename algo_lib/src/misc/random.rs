//! Pseudo-random number generation for stress tests and randomized algorithms.
//!
//! - [`Random`] — the trait every generator implements. A generator only has
//!   to supply [`next_u64`](Random::next_u64); integers, floats, bounded
//!   values and ranges are derived from it.
//! - [`Rng`] — an explicitly seeded xoshiro256** generator. Use
//!   [`Rng::from_seed`] for reproducible runs and [`Rng::new`] for a
//!   clock-seeded one.
//! - [`StaticRng`] — a zero-sized handle to a per-thread [`Rng`], for call
//!   sites that don't want to thread a generator through.
//! - [`Shuffle`] — shuffling and random choice on slices.
//!
//! None of these are cryptographically secure.

use std::{cell::RefCell, ops::RangeBounds, time::SystemTime};

use crate::{
    collections::slice_ext::indices::Indices,
    math::{
        algebra::{AdditionMonoidWithSub, One},
        bounded::Bounded,
        cast::Cast,
    },
};

/// A source of pseudo-random bits, plus convenience samplers built on top.
///
/// Implement only [`next_u64`](Random::next_u64); every other method has a
/// default in terms of it.
pub trait Random {
    /// Returns the next 64 uniformly distributed random bits.
    fn next_u64(&mut self) -> u64;

    /// Returns a random integer of any primitive type `T`, uniform over all of
    /// its bit patterns.
    ///
    /// The `u64` is cast with `as`, so narrower types keep the low bits and
    /// signed types reinterpret them (e.g. `next_int::<i8>()` is uniform over
    /// `-128..=127`). For a bounded value use
    /// [`next_below`](Random::next_below) or
    /// [`next_range`](Random::next_range) instead.
    fn next_int<T>(&mut self) -> T
    where
        u64: Cast<T>,
    {
        self.next_u64().to()
    }

    /// Returns 128 random bits, built from two [`next_u64`](Random::next_u64)
    /// draws (high half first).
    fn next_u128(&mut self) -> u128 {
        (self.next_u64() as u128) << 64 | self.next_u64() as u128
    }

    /// Returns a random `i128`, uniform over its whole range.
    fn next_i128(&mut self) -> i128 {
        self.next_u128().to()
    }

    /// Returns `true` or `false` with equal probability.
    ///
    /// Uses the top bit of the draw, which is the strongest bit for most
    /// generators.
    fn next_bool(&mut self) -> bool {
        (self.next_u64() >> 63) != 0
    }

    /// Returns a uniform `f64` in `[0, 1)`.
    ///
    /// Takes 53 random bits (the width of an `f64` mantissa) and scales by
    /// 2^-53, so every output is exactly representable and `1.0` is never
    /// returned.
    fn next_f64(&mut self) -> f64 {
        let f = (self.next_u64() >> (64 - f64::MANTISSA_DIGITS)) as f64;
        let i = (1u64 << f64::MANTISSA_DIGITS) as f64;
        f / i
    }

    /// Returns a uniform integer in `[0, n)`, without modulo bias.
    ///
    /// Works for every primitive integer type, including `u128`. `n` must be
    /// positive: this is only checked by a `debug_assert!`, and a signed `n`
    /// that is negative is not caught (it is read as a huge `u128`). Use
    /// `next_range(0..n)` when `n` may be signed.
    fn next_below<T: Cast<u128>>(&mut self, n: T) -> T {
        let n: u128 = n.to();
        debug_assert!(n > 0, "next_below requires n > 0");
        Cast::<u128>::of(next_below_u128(self, n))
    }

    /// Returns a uniform integer from `range`, which may be any
    /// [`RangeBounds`]: `a..b`, `a..=b`, `a..`, `..b`, `..=b` or `..`.
    ///
    /// Works for every primitive integer type, including `i128` and `u128`
    /// and ranges that span the type's entire domain. An unbounded side is
    /// replaced by the type's `MIN` or `MAX`. Named explicitly when the type
    /// can't be inferred, e.g. `next_range::<u32, _>(..)`.
    ///
    /// # Caveats
    ///
    /// The range **must be non-empty**, and this is not checked. An empty
    /// range such as `5..5` or `10..3` does not panic; it returns an
    /// arbitrary value. Excluding the type's own bound (`..T::MIN`,
    /// `(Excluded(T::MAX), _)`) overflows, which panics in debug builds and
    /// wraps in release builds.
    fn next_range<T, R>(&mut self, range: R) -> T
    where
        T: AdditionMonoidWithSub + One + Cast<u128> + Bounded,
        R: RangeBounds<T>,
    {
        use std::ops::Bound::*;
        // Normalise to an inclusive `[from, to]` in `u128` space. Signed values
        // sign-extend, so the wrapping arithmetic below is correct for them too.
        let from = match range.start_bound() {
            Included(&s) => s,
            Excluded(&s) => s + T::one(),
            Unbounded => T::MIN,
        }
        .to();
        let to = match range.end_bound() {
            Included(&e) => e,
            Excluded(&e) => e - T::one(),
            Unbounded => T::MAX,
        }
        .to();
        // The number of values in the range; wraps to 0 only for the full
        // 128-bit range, where any random value is already in range.
        let span = to.wrapping_sub(from).wrapping_add(1);
        let offset = if span == 0 {
            self.next_u128()
        } else {
            next_below_u128(self, span)
        };
        Cast::<u128>::of(from.wrapping_add(offset))
    }
}

/// Unbiased uniform value in `[0, n)`; `n` must be positive.
///
/// Free function (not a trait method) so it stays private. `?Sized` lets the
/// trait's default methods pass `self`, since `Self` is unsized-by-default
/// inside a trait.
fn next_below_u128<R: Random + ?Sized>(rng: &mut R, n: u128) -> u128 {
    if let Ok(n) = u64::try_from(n) {
        // Lemire's multiply-shift with rejection.
        let mut m = rng.next_u64() as u128 * n as u128;
        if (m as u64) < n {
            let threshold = n.wrapping_neg() % n;
            while (m as u64) < threshold {
                m = rng.next_u64() as u128 * n as u128;
            }
        }
        m >> 64
    } else {
        // Plain rejection: discard the 2^128 mod n values that cause bias.
        let threshold = n.wrapping_neg() % n;
        loop {
            let x = rng.next_u128();
            if x >= threshold {
                return x % n;
            }
        }
    }
}

/// SplitMix64, used only to expand a single `u64` seed into [`Rng`]'s 256-bit
/// state. Its outputs are well mixed even for adjacent seeds (0, 1, 2, ...),
/// which seeding xoshiro directly from the seed would not give.
struct SplitMix64(u64);

impl SplitMix64 {
    fn from_seed(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

/// The xoshiro256** generator: fast, with a 2^256 − 1 period and good
/// statistical quality. Not cryptographically secure.
///
/// `Clone` copies the state, so a clone replays the same sequence.
#[derive(Clone)]
pub struct Rng {
    state: [u64; 4],
}

impl Rng {
    /// Creates a generator seeded from the system clock (nanoseconds since the
    /// Unix epoch), so runs differ.
    ///
    /// Two generators created within the same clock tick get the same seed and
    /// therefore the same sequence. Panics if the clock is set before the epoch.
    pub fn new() -> Self {
        Self::from_seed(SystemTime::UNIX_EPOCH.elapsed().unwrap().as_nanos().to())
    }

    /// Creates a generator from a fixed seed. The same seed always yields the
    /// same sequence, which makes failing stress tests reproducible.
    pub fn from_seed(seed: u64) -> Self {
        let mut generator = SplitMix64::from_seed(seed);
        Self {
            state: std::array::from_fn(|_| generator.next_u64()),
        }
    }
}

impl Random for Rng {
    fn next_u64(&mut self) -> u64 {
        let result = self.state[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.state[1] << 17;
        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);
        result
    }
}

impl Default for Rng {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    static RNG: RefCell<Rng> = RefCell::new(Rng::new());
}

/// A zero-sized handle to a generator that lives in thread-local storage.
///
/// Lets code draw random numbers without passing an [`Rng`] around:
/// `StaticRng.next_u64()`. Each thread has its own generator, seeded from the
/// clock the first time the thread uses it, so it is not reproducible. Use
/// [`Rng::from_seed`] when you need a repeatable sequence.
pub struct StaticRng;

impl Random for StaticRng {
    fn next_u64(&mut self) -> u64 {
        RNG.with(|r| r.borrow_mut().next_u64())
    }
}

/// Random reordering and selection for slices.
///
/// The `_with` methods take an explicit generator (use these for reproducible
/// runs); the others use [`StaticRng`].
pub trait Shuffle<T> {
    /// Shuffles the slice in place, uniformly over all permutations.
    fn shuffle_with(&mut self, rng: &mut impl Random);

    /// Returns a uniformly chosen element. Panics if the slice is empty.
    fn choice_with(&self, rng: &mut impl Random) -> &T;

    /// [`shuffle_with`](Shuffle::shuffle_with) using [`StaticRng`].
    fn shuffle(&mut self) {
        self.shuffle_with(&mut StaticRng)
    }

    /// [`choice_with`](Shuffle::choice_with) using [`StaticRng`].
    fn choide(&mut self) -> &T {
        self.choice_with(&mut StaticRng)
    }
}

impl<T> Shuffle<T> for [T] {
    /// Fisher–Yates: each element swaps with a uniformly chosen position at or
    /// before it.
    fn shuffle_with(&mut self, rng: &mut impl Random) {
        for i in self.indices() {
            let at = rng.next_below(i + 1);
            self.swap(at, i);
        }
    }

    fn choice_with(&self, rng: &mut impl Random) -> &T {
        let index = rng.next_below(self.len());
        &self[index]
    }
}
