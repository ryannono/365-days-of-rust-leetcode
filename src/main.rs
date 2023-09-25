use std::collections::VecDeque;

pub mod easy;
pub mod hard;
pub mod medium;
pub mod util;

fn main() {
	println!("{}", count_primes(10));
	println!("{}", count_primes(0));
	println!("{}", count_primes(200));
	println!("{}", count_primes(999983));
}

pub fn count_primes(n: i32) -> i32 {
	let mut sieve: Vec<bool> = (0..n).map(|n| !(n == 0 || n == 1)).collect();

	let mut prime: usize = 2;

	while prime.pow(2) < n as usize {
		sieve
			.iter_mut()
			.skip(prime.pow(2))
			.step_by(prime)
			.for_each(|bool_val| *bool_val = false);

		prime = sieve
			.iter()
			.enumerate()
			.skip(prime + 1)
			.skip_while(|(_, &bool_val)| !bool_val)
			.next()
			.map_or(prime + 1, |(next_prime, _)| next_prime);
	}

	sieve.iter().filter(|&&is_prime| is_prime).count() as i32
}
