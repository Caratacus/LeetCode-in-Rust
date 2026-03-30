// Problem 1028: recover a tree from preorder traversal

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void recoverFromPreorder()
    //   TreeNode expected = TreeNode.create(Arrays.asList(1, 2, 5, 3, 4, 6, 7));
    //   assertThat(
    //   new Solution().recoverFromPreorder("1-2--3--4-5--6--7").toString(),
    //   equalTo(expected.toString()));
    #[test]
    fn test_recover_from_preorder() {
        // TODO: 翻译 Java 测试
    }

    // Java: void recoverFromPreorder2()
    //   TreeNode expected = TreeNode.create(Arrays.asList(1, 2, 5, 3, null, 6, null, 4, null, 7));
    //   assertThat(
    //   new Solution().recoverFromPreorder("1-2--3---4-5--6---7").toString(),
    //   equalTo(expected.toString()));
    #[test]
    fn test_recover_from_preorder2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void recoverFromPreorder3()
    //   TreeNode expected = TreeNode.create(Arrays.asList(1, 401, null, 349, 88, 90));
    //   assertThat(
    //   new Solution().recoverFromPreorder("1-401--349---90--88").toString(),
    //   equalTo(expected.toString()));
    #[test]
    fn test_recover_from_preorder3() {
        // TODO: 翻译 Java 测试
    }
}
