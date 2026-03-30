// Problem 0427: construct quad tree
use std::cell::RefCell;
use std::rc::Rc;

use crate::common::graph_node::GraphNode;

pub struct Solution;

impl Solution {
    pub fn construct(grid: Vec<Vec<i32>>) -> Option<Rc<RefCell<GraphNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void construct()
    //   assertThat(
    //   new Solution().construct(new int[][] {{0, 1}, {1, 0}}).toString(),
    //   equalTo("[0,1][1,0][1,1][1,1][1,0]"));
    #[test]
    fn test_construct() {
        // TODO: 翻译 Java 测试
    }

    // Java: void construct2()
    //   assertThat(
    //   new Solution()
    //   .construct(
    //   new int[][] {
    //   {1, 1, 1, 1, 0, 0, 0, 0},
    //   ... (10 more lines)
    #[test]
    fn test_construct2() {
        // TODO: 翻译 Java 测试
    }
}
