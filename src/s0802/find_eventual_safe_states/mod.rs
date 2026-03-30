// Problem 0802: find eventual safe states

pub struct Solution;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void eventualSafeNodes()
    //   assertThat(
    //   new Solution()
    //   .eventualSafeNodes(new int[][] {{1, 2}, {2, 3}, {5}, {0}, {5}, {}, {}}),
    //   equalTo(Arrays.asList(2, 4, 5, 6)));
    #[test]
    fn test_eventual_safe_nodes() {
        // TODO: 翻译 Java 测试
    }

    // Java: void eventualSafeNodes2()
    //   assertThat(
    //   new Solution()
    //   .eventualSafeNodes(new int[][] {{1, 2, 3, 4}, {1, 2}, {3, 4}, {0, 4}, {}}),
    //   equalTo(Arrays.asList(4)));
    #[test]
    fn test_eventual_safe_nodes2() {
        // TODO: 翻译 Java 测试
    }
}
