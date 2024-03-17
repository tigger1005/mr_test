use miller_rabin::is_prime;
use num_bigint::{BigUint, RandBigInt};
use num_integer::Integer;
use rand::rngs::ThreadRng;

// Multiplicated primes for 3 to 353
const SMALL_PRIMES: [u8; 60] = [
    0x03, 0xc9, 0x1d, 0xd1, 0x2e, 0x89, 0x31, 0x91, 0x94, 0x09, 0x56, 0x49, 0x87, 0x4b, 0x41, 0xd6,
    0x05, 0x81, 0x0c, 0x06, 0x19, 0x5d, 0x70, 0xeb, 0xbd, 0x54, 0xa8, 0x62, 0x50, 0xc5, 0x27, 0x33,
    0x06, 0xdc, 0x66, 0x48, 0x1c, 0x25, 0x1c, 0xa4, 0xa0, 0x2c, 0x9a, 0x04, 0x78, 0xc9, 0x6f, 0x0d,
    0x02, 0xf0, 0xdb, 0x0b, 0x39, 0xd6, 0x24, 0xca, 0x0b, 0x04, 0x41, 0xc1,
];

pub fn get_prime_numer(rng: &mut ThreadRng, k: usize) -> (usize, usize) {
    let mut count_mr = 0;
    let mut count_pre = 0;
    let mut pos_prime: BigUint;
    let small_prime: BigUint = BigUint::from_bytes_be(&SMALL_PRIMES);

    loop {
        // Get odd random number
        pos_prime = rng.gen_biguint(1024);
        pos_prime.set_bit(0, true);

        // pos_prime.gcd(other)
        let array = pos_prime.to_bytes_le();
        // Pre test for divider 3, 5 and 17
        let cross_sum = array.iter().map(|x| *x as i32).sum::<i32>();
        if cross_sum % 3 == 0 || cross_sum % 5 == 0 || cross_sum % 17 == 0 {
            count_pre += 1;
            continue;
        }
        // Pre test for divider 11 and 23
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
        // Pre test for divider of prime between 3 and 353
        if pos_prime.gcd(&small_prime) != 1u32.into() {
            count_pre += 1;
            continue;
        }
        // Do extensive Miller-Rabin test
        count_mr += 1;
        if is_prime(&pos_prime, k) {
            break;
        }
    }
    (count_mr, count_pre)
}
