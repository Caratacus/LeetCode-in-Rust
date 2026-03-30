// Problem 1325: delete leaves with a given value

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void removeLeafNodes()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 2, 3, 2, null, 2, 4));
    //   TreeNode expected = TreeNode.create(Arrays.asList(1, null, 3, null, 4));
    //   assertThat(
    //   new Solution().removeLeafNodes(treeNode, 2).toString(),
    //   equalTo(expected.toString()));
    #[test]
    fn test_remove_leaf_nodes() {
        // TODO: 翻译 Java 测试
    }

    // Java: void removeLeafNodes2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 3, 3, 3, 2));
    //   TreeNode expected = TreeNode.create(Arrays.asList(1, 3, null, null, 2));
    //   assertThat(
    //   new Solution().removeLeafNodes(treeNode, 3).toString(),
    //   equalTo(expected.toString()));
    #[test]
    fn test_remove_leaf_nodes2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void removeLeafNodes3()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 2, null, 2, null, 2));
    //   TreeNode expected = TreeNode.create(Collections.singletonList(1));
    //   assertThat(
    //   new Solution().removeLeafNodes(treeNode, 2).toString(),
    //   equalTo(expected.toString()));
    #[test]
    fn test_remove_leaf_nodes3() {
        // TODO: 翻译 Java 测试
    }
}
