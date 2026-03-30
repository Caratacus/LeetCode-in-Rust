// Problem 0095: unique binary search trees ii

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void generateTrees()
    //   assertThat(
    //   new Solution().generateTrees(3).toString(),
    //   equalTo(
    //   "[3,2,1,null,null, 2,1,3, 3,1,null,2,null, 1,null,3,2,null, 1,null,2,null,3]"));
    #[test]
    fn test_generate_trees() {
        // TODO: 翻译 Java 测试
    }

    // Java: void generateTrees2()
    //   assertThat(new Solution().generateTrees(1).toString(), equalTo("[1]"));
    #[test]
    fn test_generate_trees2() {
        // TODO: 翻译 Java 测试
    }
}
