// Problem 3362: Zero Array Transformation III
// #Medium #Array #Sorting #Greedy #Heap_Priority_Queue #Prefix_Sum

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut queries = queries;
        queries.sort_by_key(|q| q[0]);

        let n = nums.len();
        let mut last: BinaryHeap<i32> = BinaryHeap::new();
        let mut diffs = vec![0i32; n + 1];
        let mut idx = 0;
        let mut cur = 0;

        for i in 0..n {
            while idx < queries.len() && queries[idx][0] == i as i32 {
                last.push(queries[idx][1]);
                idx += 1;
            }
            cur += diffs[i];
            while cur < nums[i] {
                match last.pop() {
                    Some(r) if r >= i as i32 => {
                        cur += 1;
                        diffs[(r + 1) as usize] -= 1;
                    }
                    _ => return -1,
                }
            }
        }
        last.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maxRemoval()
    //   assertThat(
    //   new Solution()
    //   .maxRemoval(new int[] {2, 0, 2}, new int[][] {{0, 2}, {0, 2}, {1, 1}}),
    //   equalTo(1));
    #[test]
    fn test_max_removal() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxRemoval2()
    //   assertThat(
    //   new Solution()
    //   .maxRemoval(
    //   new int[] {1, 1, 1, 1},
    //   new int[][] {{1, 3}, {0, 2}, {1, 3}, {1, 2}}),
    //   ... (1 more lines)
    #[test]
    fn test_max_removal2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxRemoval3()
    //   assertThat(
    //   new Solution().maxRemoval(new int[] {1, 2, 3, 4}, new int[][] {{0, 3}}),
    //   equalTo(-1));
    #[test]
    fn test_max_removal3() {
        // TODO: 翻译 Java 测试
    }
}
