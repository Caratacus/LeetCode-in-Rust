// Problem 1372: longest zigzag path in a binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void longestZigZag()
    //   TreeNode treeNode =
    //   TreeNode.create(
    //   Arrays.asList(
    //   1, null, 1, 1, 1, null, null, 1, 1, null, 1, null, null, null, 1,
    //   null, 1));
    //   ... (1 more lines)
    #[test]
    fn test_longest_zig_zag() {
        // TODO: 翻译 Java 测试
    }

    // Java: void longestZigZag2()
    //   TreeNode treeNode =
    //   TreeNode.create(Arrays.asList(1, 1, 1, null, 1, null, null, 1, 1, null, 1));
    //   assertThat(new Solution().longestZigZag(treeNode), equalTo(4));
    #[test]
    fn test_longest_zig_zag2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void longestZigZag3()
    //   TreeNode treeNode = TreeNode.create(Collections.singletonList(1));
    //   assertThat(new Solution().longestZigZag(treeNode), equalTo(0));
    #[test]
    fn test_longest_zig_zag3() {
        // TODO: 翻译 Java 测试
    }
}
