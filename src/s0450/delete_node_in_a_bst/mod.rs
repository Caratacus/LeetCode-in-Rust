// Problem 0450: delete node in a bst

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void deleteNode()
    //   TreeNode input = TreeNode.create(Arrays.asList(5, 3, 6, 2, 4, null, 7));
    //   TreeNode expected = TreeNode.create(Arrays.asList(5, 4, 6, 2, null, null, 7));
    //   assertThat(new Solution().deleteNode(input, 3).toString(), equalTo(expected.toString()));
    #[test]
    fn test_delete_node() {
        // TODO: 翻译 Java 测试
    }

    // Java: void deleteNode2()
    //   TreeNode input = TreeNode.create(Arrays.asList(5, 3, 6, 2, 4, null, 7));
    //   TreeNode expected = TreeNode.create(Arrays.asList(5, 3, 6, 2, 4, null, 7));
    //   assertThat(new Solution().deleteNode(input, 0).toString(), equalTo(expected.toString()));
    #[test]
    fn test_delete_node2() {
        // TODO: 翻译 Java 测试
    }
}
