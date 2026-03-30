// Problem 0513: find bottom left tree value

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void findBottomLeftValue()
    //   TreeNode treeNode = TreeUtils.constructBinaryTree(Arrays.asList(2, 1, 3));
    //   assertThat(new Solution().findBottomLeftValue(treeNode), equalTo(1));
    #[test]
    fn test_find_bottom_left_value() {
        // TODO: 翻译 Java 测试
    }

    // Java: void findBottomLeftValue2()
    //   TreeNode treeNode =
    //   TreeUtils.constructBinaryTree(Arrays.asList(1, 2, 3, 4, null, 5, 6, null, null, 7));
    //   assertThat(new Solution().findBottomLeftValue(treeNode), equalTo(7));
    #[test]
    fn test_find_bottom_left_value2() {
        // TODO: 翻译 Java 测试
    }
}
