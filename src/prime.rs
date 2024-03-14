use miller_rabin::is_prime;
use num_bigint::{BigUint, RandBigInt};
use num_integer::Integer;
use rand::rngs::ThreadRng;

pub fn get_prime_numer(rng: &mut ThreadRng, k: usize) -> (usize, usize) {
    let mut count_mr = 0;
    let mut count_pre = 0;
    let mut pos_prime: BigUint;

    loop {
        // Get odd random number
        pos_prime = rng.gen_biguint(1024);
        pos_prime.set_bit(0, true);

        // Pre test for divider 3 and 5
        let array = pos_prime.to_bytes_le();

        let cross_sum = array.iter().map(|x| *x as i32).sum::<i32>();
        if cross_sum % 3 == 0 || cross_sum % 5 == 0 || cross_sum % 17 == 0 {
            count_pre += 1;
            continue;
        }
        let cross_sum = array
            .iter()
            .enumerate()
            .map(|(i, x)| {
                if i.is_even() {
                    *x as i32
                } else {
                    -1 * (*x) as i32
                }
            })
            .sum::<i32>();
        if cross_sum % 11 == 0 || cross_sum % 23 == 0 {
            count_pre += 1;
            continue;
        }

        count_mr += 1;
        if is_prime(&pos_prime, k) {
            break;
        }
    }
    (count_mr, count_pre)
}
