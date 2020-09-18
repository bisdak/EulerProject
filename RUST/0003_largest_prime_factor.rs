use std::time::{Instant};

fn get_vec_prime_list(range: i32) -> Vec<i32> {
    let mut primes: Vec<i32> = vec![];  
    let mut number = if range % 2 == 0 {
        range - 1
    } else {
        range
    };
    
    let mut i = 3;

    while number != 1 {
        if primes.iter().any(|&x| i % x == 0) {
            i += 2;
            continue;
        } else {
            primes.push(i);
        }

        if number % i == 0 {
            number /= i;
        } else {
            i += 2;
        }
    }
    primes
}

// fn is_prime(num: i64) -> bool {
//     if &num == get_vec_prime_list(num).last().unwrap() {
//         return true;
//     }
//     false
// }

fn main() {
    let start = Instant::now();    
    let limit: i64 = 600_851_475_143;
    let new_limit = (limit as f64).sqrt();

    let mut primes = get_vec_prime_list(new_limit as i32);
    let mut largest_prime_factor: i32 = 1;

    for _ in 0..primes.len() {
        let lastval = primes.pop().unwrap();
        if limit % lastval as i64 == 0 {
            largest_prime_factor = lastval;
            break;
        }
    }

    let duration = start.elapsed();

    println!("{}", largest_prime_factor);
    println!("{:?}", duration);
}