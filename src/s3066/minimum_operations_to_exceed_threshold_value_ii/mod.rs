// Problem 3066: minimum operations to exceed threshold value ii
// #Medium #Array #Heap_Priority_Queue #Simulation
// #2024_03_31_Time_26_ms_(99.91%)_Space_65.7_MB_(97.28%)

pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        let mut steps = 0;
        nums.sort_unstable();
        let mut extra: Vec<i32> = Vec::new();
        let mut i = 0;
        let mut j = 0;
        while (i < n && nums[i] < k) || (j < extra.len() && extra[j] < k) {
            let min;
            let max;
            if i < n && (j >= extra.len() || extra[j] > nums[i]) {
                min = nums[i];
                i += 1;
            } else {
                min = extra[j];
                j += 1;
            }
            if i < n && (j >= extra.len() || extra[j] > nums[i]) {
                max = nums[i];
                i += 1;
            } else {
                max = extra[j];
                j += 1;
            }
            steps += 1;
            let res = 2_i64 * min as i64 + max as i64;
            if res > i32::MAX as i64 {
                extra.push(i32::MAX);
            } else {
                extra.push(res as i32);
            }
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minOperations()
    //   assertThat(new Solution().minOperations(new int[] {2, 11, 10, 1, 3}, 10), equalTo(2));
    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![2, 11, 10, 1, 3], 10), 2);
    }

    // Java: void minOperations2()
    //   assertThat(new Solution().minOperations(new int[] {1, 1, 2, 4, 9}, 20), equalTo(4));
    #[test]
    fn test_min_operations2() {
        assert_eq!(Solution::min_operations(vec![1, 1, 2, 4, 9], 20), 4);
    }
}
