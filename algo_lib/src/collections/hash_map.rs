use std::{collections::HashMap, hash::BuildHasherDefault};

use crate::collections::fx_hasher::FxHasher;

/// A [`HashMap`] that hashes keys with [`FxHasher`] instead of the standard
/// library's default [`SipHasher`].
///
/// # Why not the default SipHash?
///
/// `std`'s `HashMap` defaults to SipHash-1-3, a keyed, cryptographically
/// strong hash chosen to make hash maps resistant to HashDoS attacks: an
/// adversary who can feed arbitrary keys into the map cannot cheaply craft
/// collisions that degrade lookups to `O(n)`. That safety costs cycles —
/// SipHash mixes every byte through several rounds of ARX operations, which is
/// overkill when the input is untrusted by no one.
///
/// In a competitive-programming / offline-solver setting the keys come from a
/// fixed problem input, not a hostile network peer, so HashDoS resistance buys
/// nothing. [`FxHasher`] trades it away for raw speed: a single
/// `rotate-xor-multiply` step per word (see [`FxHasher`]'s `add_to_hash`).
/// That makes it several times faster than SipHash for the small integer and
/// short-byte-slice keys typical here, which is usually the dominant cost in
/// hash-heavy solutions.
///
/// The trade-offs to keep in mind:
/// - Not DoS-resistant — never expose this to adversarial input.
/// - Weaker avalanche than SipHash, so pathological key sets can cluster. In
///   practice this is fine for contest inputs.
///
/// [`SipHasher`]: std::hash::SipHasher
pub type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
