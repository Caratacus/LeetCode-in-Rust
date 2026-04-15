// Problem 3355: zero array transformation i

pub struct Solution;

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let n = nums.len();
        let mut sum: i64 = nums.iter().map(|&x| x as i64).sum();
        if sum == 0 {
            return true;
        }
        let mut diff = vec![0i64; n + 1];
        for q in &queries {
            let low = q[0] as usize;
            let high = q[1] as usize;
            diff[low] -= 1;
            if high + 1 < n {
                diff[high + 1] += 1;
            }
        }
        let mut nums = nums;
        for i in 0..n {
            if i > 0 {
                diff[i] += diff[i - 1];
            }
            nums[i] = (nums[i] as i64 + diff[i]) as i32;
            sum += diff[i];
            if nums[i] > 0 {
                return false;
            }
        }
        sum <= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void isZeroArray()
    //   assertThat(
    //   new Solution().isZeroArray(new int[] {1, 0, 1}, new int[][] {{0, 2}}),
    //   equalTo(true));
    #[test]
    fn test_is_zero_array() {
        // TODO: 翻译 Java 测试
    }

    // Java: void isZeroArray2()
    //   assertThat(
    //   new Solution().isZeroArray(new int[] {4, 3, 2, 1}, new int[][] {{1, 3}, {0, 2}}),
    //   equalTo(false));
    #[test]
    fn test_is_zero_array2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void isZeroArray3()
    //   assertThat(
    //   new Solution().isZeroArray(new int[] {-1, 0, 1}, new int[][] {{1, 3}, {0, 2}}),
    //   equalTo(true));
    #[test]
    fn test_is_zero_array3() {
        // TODO: 翻译 Java 测试
    }
}
