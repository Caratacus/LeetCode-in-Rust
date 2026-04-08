// Problem 3086: minimum moves to pick k ones
// #Hard #Array #Greedy #Prefix_Sum #Sliding_Window

pub struct Solution;

impl Solution {
    pub fn minimum_moves(nums: Vec<i32>, k: i32, max_changes: i32) -> i64 {
        let n = nums.len();
        let mut max_adj_len = 0;
        let mut num_one = 0;
        let mut l = 0;

        for r in 0..n {
            if nums[r] != 1 {
                max_adj_len = max_adj_len.max(r - l);
                l = r + 1;
            } else {
                num_one += 1;
            }
        }
        max_adj_len = 3.min(max_adj_len.max(n - l));

        if max_adj_len + max_changes as usize >= k as usize {
            if max_adj_len >= k as usize {
                return k as i64 - 1;
            } else {
                return 0i64.max(max_adj_len as i64 - 1) + (k as i64 - max_adj_len as i64) * 2;
            }
        }

        let mut ones = vec![0usize; num_one];
        let mut ind = 0;
        for i in 0..n {
            if nums[i] == 1 {
                ones[ind] = i;
                ind += 1;
            }
        }

        let mut pre_sum = vec![0i64; ones.len() + 1];
        for i in 1..pre_sum.len() {
            pre_sum[i] = pre_sum[i - 1] + ones[i - 1] as i64;
        }

        let target = (k - max_changes) as usize;
        l = 0;
        let mut res = i64::MAX;

        while l + target <= ones.len() {
            let r = l + target - 1;
            let mid = (l + r) / 2;
            let median = ones[mid] as i64;
            let sum1 = pre_sum[mid + 1] - pre_sum[l];
            let sum2 = pre_sum[r + 1] - pre_sum[mid + 1];
            let area1 = (mid - l + 1) as i64 * median;
            let area2 = (r - mid) as i64 * median;
            let cur_res = area1 - sum1 + sum2 - area2;
            res = res.min(cur_res);
            l += 1;
        }

        res + 2 * max_changes as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumMoves()
    //   assertThat(
    //   new Solution().minimumMoves(new int[] {1, 1, 0, 0, 0, 1, 1, 0, 0, 1}, 3, 1),
    //   equalTo(3L));
    #[test]
    fn test_minimum_moves() {
        assert_eq!(
            Solution::minimum_moves(vec![1, 1, 0, 0, 0, 1, 1, 0, 0, 1], 3, 1),
            3
        );
    }

    // Java: void minimumMoves2()
    //   assertThat(new Solution().minimumMoves(new int[] {0, 0, 0, 0}, 2, 3), equalTo(4L));
    #[test]
    fn test_minimum_moves2() {
        assert_eq!(Solution::minimum_moves(vec![0, 0, 0, 0], 2, 3), 4);
    }

    // Java: void minimumMoves3()
    //   assertThat(new Solution().minimumMoves(new int[] {1, 0, 1, 0, 1}, 3, 0), equalTo(4L));
    #[test]
    fn test_minimum_moves3() {
        assert_eq!(Solution::minimum_moves(vec![1, 0, 1, 0, 1], 3, 0), 4);
    }
}
