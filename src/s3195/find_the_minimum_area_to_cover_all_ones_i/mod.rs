// Problem 3195: find the minimum area to cover all ones i
// #Medium #Array #Matrix #2024_06_26_Time_5_ms_(94.40%)_Space_197.2_MB_(14.87%)

pub struct Solution;

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut xmin = i32::MAX;
        let mut xmax: i32 = -1;
        let mut ymin = i32::MAX;
        let mut ymax: i32 = -1;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    xmin = xmin.min(i as i32);
                    xmax = xmax.max(i as i32);
                    ymin = ymin.min(j as i32);
                    ymax = ymax.max(j as i32);
                }
            }
        }
        (xmax - xmin + 1) * (ymax - ymin + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumArea()
    //   assertThat(new Solution().minimumArea(new int[][] {{0, 1, 0}, {1, 0, 1}}), equalTo(6));
    #[test]
    fn test_minimum_area() {
        assert_eq!(
            Solution::minimum_area(vec![vec![0, 1, 0], vec![1, 0, 1]]),
            6
        );
    }

    // Java: void minimumArea2()
    //   assertThat(new Solution().minimumArea(new int[][] {{1, 0}, {0, 0}}), equalTo(1));
    #[test]
    fn test_minimum_area2() {
        assert_eq!(Solution::minimum_area(vec![vec![1, 0], vec![0, 0]]), 1);
    }
}
