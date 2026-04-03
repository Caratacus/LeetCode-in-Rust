// Problem 1572: Matrix Diagonal Sum
// #Easy #Array #Matrix
// #Big_O_Time_O(n)_Space_O(1)

pub struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let mut sum = 0;

        for i in 0..n {
            // Primary diagonal (top-left to bottom-right)
            sum += mat[i][i];
            // Secondary diagonal (top-right to bottom-left)
            sum += mat[i][n - 1 - i];
        }

        // If n is odd, subtract the center element (counted twice)
        if n % 2 == 1 {
            sum -= mat[n / 2][n / 2];
        }

        sum
    }
}
