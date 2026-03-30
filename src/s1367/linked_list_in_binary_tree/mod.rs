// Problem 1367: linked list in binary tree

use crate::common::list_node::ListNode;
use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void isSubPath()
    //   ListNode listNode = LinkedListUtils.contructLinkedList(new int[] {4, 2, 8});
    //   TreeNode treeNode =
    //   TreeNode.create(
    //   Arrays.asList(
    //   1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1,
    //   ... (2 more lines)
    #[test]
    fn test_is_sub_path() {
        // TODO: 翻译 Java 测试
    }

    // Java: void isSubPath2()
    //   ListNode listNode = LinkedListUtils.contructLinkedList(new int[] {1, 4, 2, 6});
    //   TreeNode treeNode =
    //   TreeNode.create(
    //   Arrays.asList(
    //   1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1,
    //   ... (2 more lines)
    #[test]
    fn test_is_sub_path2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void isSubPath3()
    //   ListNode listNode = LinkedListUtils.contructLinkedList(new int[] {1, 4, 2, 6, 8});
    //   TreeNode treeNode =
    //   TreeNode.create(
    //   Arrays.asList(
    //   1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1,
    //   ... (2 more lines)
    #[test]
    fn test_is_sub_path3() {
        // TODO: 翻译 Java 测试
    }
}
