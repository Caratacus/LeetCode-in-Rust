// Problem 0971: flip binary tree to match preorder traversal

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void flipMatchVoyage()
    //   assertThat(
    //   new Solution()
    //   .flipMatchVoyage(TreeNode.create(Arrays.asList(1, 2)), new int[] {2, 1}),
    //   equalTo(Arrays.asList(-1)));
    #[test]
    fn test_flip_match_voyage() {
        // TODO: 翻译 Java 测试
    }

    // Java: void flipMatchVoyage2()
    //   assertThat(
    //   new Solution()
    //   .flipMatchVoyage(
    //   TreeNode.create(Arrays.asList(1, 2, 3)), new int[] {1, 3, 2}),
    //   equalTo(Arrays.asList(1)));
    #[test]
    fn test_flip_match_voyage2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void flipMatchVoyage3()
    //   assertThat(
    //   new Solution()
    //   .flipMatchVoyage(
    //   TreeNode.create(Arrays.asList(1, 2, 3)), new int[] {1, 2, 3}),
    //   equalTo(Collections.emptyList()));
    #[test]
    fn test_flip_match_voyage3() {
        // TODO: 翻译 Java 测试
    }
}
