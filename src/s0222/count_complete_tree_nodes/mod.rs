// Problem 0222: count complete tree nodes

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void countNodes()
    //   TreeNode leftNodeLeftNode = new TreeNode(4);
    //   TreeNode leftNodeRightNode = new TreeNode(5);
    //   TreeNode leftNode = new TreeNode(2, leftNodeLeftNode, leftNodeRightNode);
    //   TreeNode rightNodeLeftNode = new TreeNode(6);
    //   TreeNode rightNode = new TreeNode(3, rightNodeLeftNode, null);
    //   ... (2 more lines)
    #[test]
    fn test_count_nodes() {
        // TODO: 翻译 Java 测试
    }

    // Java: void countNodes2()
    //   assertThat(new Solution().countNodes(null), equalTo(0));
    #[test]
    fn test_count_nodes2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void countNodes3()
    //   assertThat(new Solution().countNodes(new TreeNode(1)), equalTo(1));
    #[test]
    fn test_count_nodes3() {
        // TODO: 翻译 Java 测试
    }
}
