// Problem 1110: delete nodes and return forest

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        local_to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void delNodes()
    //   TreeNode root = TreeNode.create(Arrays.asList(1, 2, 3, 4, 5, 6, 7));
    //   assertThat(
    //   new Solution().delNodes(root, new int[] {3, 5}).toString(),
    //   equalTo(
    //   Arrays.asList(
    //   ... (4 more lines)
    #[test]
    fn test_del_nodes() {
        // TODO: 翻译 Java 测试
    }

    // Java: void delNodes2()
    //   TreeNode root = TreeNode.create(Arrays.asList(1, 2, 4, null, 3));
    //   assertThat(
    //   new Solution().delNodes(root, new int[] {3}).toString(),
    //   equalTo(
    //   Collections.singletonList(TreeNode.create(Arrays.asList(1, 2, 4)))
    //   ... (1 more lines)
    #[test]
    fn test_del_nodes2() {
        // TODO: 翻译 Java 测试
    }
}
