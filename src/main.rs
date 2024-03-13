extern crate rand;

use miller_rabin::is_prime;
use num_bigint::{BigUint, RandBigInt};
use rand::rngs::ThreadRng;
use rayon::prelude::*;

const K: usize = 1;

fn get_prime_numer(rng: &mut ThreadRng) -> usize {
    let mut count = 0;
    let mut pos_prime: BigUint;

    loop {
        // Get odd random number
        pos_prime = rng.gen_biguint(1024);
        pos_prime.set_bit(0, true);

        count += 1;
        if is_prime(&pos_prime, K) {
            break;
        }
    }
    count
}

fn average(numbers: &Vec<usize>) -> f32 {
    numbers.iter().sum::<usize>() as f32 / numbers.len() as f32
}

fn main() {
    println!("Check for prime");

    let num_of_counts_per_prime: Vec<usize> = (0..100)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let count = get_prime_numer(&mut rng);
            println!("Prime number found after {} tries", &count);
            count
        })
        .collect();

    println!("0x{:#?}", &num_of_counts_per_prime);
    println!("Average: {}", average(&num_of_counts_per_prime));
}
