/// Difficulty: [Medium](https://leetcode.com/problems/count-primes/)
///
/// Counts the number of prime numbers less than a non-negative integer `n`.
///
/// This function uses the Sieve of Eratosthenes algorithm to identify prime numbers.
///
/// # Arguments
///
/// * `n` - The non-negative integer up to which prime numbers are counted.
///         Primes are counted that are less than `n` (exclusive).
///
/// # Examples
///
/// ```
/// assert_eq!(count_primes(10), 4); // There are four prime numbers less than 10: 2, 3, 5, 7
/// ```
///
/// # Note
///
/// * The function is optimized for counting primes up to relatively small numbers.
///   For very large values of `n`, performance may decrease.
pub fn count_primes(n: i32) -> i32 {
	let mut sieve: Vec<bool> = (0..n).map(|n| !(n == 0 || n == 1)).collect();

	let mut prime: usize = 2;

	while prime.pow(2) < n as usize {
		sieve
			.iter_mut()
			.skip(prime.pow(2))
			.step_by(prime)
			.for_each(|is_prime| *is_prime = false);

		prime = sieve
			.iter()
			.enumerate()
			.skip(prime + 1)
			.skip_while(|(_, &is_prime)| !is_prime)
			.next()
			.map_or(prime + 1, |(next_prime, _)| next_prime);
	}

	sieve.iter().filter(|&&is_prime| is_prime).count() as i32
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_10() {
		assert_eq!(count_primes(10), 4); // primes < 10: 2, 3, 5, 7
	}

	#[test]
	fn test_20() {
		assert_eq!(count_primes(20), 8); // primes < 20: 2, 3, 5, 7, 11, 13, 17, 19
	}

	#[test]
	fn test_0() {
		assert_eq!(count_primes(0), 0); // no primes < 0
	}

	#[test]
	fn test_1() {
		assert_eq!(count_primes(1), 0); // no primes < 1
	}

	#[test]
	fn test_2() {
		assert_eq!(count_primes(2), 0); // no primes < 2
	}

	#[test]
	fn test_100() {
		assert_eq!(count_primes(100), 25); // primes < 100
	}
}
