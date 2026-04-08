// Problem 3148: maximum difference score in a grid
// #Medium #Array #Dynamic_Programming #Matrix #2024_05_15_Time_5_ms_(100.00%)_Space_67.4_MB_(5.12%)

pub struct Solution;

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len() - 1;
        let n = grid[m].len();
        let mut max_rb = vec![0; n];
        let n_idx = n - 1;
        let mut mx = grid[m][n_idx];
        max_rb[n_idx] = mx;
        let mut result = i32::MIN;

        for i in (0..n_idx).rev() {
            let x = grid[m][i];
            result = result.max(mx - x);
            mx = mx.max(x);
            max_rb[i] = mx;
        }

        for i in (0..m).rev() {
            mx = 0;
            for j in (0..n).rev() {
                mx = mx.max(max_rb[j]);
                let x = grid[i][j];
                result = result.max(mx - x);
                max_rb[j] = mx.max(x);
                mx = max_rb[j];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_score() {
        assert_eq!(
            Solution::max_score(vec![vec![9, 5, 7, 3], vec![8, 9, 6, 1], vec![6, 7, 14, 3], vec![2, 5, 3, 1]]),
            9
        );
    }

    #[test]
    fn test_max_score2() {
        assert_eq!(
            Solution::max_score(vec![vec![4, 3, 2], vec![3, 2, 1]]),
            -1
        );
    }
}
