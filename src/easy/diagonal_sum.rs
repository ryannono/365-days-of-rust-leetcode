/// Difficu`: [Easy](https://leetcode.com/problems/matrix-diagonal-sum)
///
/// Computes the sum of the diagonals of a given square matrix.
///
/// Specifically, this function calculates the sum of all the elements on the 
/// primary diagonal (from the top-left to the bottom-right) and the secondary
/// diagonal (from the top-right to the bottom-left).
///
/// # Arguments
///
/// * `mat` - A 2D square matrix for which the diagonal sum is to be calculated.
///
/// # Examples
///
/// ```
/// let mat = vec![
///     vec![1, 2, 3],
///     vec![4, 5, 6],
///     vec![7, 8, 9]
/// ];
/// assert_eq!(diagonal_sum(mat), 30); // 1 + 5 + 9 + 3 + 7 = 25
/// ```
///
/// # Note
///
/// * This function expects the input matrix `mat` to be square. Behavior is 
///   undefined for non-square matrices.
pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
	let mut sum = 0;

	for step in 0..mat.len() {
			if step != mat.len() - 1 - step {
					sum += mat[step][step] + mat[step][mat.len() - 1 - step];
			} else {
					sum += mat[step][step];
			}
	}

	sum
}