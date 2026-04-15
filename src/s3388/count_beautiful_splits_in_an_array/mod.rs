// Problem 3388: count beautiful splits in an array

pub struct Solution;

impl Solution {
    pub fn beautiful_splits(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut lcp = vec![vec![0i32; n + 1]; n + 1];
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if nums[i] == nums[j] {
                    lcp[i][j] = 1 + lcp[i + 1][j + 1];
                }
            }
        }
        let mut res = 0;
        for i in 1..n {
            for j in (i + 1)..n {
                let lcp1 = lcp[0][i].min(i as i32).min((j - i) as i32);
                let lcp2 = lcp[i][j].min((j - i) as i32).min((n - j) as i32);
                if lcp1 >= i as i32 || lcp2 >= (j - i) as i32 {
                    res += 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void beautifulSplits()
    //   assertThat(new Solution().beautifulSplits(new int[] {1, 1, 2, 1}), equalTo(2));
    #[test]
    fn test_beautiful_splits() {
        // TODO: 翻译 Java 测试
    }

    // Java: void beautifulSplits2()
    //   assertThat(new Solution().beautifulSplits(new int[] {1, 2, 3, 4}), equalTo(0));
    #[test]
    fn test_beautiful_splits2() {
        // TODO: 翻译 Java 测试
    }
}
