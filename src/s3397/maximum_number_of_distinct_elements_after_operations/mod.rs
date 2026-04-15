// Problem 3397: maximum number of distinct elements after operations

pub struct Solution;

impl Solution {
    pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut next = nums[0] - k + 1;
        let mut ans = 1;
        for i in 1..nums.len() {
            if nums[i] + k < next {
                continue;
            }
            next = next.max(nums[i] - k);
            ans += 1;
            next += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maxDistinctElements()
    //   assertThat(new Solution().maxDistinctElements(new int[] {1, 2, 2, 3, 3, 4}, 2), equalTo(6));
    #[test]
    fn test_max_distinct_elements() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxDistinctElements2()
    //   assertThat(new Solution().maxDistinctElements(new int[] {4, 4, 4, 4}, 1), equalTo(3));
    #[test]
    fn test_max_distinct_elements2() {
        // TODO: 翻译 Java 测试
    }
}
