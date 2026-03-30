// Problem 0218: the skyline problem

pub struct Solution;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void getSkyline()
    //   int[][] buildings = {{2, 9, 10}, {3, 7, 15}, {5, 12, 12}, {15, 20, 10}, {19, 24, 8}};
    //   int[][] expected = {{2, 10}, {3, 15}, {7, 12}, {12, 0}, {15, 10}, {20, 8}, {24, 0}};
    //   assertThat(new Solution().getSkyline(buildings), equalTo(ArrayUtils.getLists(expected)));
    #[test]
    fn test_get_skyline() {
        // TODO: 翻译 Java 测试
    }

    // Java: void getSkyline2()
    //   int[][] buildings = {{0, 2, 3}, {2, 5, 3}};
    //   int[][] expected = {{0, 3}, {5, 0}};
    //   assertThat(new Solution().getSkyline(buildings), equalTo(ArrayUtils.getLists(expected)));
    #[test]
    fn test_get_skyline2() {
        // TODO: 翻译 Java 测试
    }
}
