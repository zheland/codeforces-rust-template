use crate::extensions::prime_factorize::{
    count_multiplier_primes, count_multipliers, factorize_to_vec,
};
use crate::{DedupCount, IntoVec, Primes};

#[test]
fn test_prime_sieve_and_factorize() {
    let sieve = Primes::new(1000000);
    for j in 2..10000 {
        assert_eq!(
            sieve.factorize(j).dedup_count().into_vec(),
            factorize_to_vec(j)
        );
        assert_eq!(sieve.factorize(j).count(), count_multipliers(j));
        assert_eq!(
            sieve.factorize(j).dedup_count().count(),
            count_multiplier_primes(j)
        );
    }
}
