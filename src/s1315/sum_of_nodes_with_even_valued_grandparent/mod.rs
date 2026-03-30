// Problem 1315: sum of nodes with even valued grandparent

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void sumEvenGrandparent()
    //   TreeNode root =
    //   TreeNode.create(
    //   Arrays.asList(6, 7, 8, 2, 7, 1, 3, 9, null, 1, 4, null, null, null, 5));
    //   assertThat(new Solution().sumEvenGrandparent(root), equalTo(18));
    #[test]
    fn test_sum_even_grandparent() {
        // TODO: 翻译 Java 测试
    }

    // Java: void sumEvenGrandparent2()
    //   TreeNode root = TreeNode.create(Collections.singletonList(1));
    //   assertThat(new Solution().sumEvenGrandparent(root), equalTo(0));
    #[test]
    fn test_sum_even_grandparent2() {
        // TODO: 翻译 Java 测试
    }
}
