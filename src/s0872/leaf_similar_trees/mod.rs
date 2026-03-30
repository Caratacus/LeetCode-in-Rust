// Problem 0872: leaf similar trees

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void leafSimilar()
    //   TreeNode root1 = TreeUtils.constructBinaryTree(Arrays.asList(3, 5, 6, 2, 7, 4, 1, 9, 8));
    //   TreeNode root2 = TreeUtils.constructBinaryTree(Arrays.asList(3, 5, 6, 2, 7, 4, 1, 9, 8));
    //   assertThat(new Solution().leafSimilar(root1, root2), equalTo(true));
    #[test]
    fn test_leaf_similar() {
        // TODO: 翻译 Java 测试
    }

    // Java: void leafSimilar2()
    //   TreeNode root1 =
    //   TreeUtils.constructBinaryTree(Arrays.asList(3, 5, 1, 6, 2, 9, 8, null, null, 7, 4));
    //   TreeNode root2 =
    //   TreeUtils.constructBinaryTree(
    //   Arrays.asList(
    //   ... (2 more lines)
    #[test]
    fn test_leaf_similar2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void leafSimilar3()
    //   TreeNode root1 = TreeUtils.constructBinaryTree(Arrays.asList(1, 2, 3));
    //   TreeNode root2 = TreeUtils.constructBinaryTree(Arrays.asList(1, 3, 2));
    //   assertThat(new Solution().leafSimilar(root1, root2), equalTo(false));
    #[test]
    fn test_leaf_similar3() {
        // TODO: 翻译 Java 测试
    }
}
