// Problem 0046: permutations

pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void permute()
    //   int[][] expected = {{1, 2, 3}, {1, 3, 2}, {2, 1, 3}, {2, 3, 1}, {3, 1, 2}, {3, 2, 1}};
    //   assertThat(
    //   new Solution().permute(new int[] {1, 2, 3}),
    //   equalTo(ArrayUtils.getLists(expected)));
    #[test]
    fn test_permute() {
        // TODO: 翻译 Java 测试
    }

    // Java: void permute2()
    //   int[][] expected = {{0, 1}, {1, 0}};
    //   assertThat(
    //   new Solution().permute(new int[] {0, 1}), equalTo(ArrayUtils.getLists(expected)));
    #[test]
    fn test_permute2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void permute3()
    //   int[][] expected = {{1}};
    //   assertThat(new Solution().permute(new int[] {1}), equalTo(ArrayUtils.getLists(expected)));
    #[test]
    fn test_permute3() {
        // TODO: 翻译 Java 测试
    }
}
