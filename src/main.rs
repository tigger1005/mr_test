use rand;

use clap::Parser;
use std::env;

mod prime;
use prime::get_prime_numer;

use rayon::prelude::*;
use std::{io::Write, time::Instant};

/// Program to simulate fault injections on ARMv8-M processors (e.g. M33)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of threads started in parallel
    #[arg(short, long, default_value_t = 0)]
    threads: u16,

    /// Investigation methode of Miller Rabin runtime
    #[arg(short, long, default_value_t = String::from("calls"))]
    investigation: String,

    /// Number of primes to be generated
    #[arg(short, long, default_value_t = 1000)]
    number_of_primes: usize,
}

fn average(numbers: &Vec<usize>) -> f32 {
    numbers.iter().sum::<usize>() as f32 / numbers.len() as f32
}

fn main() {
    // Get parameter from command line
    let args = Args::parse();
    // Set parameter from cli
    env::set_var("RAYON_NUM_THREADS", args.threads.to_string());

    println!("Check for prime (1024 bit) numbers with Miller-Rabin test");
    println!("Number of primes: {}", args.number_of_primes);

    match args.investigation.as_str() {
        "calls" => mr_calls_investigation(args.number_of_primes, 1),
        "time" => mr_time_investigation(args.number_of_primes),
        _ => {}
    }
}

fn mr_time_investigation(n: usize) {
    println!("Miller Rabin test with K=3");
    let now = Instant::now();
    (0..n).into_par_iter().for_each(|_| {
        let mut rng = rand::thread_rng();
        let _count = get_prime_numer(&mut rng, 3);
        print!(".");
        _ = std::io::stdout().flush();
    });
    let elapsed_3 = now.elapsed();
    println!();

    println!("Miller Rabin test with K=25");
    let now = Instant::now();
    (0..n).into_par_iter().for_each(|_| {
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
}

fn mr_calls_investigation(n: usize, k: usize) {
    let num_of_counts_per_prime: Vec<usize> = (0..n)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let count = get_prime_numer(&mut rng, k);
            //            println!("Prime number found after {} tries", &count);
            print!(".");
            _ = std::io::stdout().flush();
            count
        })
        .collect();
    println!();
    //    println!("0x{:#?}", &num_of_counts_per_prime);
    println!(
        "Average number of Miller-Rabin calls: {}",
        average(&num_of_counts_per_prime)
    );
}
