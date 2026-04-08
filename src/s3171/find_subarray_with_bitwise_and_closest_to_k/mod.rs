// Problem 3171: find subarray with bitwise and closest to k
// #Hard #Array #Binary_Search #Bit_Manipulation #Segment_Tree
// #2024_06_06_Time_10_ms_(98.04%)_Space_56.3_MB_(79.06%)

pub struct Solution;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut res = i32::MAX;
        for i in 0..nums.len() {
            res = res.min((nums[i] - k).abs());
            let mut j = i as i32 - 1;
            while j >= 0 && (nums[j as usize] & nums[i]) != nums[j as usize] {
                nums[j as usize] &= nums[i];
                res = res.min((nums[j as usize] - k).abs());
                j -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumDifference()
    //   assertThat(new Solution().minimumDifference(new int[] {1, 2, 4, 5}, 3), equalTo(1));
    #[test]
    fn test_minimum_difference() {
        assert_eq!(Solution::minimum_difference(vec![1, 2, 4, 5], 3), 1);
    }

    // Java: void minimumDifference2()
    //   assertThat(new Solution().minimumDifference(new int[] {1, 2, 1, 2}, 2), equalTo(0));
    #[test]
    fn test_minimum_difference2() {
        assert_eq!(Solution::minimum_difference(vec![1, 2, 1, 2], 2), 0);
    }

    // Java: void minimumDifference3()
    //   assertThat(new Solution().minimumDifference(new int[] {1}, 10), equalTo(9));
    #[test]
    fn test_minimum_difference3() {
        assert_eq!(Solution::minimum_difference(vec![1], 10), 9);
    }
}
