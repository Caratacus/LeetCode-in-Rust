// Problem 3402: minimum operations to make columns strictly increasing

pub struct Solution;

impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut ans = 0i32;
        let cols = grid[0].len();
        let rows = grid.len();
        for c in 0..cols {
            for r in 1..rows {
                if grid[r][c] <= grid[r - 1][c] {
                    ans += grid[r - 1][c] + 1 - grid[r][c];
                    grid[r][c] = grid[r - 1][c] + 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumOperations()
    //   assertThat(
    //   new Solution().minimumOperations(new int[][] {{3, 2}, {1, 3}, {3, 4}, {0, 1}}),
    //   equalTo(15));
    #[test]
    fn test_minimum_operations() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minimumOperations2()
    //   assertThat(
    //   new Solution().minimumOperations(new int[][] {{3, 2, 1}, {2, 1, 0}, {1, 2, 3}}),
    //   equalTo(12));
    #[test]
    fn test_minimum_operations2() {
        // TODO: 翻译 Java 测试
    }
}
