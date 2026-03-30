// Problem 1008: construct binary search tree from preorder traversal

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void bstFromPreorder()
    //   assertThat(
    //   new Solution().bstFromPreorder(new int[] {8, 5, 1, 7, 10, 12}).toString(),
    //   equalTo(TreeNode.create(Arrays.asList(8, 5, 10, 1, 7, null, 12)).toString()));
    #[test]
    fn test_bst_from_preorder() {
        // TODO: 翻译 Java 测试
    }

    // Java: void bstFromPreorder2()
    //   assertThat(
    //   new Solution().bstFromPreorder(new int[] {1, 3}).toString(),
    //   equalTo(TreeNode.create(Arrays.asList(1, null, 3)).toString()));
    #[test]
    fn test_bst_from_preorder2() {
        // TODO: 翻译 Java 测试
    }
}
