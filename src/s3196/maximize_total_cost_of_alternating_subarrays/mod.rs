// Problem 3196: maximize total cost of alternating subarrays
// #Medium #Array #Dynamic_Programming #2024_06_26_Time_1_ms_(100.00%)_Space_61.5_MB_(68.90%)

pub struct Solution;

impl Solution {
    pub fn maximum_total_cost(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut add_result = nums[0] as i64;
        let mut sub_result = nums[0] as i64;
        for i in 1..n {
            let temp_add = add_result.max(sub_result) + nums[i] as i64;
            let temp_sub = add_result - nums[i] as i64;
            add_result = temp_add;
            sub_result = temp_sub;
        }
        add_result.max(sub_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maximumTotalCost()
    //   assertThat(new Solution().maximumTotalCost(new int[] {1, -2, 3, 4}), equalTo(10L));
    #[test]
    fn test_maximum_total_cost() {
        assert_eq!(Solution::maximum_total_cost(vec![1, -2, 3, 4]), 10);
    }

    // Java: void maximumTotalCost2()
    //   assertThat(new Solution().maximumTotalCost(new int[] {1, -1, 1, -1}), equalTo(4L));
    #[test]
    fn test_maximum_total_cost2() {
        assert_eq!(Solution::maximum_total_cost(vec![1, -1, 1, -1]), 4);
    }

    // Java: void maximumTotalCost3()
    //   assertThat(new Solution().maximumTotalCost(new int[] {0}), equalTo(0L));
    #[test]
    fn test_maximum_total_cost3() {
        assert_eq!(Solution::maximum_total_cost(vec![0]), 0);
    }

    // Java: void maximumTotalCost4()
    //   assertThat(new Solution().maximumTotalCost(new int[] {1, -1}), equalTo(2L));
    #[test]
    fn test_maximum_total_cost4() {
        assert_eq!(Solution::maximum_total_cost(vec![1, -1]), 2);
    }
}
