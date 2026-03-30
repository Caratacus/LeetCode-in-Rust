// Problem 2487: remove nodes from linked list

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void removeNodes()
    //   assertThat(
    //   new Solution()
    //   .removeNodes(LinkedListUtils.contructLinkedList(new int[] {5, 2, 13, 3, 8}))
    //   .toString(),
    //   equalTo(LinkedListUtils.contructLinkedList(new int[] {13, 8}).toString()));
    #[test]
    fn test_remove_nodes() {
        // TODO: 翻译 Java 测试
    }

    // Java: void removeNodes2()
    //   assertThat(
    //   new Solution()
    //   .removeNodes(LinkedListUtils.contructLinkedList(new int[] {1, 1, 1, 1}))
    //   .toString(),
    //   equalTo(LinkedListUtils.contructLinkedList(new int[] {1, 1, 1, 1}).toString()));
    #[test]
    fn test_remove_nodes2() {
        // TODO: 翻译 Java 测试
    }
}
