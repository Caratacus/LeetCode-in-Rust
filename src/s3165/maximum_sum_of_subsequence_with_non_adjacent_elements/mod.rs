// Problem 3165: maximum sum of subsequence with non adjacent elements
// #Hard #Array #Dynamic_Programming #Divide_and_Conquer #Segment_Tree
// #2024_11_09_Time_64_ms_(100.00%)_Space_64.1_MB_(97.01%)

pub struct Solution;

const YY: usize = 0;
const YN: usize = 1;
const NY: usize = 2;
const NN: usize = 3;
const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn maximum_sum_subsequence(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let tree = Self::build(&nums);
        let mut tree = tree;
        let mut result: i64 = 0;
        for query in &queries {
            result += Self::set(&mut tree, query[0] as usize, query[1]);
            result %= MOD;
        }
        result as i32
    }

    fn build(nums: &[i32]) -> Vec<[i64; 4]> {
        let len = nums.len();
        let mut size = 1;
        while size < len {
            size <<= 1;
        }
        let mut tree = vec![[0i64; 4]; size * 2];
        for i in 0..len {
            tree[size + i][YY] = nums[i] as i64;
        }
        for i in (1..size).rev() {
            tree[i][YY] = (tree[2 * i][YY] + tree[2 * i + 1][NY])
                .max(tree[2 * i][YN] + tree[2 * i + 1][YY].max(tree[2 * i + 1][NY]));
            tree[i][YN] = (tree[2 * i][YY] + tree[2 * i + 1][NN])
                .max(tree[2 * i][YN] + tree[2 * i + 1][YN].max(tree[2 * i + 1][NN]));
            tree[i][NY] = (tree[2 * i][NY] + tree[2 * i + 1][NY])
                .max(tree[2 * i][NN] + tree[2 * i + 1][YY].max(tree[2 * i + 1][NY]));
            tree[i][NN] = (tree[2 * i][NY] + tree[2 * i + 1][NN])
                .max(tree[2 * i][NN] + tree[2 * i + 1][YN].max(tree[2 * i + 1][NN]));
        }
        tree
    }

    fn set(tree: &mut Vec<[i64; 4]>, idx: usize, val: i32) -> i64 {
        let size = tree.len() / 2;
        tree[size + idx][YY] = val as i64;
        let mut i = (size + idx) / 2;
        while i > 0 {
            tree[i][YY] = (tree[2 * i][YY] + tree[2 * i + 1][NY])
                .max(tree[2 * i][YN] + tree[2 * i + 1][YY].max(tree[2 * i + 1][NY]));
            tree[i][YN] = (tree[2 * i][YY] + tree[2 * i + 1][NN])
                .max(tree[2 * i][YN] + tree[2 * i + 1][YN].max(tree[2 * i + 1][NN]));
            tree[i][NY] = (tree[2 * i][NY] + tree[2 * i + 1][NY])
                .max(tree[2 * i][NN] + tree[2 * i + 1][YY].max(tree[2 * i + 1][NY]));
            tree[i][NN] = (tree[2 * i][NY] + tree[2 * i + 1][NN])
                .max(tree[2 * i][NN] + tree[2 * i + 1][YN].max(tree[2 * i + 1][NN]));
            i /= 2;
        }
        tree[1][YY]
            .max(tree[1][YN])
            .max(tree[1][NY])
            .max(tree[1][NN])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maximumSumSubsequence()
    //   assertThat(
    //   new Solution()
    //   .maximumSumSubsequence(new int[] {3, 5, 9}, new int[][] {{1, -2}, {0, -3}}),
    //   equalTo(21));
    #[test]
    fn test_maximum_sum_subsequence() {
        assert_eq!(
            Solution::maximum_sum_subsequence(vec![3, 5, 9], vec![vec![1, -2], vec![0, -3]]),
            21
        );
    }

    // Java: void maximumSumSubsequence2()
    //   assertThat(
    //   new Solution().maximumSumSubsequence(new int[] {0, -1}, new int[][] {{0, -5}}),
    //   equalTo(0));
    #[test]
    fn test_maximum_sum_subsequence2() {
        assert_eq!(
            Solution::maximum_sum_subsequence(vec![0, -1], vec![vec![0, -5]]),
            0
        );
    }
}
