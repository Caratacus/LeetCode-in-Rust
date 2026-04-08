// Problem 3096: minimum levels to gain more points
// #Medium #Array #Prefix_Sum #2024_04_19_Time_3_ms_(99.97%)_Space_59.6_MB_(49.99%)

pub struct Solution;

impl Solution {
    pub fn minimum_levels(possible: Vec<i32>) -> i32 {
        let n = possible.len();
        let sum: i32 = possible.iter().sum();

        if sum == 0 && n == 2 {
            return -1;
        }
        if sum == 0 && n > 2 {
            return 1;
        }

        let mut sum_left = 0;
        for i in 0..n - 1 {
            sum_left += possible[i];
            let sum_right = sum - sum_left;
            let dan_score = sum_left - ((i + 1) as i32 - sum_left);
            let bob_score = sum_right - ((n - i - 1) as i32 - sum_right);
            if dan_score > bob_score {
                return (i + 1) as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumLevels()
    //   assertThat(new Solution().minimumLevels(new int[] {1, 0, 1, 0}), equalTo(1));
    #[test]
    fn test_minimum_levels() {
        assert_eq!(Solution::minimum_levels(vec![1, 0, 1, 0]), 1);
    }

    // Java: void minimumLevels2()
    //   assertThat(new Solution().minimumLevels(new int[] {1, 1, 1, 1, 1}), equalTo(3));
    #[test]
    fn test_minimum_levels2() {
        assert_eq!(Solution::minimum_levels(vec![1, 1, 1, 1, 1]), 3);
    }

    // Java: void minimumLevels3()
    //   assertThat(new Solution().minimumLevels(new int[] {0, 0}), equalTo(-1));
    #[test]
    fn test_minimum_levels3() {
        assert_eq!(Solution::minimum_levels(vec![0, 0]), -1);
    }
}
