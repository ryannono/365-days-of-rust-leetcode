/**
Difficulty: [Easy](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/)

Calculates the maximum profit that can be achieved from performing at most one operation
of buying and one operation of selling a stock.

The function iterates over the prices vector, tracking the smallest price seen so far
and the maximum profit that can be achieved at each step. The maximum profit is calculated
as the difference between the current price and the smallest price seen so far.

# Parameters
- `prices`: A vector of integers representing the prices of the stock on different days.

# Returns

An integer representing the maximum profit that could have been achieved from one buy and
one sell operation. If it is not possible to achieve any profit (i.e., if the prices vector
has less than 2 elements or the prices are in non-increasing order), the function returns 0.

# Example

```rust
use your_crate_name::max_profit;

let prices = vec![7, 1, 5, 3, 6, 4];
let result = max_profit(prices);

assert_eq!(result, 5);
*/
pub fn max_profit(prices: Vec<i32>) -> i32 {
	if prices.len() < 2 {
		return 0;
	}

	let mut profit = 0;
	let mut smallest_prev_price = prices[0];

	for &price in prices.iter().skip(1) {
		profit = profit.max(price - smallest_prev_price);

		if price < smallest_prev_price {
			smallest_prev_price = price;
		}
	}

	profit
}

#[cfg(test)]
mod tests {
	use super::max_profit;

	#[test]
	fn test_example_1() {
		let prices = vec![7, 1, 5, 3, 6, 4];
		let expected = 5;
		assert_eq!(max_profit(prices), expected);
	}

	#[test]
	fn test_example_2() {
		let prices = vec![7, 6, 4, 3, 1];
		let expected = 0;
		assert_eq!(max_profit(prices), expected);
	}

	#[test]
	fn test_fluctuating_prices() {
		let prices = vec![3, 2, 6, 5, 0, 8];
		let expected = 8;
		assert_eq!(max_profit(prices), expected);
	}

	#[test]
	fn test_single_price() {
		let prices = vec![5];
		let expected = 0;
		assert_eq!(max_profit(prices), expected);
	}

	#[test]
	fn test_empty_list() {
		let prices: Vec<i32> = vec![];
		let expected = 0;
		assert_eq!(max_profit(prices), expected);
	}
}
