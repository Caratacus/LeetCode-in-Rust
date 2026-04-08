// Problem 3041: Maximize Consecutive Elements in an Array After Modification
// #Hard #Array #Dynamic_Programming #Sorting
// #Big_O_Time_O(n)_Space_O(n)

pub struct Solution;

impl Solution {
    pub fn max_selected_elements(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut min = i32::MAX;
        for &x in &nums {
            max = max.max(x);
            min = min.min(x);
        }

        let mut count = vec![0; (max + 1) as usize];
        for &x in &nums {
            count[x as usize] += 1;
        }

        let mut dp = vec![0; (max + 2) as usize];
        let mut ans = 0;

        for x in min..=max {
            let x_usize = x as usize;
            if count[x_usize] == 0 {
                continue;
            }
            let c = count[x_usize];
            if c == 1 {
                dp[x_usize + 1] = dp[x_usize] + 1;
                dp[x_usize] = dp[x_usize - 1] + 1;
            } else {
                dp[x_usize] = dp[x_usize - 1] + 1;
                dp[x_usize + 1] = dp[x_usize] + 1;
            }
            ans = ans.max(dp[x_usize]);
            ans = ans.max(dp[x_usize + 1]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_selected_elements() {
        assert_eq!(Solution::max_selected_elements(vec![2, 1, 5, 1, 1]), 3);
    }

    #[test]
    fn test_max_selected_elements2() {
        assert_eq!(Solution::max_selected_elements(vec![1, 4, 7, 10]), 1);
    }
}
