// Problem 2295: replace elements in an array

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut index_map: HashMap<i32, usize> = nums
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect();
        let mut result = nums;
        for op in operations {
            if let Some(&idx) = index_map.get(&op[0]) {
                result[idx] = op[1];
                index_map.remove(&op[0]);
                index_map.insert(op[1], idx);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void arrayChange()
    //   assertThat(
    //   new Solution()
    //   .arrayChange(new int[] {1, 2, 4, 6}, new int[][] {{1, 3}, {4, 7}, {6, 1}}),
    //   equalTo(new int[] {3, 2, 7, 1}));
    #[test]
    fn test_array_change() {
        // TODO: 翻译 Java 测试
    }

    // Java: void arrayChange2()
    //   assertThat(
    //   new Solution().arrayChange(new int[] {1, 2}, new int[][] {{1, 3}, {2, 1}, {3, 2}}),
    //   equalTo(new int[] {2, 1}));
    #[test]
    fn test_array_change2() {
        // TODO: 翻译 Java 测试
    }
}
