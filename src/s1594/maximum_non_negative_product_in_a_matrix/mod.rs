// Problem 1594: Maximum Non Negative Product in a Matrix
// #Medium #Array #Dynamic_Programming #Matrix
// #Big_O_Time_O(m*n)_Space_O(m*n)

const MOD: i64 = 1_000_000_007;

pub struct Solution;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }

        let rows = grid.len();
        let cols = grid[0].len();

        // dp[i][j] = (max_product, min_product)
        let mut dp: Vec<Vec<(i64, i64)>> = vec![vec![(1, 1); cols]; rows];

        // Init first cell
        dp[0][0] = (grid[0][0] as i64, grid[0][0] as i64);

        // Init first column
        for i in 1..rows {
            let val = grid[i][0] as i64;
            dp[i][0] = (val * dp[i - 1][0].0, val * dp[i - 1][0].1);
        }

        // Init first row
        for j in 1..cols {
            let val = grid[0][j] as i64;
            dp[0][j] = (val * dp[0][j - 1].0, val * dp[0][j - 1].1);
        }

        // DP
        for i in 1..rows {
            for j in 1..cols {
                let val = grid[i][j] as i64;
                let up1 = dp[i - 1][j].0 * val;
                let up2 = dp[i - 1][j].1 * val;
                let left1 = dp[i][j - 1].0 * val;
                let left2 = dp[i][j - 1].1 * val;

                let max_val = up1.max(up2).max(left1).max(left2);
                let min_val = up1.min(up2).min(left1).min(left2);
                dp[i][j] = (max_val, min_val);
            }
        }

        if dp[rows - 1][cols - 1].0 < 0 {
            -1
        } else {
            (dp[rows - 1][cols - 1].0 % MOD) as i32
        }
    }
}
