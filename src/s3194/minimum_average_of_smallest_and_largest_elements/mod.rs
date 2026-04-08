// Problem 3194: minimum average of smallest and largest elements
// #Easy #Array #Sorting #Two_Pointers #2024_06_26_Time_2_ms_(98.88%)_Space_43.5_MB_(88.12%)

pub struct Solution;

impl Solution {
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
        nums.sort();
        let l = nums.len();
        let mut m: f64 = 102.0;
        for i in 0..l / 2 {
            let avg: f64 = ((nums[i] + nums[l - i - 1]) as f64) / 2.0;
            if avg < m {
                m = avg;
            }
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumAverage()
    //   assertThat(
    //   new Solution().minimumAverage(new int[] {7, 8, 3, 4, 15, 13, 4, 1}), equalTo(5.5));
    #[test]
    fn test_minimum_average() {
        assert_eq!(
            Solution::minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1]),
            5.5
        );
    }

    // Java: void minimumAverage2()
    //   assertThat(new Solution().minimumAverage(new int[] {1, 9, 8, 3, 10, 5}), equalTo(5.5));
    #[test]
    fn test_minimum_average2() {
        assert_eq!(Solution::minimum_average(vec![1, 9, 8, 3, 10, 5]), 5.5);
    }

    // Java: void minimumAverage3()
    //   assertThat(new Solution().minimumAverage(new int[] {1, 2, 3, 7, 8, 9}), equalTo(5.0));
    #[test]
    fn test_minimum_average3() {
        assert_eq!(Solution::minimum_average(vec![1, 2, 3, 7, 8, 9]), 5.0);
    }
}
