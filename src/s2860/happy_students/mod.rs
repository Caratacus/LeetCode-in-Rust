// Problem 2860: happy students
// #Medium #Array #Sorting #Enumeration

pub struct Solution;

impl Solution {
    pub fn count_ways(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut cnt = 0;
        let n = nums.len();

        // 如果第一个学生不是0，可以选择0个学生
        if nums[0] != 0 {
            cnt += 1;
        }

        // 遍历所有可能的分割点
        for i in 0..n - 1 {
            // 选择i+1个学生，需要满足：
            // 1. nums[i] < i+1 (第i个学生希望少于i+1人被选中)
            // 2. nums[i+1] > i+1 (第i+1个学生希望多于i+1人被选中)
            if nums[i] < (i + 1) as i32 && nums[i + 1] > (i + 1) as i32 {
                cnt += 1;
            }
        }

        // 如果可以选择所有学生
        if n as i32 > nums[n - 1] {
            cnt += 1;
        }

        cnt
    }
}
