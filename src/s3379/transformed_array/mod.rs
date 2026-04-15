// Problem 3379: transformed array

pub struct Solution;

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![0; n];
        for i in 0..n {
            if nums[i] > 0 {
                res[i] = nums[((i as i32 + nums[i]) as usize) % n];
            } else if nums[i] < 0 {
                let r = nums[i].unsigned_abs() as usize / n;
                res[i] = nums[((i as i32 + nums[i] + (r as i32) * (n as i32) + n as i32).unsigned_abs() as usize) % n];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void constructTransformedArray()
    //   assertThat(
    //   new Solution().constructTransformedArray(new int[] {3, -2, 1, 1}),
    //   equalTo(new int[] {1, 1, 1, 3}));
    #[test]
    fn test_construct_transformed_array() {
        // TODO: 翻译 Java 测试
    }

    // Java: void constructTransformedArray2()
    //   assertThat(
    //   new Solution().constructTransformedArray(new int[] {-1, 4, -1}),
    //   equalTo(new int[] {-1, -1, 4}));
    #[test]
    fn test_construct_transformed_array2() {
        // TODO: 翻译 Java 测试
    }
}
