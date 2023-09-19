/**
Difficulty: [Easy](https://leetcode.com/problems/climbing-stairs/description/)

Returns the number of distinct ways to climb a staircase with `n` steps.

This function computes the number of ways using dynamic programming. It builds
up a vector `ways_vec` where the i-th element represents the number of ways
to reach step i. The base cases are 0 ways to reach step 0, 1 way to reach
step 1, and 2 ways to reach step 2.

# Arguments

* `n` - The number of steps in the staircase. It should be a non-negative 
        integer.

# Returns

* An `i32` representing the number of distinct ways to climb the staircase
  with `n` steps.

# Examples

```
# use your_crate::climb_stairs;
assert_eq!(climb_stairs(2), 2);
assert_eq!(climb_stairs(3), 3);
```
*/
pub fn climb_stairs(n: i32) -> i32 {
  let mut ways_vec:Vec<i32> = vec![0, 1, 2];

  if ways_vec.contains(&n) {
    return n;
  }

  while ways_vec.len() <= n as usize {
     ways_vec.push(ways_vec[ways_vec.len() - 2] + ways_vec[ways_vec.len() - 1]);
  }

  ways_vec[ways_vec.len() - 1]
}


#[cfg(test)]
mod tests {
    use super::climb_stairs;

    #[test]
    fn test_1() {
        assert_eq!(climb_stairs(1), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(climb_stairs(3), 3);
    }

    #[test]
    fn test_4() {
        assert_eq!(climb_stairs(4), 5);
    }

    #[test]
    fn test_5() {
        assert_eq!(climb_stairs(5), 8);
    }

    #[test]
    fn test_large_input() {
        assert_eq!(climb_stairs(35), 14930352);
    }

    #[test]
    fn test_zero() {
        assert_eq!(climb_stairs(0), 0);
    }
}
