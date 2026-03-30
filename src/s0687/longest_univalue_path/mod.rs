// Problem 0687: longest univalue path

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void longestUnivaluePath()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(5, 4, 5, 1, 1, 5));
    //   assertThat(new Solution().longestUnivaluePath(treeNode), equalTo(2));
    #[test]
    fn test_longest_univalue_path() {
        // TODO: 翻译 Java 测试
    }

    // Java: void longestUnivaluePath2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 4, 5, 4, 4, 5));
    //   assertThat(new Solution().longestUnivaluePath(treeNode), equalTo(2));
    #[test]
    fn test_longest_univalue_path2() {
        // TODO: 翻译 Java 测试
    }
}
