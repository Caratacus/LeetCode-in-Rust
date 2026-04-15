// Problem 3356: zero array transformation ii

// Problem 3356: Zero Array Transformation II
// #Medium #Array #Binary_Search #Prefix_Sum

pub struct Solution;

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut diff = vec![0i64; n];
        let mut idx = 0;
        let mut d: i64 = 0;

        for i in 0..n {
            d += diff[i];
            while nums[i] as i64 + d > 0 && idx < queries.len() {
                let q = &queries[idx];
                let l = q[0] as usize;
                let r = q[1] as usize;
                let val = q[2] as i64;

                if i >= l && i <= r {
                    d -= val;
                }
                diff[l] -= val;
                if r + 1 < n {
                    diff[r + 1] += val;
                }
                idx += 1;
            }
            if nums[i] as i64 + d > 0 {
                return -1;
            }
        }
        idx as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minZeroArray()
    //   assertThat(
    //   new Solution()
    //   .minZeroArray(
    //   new int[] {2, 0, 2}, new int[][] {{0, 2, 1}, {0, 2, 1}, {1, 1, 3}}),
    //   equalTo(2));
    #[test]
    fn test_min_zero_array() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minZeroArray2()
    //   assertThat(
    //   new Solution()
    //   .minZeroArray(new int[] {4, 3, 2, 1}, new int[][] {{1, 3, 2}, {0, 2, 1}}),
    //   equalTo(-1));
    #[test]
    fn test_min_zero_array2() {
        // TODO: 翻译 Java 测试
    }
}
