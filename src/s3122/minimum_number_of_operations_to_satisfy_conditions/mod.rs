// Problem 3122: minimum number of operations to satisfy conditions
// #Medium #Array #Dynamic_Programming #Matrix
// #2024_04_27_Time_6_ms_(100.00%)_Space_156.6_MB_(54.30%)

pub struct Solution;

impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp = vec![[0i32; 10]; m];
        let mut cnt = vec![[0i32; 10]; m];

        for row in &grid {
            for j in 0..m {
                cnt[j][row[j] as usize] += 1;
            }
        }

        let mut first = i32::MAX;
        let mut second = i32::MAX;
        let mut first_id: i32 = -1;
        let mut second_id: i32 = -1;

        for i in 0..10 {
            dp[0][i] = (n as i32) - cnt[0][i];
            if dp[0][i] <= first {
                second = first;
                first = dp[0][i];
                second_id = first_id;
                first_id = i as i32;
            } else if dp[0][i] < second {
                second = dp[0][i];
                second_id = i as i32;
            }
        }

        for j in 1..m {
            let last_first_id = first_id;
            let last_second_id = second_id;
            first = i32::MAX;
            second = i32::MAX;
            first_id = -1;
            second_id = -1;

            for i in 0..10 {
                let fix = (n as i32) - cnt[j][i];
                let tmp = if i as i32 == last_first_id {
                    fix + dp[j - 1][last_second_id as usize]
                } else {
                    fix + dp[j - 1][last_first_id as usize]
                };

                if tmp <= first {
                    second = first;
                    first = tmp;
                    second_id = first_id;
                    first_id = i as i32;
                } else if tmp < second {
                    second = tmp;
                    second_id = i as i32;
                }
                dp[j][i] = tmp;
            }
        }

        let mut ans = i32::MAX;
        for i in 0..10 {
            ans = ans.min(dp[m - 1][i]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_operations() {
        assert_eq!(
            Solution::minimum_operations(vec![vec![1, 0, 2], vec![1, 0, 2]]),
            0
        );
    }

    #[test]
    fn test_minimum_operations2() {
        assert_eq!(
            Solution::minimum_operations(vec![vec![1, 1, 1], vec![0, 0, 0]]),
            3
        );
    }
}
