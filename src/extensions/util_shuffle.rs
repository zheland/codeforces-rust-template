pub use util_shuffle::*;
mod util_shuffle {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    pub trait Shuffle {
        fn shuffle(self, seed: u64) -> Self;
    }

    impl<T> Shuffle for Vec<T> {
        fn shuffle(mut self, seed: u64) -> Self {
            let mut rand = ChaCha8Rng::seed_from_u64(seed);
            let mut result = Vec::new();
            while !self.is_empty() {
                let j = rand.gen_range(0..self.len());
                result.push(self.swap_remove(j));
            }
            result
        }
    }
}
