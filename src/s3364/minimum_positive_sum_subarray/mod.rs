// Problem 3364: Minimum Positive Sum Subarray
// #Easy #Array #Prefix_Sum #Sliding_Window

pub struct Solution;

impl Solution {
    pub fn minimum_sum_subarray(li: Vec<i32>, l: i32, r: i32) -> i32 {
        let n = li.len();
        let mut min_sum = i32::MAX;
        let mut prefix = vec![0i32; n + 1];
        for i in 1..=n {
            prefix[i] = prefix[i - 1] + li[i - 1];
        }
        for size in l as usize..=r as usize {
            for i in (size - 1)..n {
                let sum = prefix[i + 1] - prefix[i + 1 - size];
                if sum > 0 && sum < min_sum {
                    min_sum = sum;
                }
            }
        }
        if min_sum == i32::MAX { -1 } else { min_sum }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumSumSubarray()
    //   assertThat(new Solution().minimumSumSubarray(List.of(3, -2, 1, 4), 2, 3), equalTo(1));
    #[test]
    fn test_minimum_sum_subarray() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minimumSumSubarray2()
    //   assertThat(new Solution().minimumSumSubarray(List.of(-2, 2, -3, 1), 2, 3), equalTo(-1));
    #[test]
    fn test_minimum_sum_subarray2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minimumSumSubarray3()
    //   assertThat(new Solution().minimumSumSubarray(List.of(1, 2, 3, 4), 2, 4), equalTo(3));
    #[test]
    fn test_minimum_sum_subarray3() {
        // TODO: 翻译 Java 测试
    }
}
