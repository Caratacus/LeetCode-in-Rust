// Problem 1609: even odd tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void isEvenOddTree()
    //   TreeNode treeNode =
    //   TreeNode.create(Arrays.asList(1, 10, 4, 3, null, 7, 9, 12, 8, 6, null, null, 2));
    //   assertThat(new Solution().isEvenOddTree(treeNode), equalTo(true));
    #[test]
    fn test_is_even_odd_tree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void isEvenOddTree2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(5, 4, 2, 3, 3, 7));
    //   assertThat(new Solution().isEvenOddTree(treeNode), equalTo(false));
    #[test]
    fn test_is_even_odd_tree2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void isEvenOddTree3()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(5, 9, 1, 3, 5, 7));
    //   assertThat(new Solution().isEvenOddTree(treeNode), equalTo(false));
    #[test]
    fn test_is_even_odd_tree3() {
        // TODO: 翻译 Java 测试
    }
}
