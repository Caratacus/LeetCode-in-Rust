// Problem 0114: flatten binary tree to linked list

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn flatten(root: Option<Rc<RefCell<TreeNode>>>) -> () {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void flatten()
    //   TreeNode root = TreeUtils.constructBinaryTree(Arrays.asList(1, 2, 5, 3, 4, null, 6));
    //   new Solution().flatten(root);
    //   assertThat(root.toString(), equalTo("1,null,2,null,3,null,4,null,5,null,6"));
    #[test]
    fn test_flatten() {
        // TODO: 翻译 Java 测试
    }

    // Java: void flatten2()
    //   TreeNode root = TreeUtils.constructBinaryTree(Collections.singletonList(0));
    //   new Solution().flatten(root);
    //   assertThat(root.toString(), equalTo("0"));
    #[test]
    fn test_flatten2() {
        // TODO: 翻译 Java 测试
    }
}
