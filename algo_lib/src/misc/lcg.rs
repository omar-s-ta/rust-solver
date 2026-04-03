use std::collections::hash_map::DefaultHasher;
use std::{
    hash::{Hash, Hasher},
    time::SystemTime,
};

fn seed() -> u64 {
    let mut hasher = DefaultHasher::new();
    SystemTime::now().hash(&mut hasher);
    hasher.finish()
}

/// Simple 'Linear Congruential Generator'
pub struct Lcg(u64);

impl Lcg {
    pub fn new() -> Self {
        Lcg(seed())
    }

    pub fn generate(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);

        self.0
    }
}

impl Default for Lcg {
    fn default() -> Self {
        Self::new()
    }
}
