// Problem 3152: special array ii
// #Medium #Array #Binary_Search #Prefix_Sum #2024_05_22_Time_2_ms_(99.93%)_Space_97.9_MB_(79.71%)

pub struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = nums.len();
        let mut bad = vec![0i32; n];
        for i in 1..n {
            bad[i] = bad[i - 1] + ((((nums[i - 1] ^ nums[i]) & 1) ^ 1) as i32);
        }
        let nq = queries.len();
        let mut res = vec![false; nq];
        for i in 0..nq {
            let q = &queries[i];
            res[i] = Self::calc(&bad, q[0] as usize, q[1] as usize) == 0;
        }
        res
    }

    fn calc(arr: &[i32], st: usize, end: usize) -> i32 {
        arr[end] - arr[st]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void isArraySpecial()
    //   assertThat(
    //   new Solution().isArraySpecial(new int[] {3, 4, 1, 2, 6}, new int[][] {{0, 4}}),
    //   equalTo(new boolean[] {false}));
    #[test]
    fn test_is_array_special() {
        assert_eq!(
            Solution::is_array_special(vec![3, 4, 1, 2, 6], vec![vec![0, 4]]),
            vec![false]
        );
    }

    // Java: void isArraySpecial2()
    //   assertThat(
    //   new Solution().isArraySpecial(new int[] {4, 3, 1, 6}, new int[][] {{0, 2}, {2, 3}}),
    //   equalTo(new boolean[] {false, true}));
    #[test]
    fn test_is_array_special2() {
        assert_eq!(
            Solution::is_array_special(vec![4, 3, 1, 6], vec![vec![0, 2], vec![2, 3]]),
            vec![false, true]
        );
    }
}
