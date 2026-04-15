// Problem 3357: Minimize the Maximum Adjacent Element Difference
// #Hard #Array #Greedy #Binary_Search

pub struct Solution;

impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_adj = 0;
        let mut mina = i32::MAX;
        let mut maxb = i32::MIN;

        for i in 0..n - 1 {
            let a = nums[i];
            let b = nums[i + 1];
            if a > 0 && b > 0 {
                max_adj = max_adj.max((a - b).abs());
            } else if a > 0 || b > 0 {
                mina = mina.min(a.max(b));
                maxb = maxb.max(a.max(b));
            }
        }

        let mut res = 0;
        let mut i = 0;
        while i < n {
            if (i > 0 && nums[i - 1] == -1) || nums[i] > 0 {
                i += 1;
                continue;
            }
            let start = i;
            while i < n && nums[i] == -1 {
                i += 1;
            }
            let mut a = i32::MAX;
            let mut b = i32::MIN;
            if start > 0 {
                a = a.min(nums[start - 1]);
                b = b.max(nums[start - 1]);
            }
            if i < n {
                a = a.min(nums[i]);
                b = b.max(nums[i]);
            }
            if a <= b {
                if i - start == 1 {
                    res = res.max((maxb - a).min(b - mina));
                } else {
                    res = res.max((maxb - a).min((b - mina).min((maxb - mina + 2) / 3 * 2)));
                }
            }
        }
        max_adj.max((res + 1) / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minDifference()
    //   assertThat(new Solution().minDifference(new int[] {1, 2, -1, 10, 8}), equalTo(4));
    #[test]
    fn test_min_difference() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minDifference2()
    //   assertThat(new Solution().minDifference(new int[] {-1, -1, -1}), equalTo(0));
    #[test]
    fn test_min_difference2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minDifference3()
    //   assertThat(new Solution().minDifference(new int[] {-1, 10, -1, 8}), equalTo(1));
    #[test]
    fn test_min_difference3() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minDifference4()
    //   assertThat(new Solution().minDifference(new int[] {14, -1, -1, 46}), equalTo(11));
    #[test]
    fn test_min_difference4() {
        // TODO: 翻译 Java 测试
    }
}
