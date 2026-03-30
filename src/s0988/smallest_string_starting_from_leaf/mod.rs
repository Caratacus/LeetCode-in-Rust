// Problem 0988: smallest string starting from leaf

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void smallestFromLeaf()
    //   assertThat(
    //   new Solution()
    //   .smallestFromLeaf(
    //   TreeUtils.constructBinaryTree(Arrays.asList(0, 1, 2, 3, 4, 3, 4))),
    //   equalTo("dba"));
    #[test]
    fn test_smallest_from_leaf() {
        // TODO: 翻译 Java 测试
    }

    // Java: void smallestFromLeaf2()
    //   assertThat(
    //   new Solution()
    //   .smallestFromLeaf(
    //   TreeUtils.constructBinaryTree(Arrays.asList(25, 1, 3, 1, 3, 0, 2))),
    //   equalTo("adz"));
    #[test]
    fn test_smallest_from_leaf2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void smallestFromLeaf3()
    //   assertThat(
    //   new Solution()
    //   .smallestFromLeaf(
    //   TreeUtils.constructBinaryTree(
    //   Arrays.asList(2, 2, 1, null, 1, 0, null, 0))),
    //   ... (1 more lines)
    #[test]
    fn test_smallest_from_leaf3() {
        // TODO: 翻译 Java 测试
    }
}
