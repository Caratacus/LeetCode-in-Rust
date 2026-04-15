// Problem 3393: count paths with the given xor value
// #Medium #Array #Dynamic_Programming #Math #Matrix #Bit_Manipulation

const MOD: i64 = 1_000_000_007;

pub struct Solution;

impl Solution {
    pub fn count_paths_with_xor_value(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![[-1i64; 16]; n]; m];
        Self::dfs(&grid, 0, k, 0, 0, &mut dp) as i32
    }
}

impl Solution {
    fn dfs(grid: &[Vec<i32>], xor_val: i32, k: i32, i: usize, j: usize, dp: &mut Vec<Vec<[i64; 16]>>) -> i64 {
        let m = grid.len();
        let n = grid[0].len();
        let new_xor = xor_val ^ grid[i][j];
        if dp[i][j][new_xor as usize] != -1 {
            return dp[i][j][new_xor as usize];
        }
        if i == m - 1 && j == n - 1 {
            return if new_xor == k { 1 } else { 0 };
        }
        let mut result = 0i64;
        if i + 1 < m {
            result = (result + Self::dfs(grid, new_xor, k, i + 1, j, dp)) % MOD;
        }
        if j + 1 < n {
            result = (result + Self::dfs(grid, new_xor, k, i, j + 1, dp)) % MOD;
        }
        dp[i][j][new_xor as usize] = result;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void countPathsWithXorValue()
    //   assertThat(
    //   new Solution()
    //   .countPathsWithXorValue(
    //   new int[][] {{2, 1, 5}, {7, 10, 0}, {12, 6, 4}}, 11),
    //   equalTo(3));
    #[test]
    fn test_count_paths_with_xor_value() {
        // TODO: 翻译 Java 测试
    }

    // Java: void countPathsWithXorValue2()
    //   assertThat(
    //   new Solution()
    //   .countPathsWithXorValue(
    //   new int[][] {{1, 3, 3, 3}, {0, 3, 3, 2}, {3, 0, 1, 1}}, 2),
    //   equalTo(5));
    #[test]
    fn test_count_paths_with_xor_value2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void countPathsWithXorValue3()
    //   assertThat(
    //   new Solution()
    //   .countPathsWithXorValue(
    //   new int[][] {{1, 1, 1, 2}, {3, 0, 3, 2}, {3, 0, 2, 2}}, 10),
    //   equalTo(0));
    #[test]
    fn test_count_paths_with_xor_value3() {
        // TODO: 翻译 Java 测试
    }
}
