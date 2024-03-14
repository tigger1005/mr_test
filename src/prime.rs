use miller_rabin::is_prime;
use num_bigint::{BigUint, RandBigInt};
use rand::rngs::ThreadRng;

pub fn get_prime_numer(rng: &mut ThreadRng, k: usize) -> usize {
    let mut count = 0;
    let mut pos_prime: BigUint;

    loop {
        // Get odd random number
        pos_prime = rng.gen_biguint(1024);
        pos_prime.set_bit(0, true);

        // Pre test for divider 3 and 5
        let cross_sum = pos_prime
            .to_bytes_le()
            .iter()
            .map(|x| ((*x >> 4) + (*x & 0x0F)) as u32)
            .sum::<u32>();
        if cross_sum % 3 == 0 || cross_sum % 5 == 0 {
            continue;
        }

        count += 1;
        if is_prime(&pos_prime, k) {
            break;
        }
    }
    count
}
