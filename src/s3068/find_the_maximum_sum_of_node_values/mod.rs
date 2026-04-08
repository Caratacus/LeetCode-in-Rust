// Problem 3068: find the maximum sum of node values
// #Hard #Array #Dynamic_Programming #Sorting #Greedy #Tree #Bit_Manipulation
// #2024_03_31_Time_1_ms_(100.00%)_Space_54.5_MB_(67.07%)

pub struct Solution;

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
        let mut res: i64 = 0;
        let mut d = 1 << 30;
        let mut c = 0;
        for &a in &nums {
            let b = a ^ k;
            res += i64::from(a.max(b));
            c ^= if a < b { 1 } else { 0 };
            d = d.min((a - b).abs());
        }
        res - i64::from(d * c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maximumValueSum()
    //   assertThat(
    //   new Solution()
    //   .maximumValueSum(new int[] {1, 2, 1}, 3, new int[][] {{0, 1}, {0, 2}}),
    //   equalTo(6L));
    #[test]
    fn test_maximum_value_sum() {
        assert_eq!(
            Solution::maximum_value_sum(vec![1, 2, 1], 3, vec![vec![0, 1], vec![0, 2]]),
            6
        );
    }

    // Java: void maximumValueSum2()
    //   assertThat(
    //   new Solution().maximumValueSum(new int[] {2, 3}, 7, new int[][] {{0, 1}}),
    //   equalTo(9L));
    #[test]
    fn test_maximum_value_sum2() {
        assert_eq!(
            Solution::maximum_value_sum(vec![2, 3], 7, vec![vec![0, 1]]),
            9
        );
    }

    // Java: void maximumValueSum3()
    //   assertThat(
    //   new Solution()
    //   .maximumValueSum(
    //   new int[] {7, 7, 7, 7, 7, 7},
    //   3,
    //   new int[][] {{0, 1}, {0, 2}, {0, 3}, {0, 4}, {0, 5}}),
    //   equalTo(42L));
    #[test]
    fn test_maximum_value_sum3() {
        assert_eq!(
            Solution::maximum_value_sum(
                vec![7, 7, 7, 7, 7, 7],
                3,
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]]
            ),
            42
        );
    }
}
