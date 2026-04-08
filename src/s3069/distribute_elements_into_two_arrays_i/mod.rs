// Problem 3069: distribute elements into two arrays i
// #Easy #Array #Simulation #2024_03_31_Time_0_ms_(100.00%)_Space_44.6_MB_(70.15%)

pub struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut s = 0;
        let mut t = 1;
        for i in 2..nums.len() {
            let mut p = i;
            if nums[s] > nums[t] {
                for _ in (s + 1)..i {
                    nums.swap(p, p - 1);
                    p -= 1;
                }
                s += 1;
                t += 1;
            } else {
                for _ in (t + 1)..i {
                    nums.swap(p, p - 1);
                    p -= 1;
                }
                t += 1;
            }
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void resultArray()
    //   assertThat(new Solution().resultArray(new int[] {2, 1, 3}), equalTo(new int[] {2, 3, 1}));
    #[test]
    fn test_result_array() {
        assert_eq!(Solution::result_array(vec![2, 1, 3]), vec![2, 3, 1]);
    }

    // Java: void resultArray2()
    //   assertThat(
    //   new Solution().resultArray(new int[] {5, 4, 3, 8}),
    //   equalTo(new int[] {5, 3, 4, 8}));
    #[test]
    fn test_result_array2() {
        assert_eq!(Solution::result_array(vec![5, 4, 3, 8]), vec![5, 3, 4, 8]);
    }
}
