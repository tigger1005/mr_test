extern crate rand;

use miller_rabin::is_prime;
use num_bigint::{BigUint, RandBigInt};
use rand::rngs::ThreadRng;
use rayon::prelude::*;
use std::{io::Write, time::Instant};
const NUMBER_OF_PRIMES: usize = 50000;

fn get_prime_numer(rng: &mut ThreadRng, k: usize) -> usize {
    let mut count = 0;
    let mut pos_prime: BigUint;

    loop {
        // Get odd random number
        pos_prime = rng.gen_biguint(1024);
        pos_prime.set_bit(0, true);

        count += 1;
        if is_prime(&pos_prime, k) {
            break;
        }
    }
    count
}

// fn average(numbers: &Vec<usize>) -> f32 {
//     numbers.iter().sum::<usize>() as f32 / numbers.len() as f32
// }

fn main() {
    println!("Check for prime");
    println!("Number of primes: {}", NUMBER_OF_PRIMES);

    println!("Miller Rabin test with K=3");
    let now = Instant::now();
    (0..NUMBER_OF_PRIMES).into_par_iter().for_each(|_| {
        let mut rng = rand::thread_rng();
        let _count = get_prime_numer(&mut rng, 3);
        print!(".");
        _ = std::io::stdout().flush();
    });
    let elapsed_3 = now.elapsed();
    println!();

    println!("Miller Rabin test with K=25");
    let now = Instant::now();
    (0..NUMBER_OF_PRIMES).into_par_iter().for_each(|_| {
        let mut rng = rand::thread_rng();
        let _count = get_prime_numer(&mut rng, 25);
        print!(".");
        _ = std::io::stdout().flush();
    });
    let elapsed_25 = now.elapsed();
    println!("\nTime elapsed K = 3  : {:?}", &elapsed_3);
    println!("Time elapsed K = 25 : {:?}", &elapsed_25);
    println!(
        "Time div: {:.2?}%",
        (elapsed_25.as_secs_f64() / elapsed_3.as_secs_f64()) * 100.0
    );

    //println!("0x{:#?}", &num_of_counts_per_prime);
    //println!("Average: {}", average(&num_of_counts_per_prime));
}
