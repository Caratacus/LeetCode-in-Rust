// Problem 0876: middle of the linked list

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void middleNode()
    //   ListNode head = LinkedListUtils.createSinglyLinkedList(Arrays.asList(1, 2, 3, 4, 5));
    //   assertThat(
    //   new Solution().middleNode(head).toString(),
    //   equalTo(LinkedListUtils.createSinglyLinkedList(Arrays.asList(3, 4, 5)).toString()));
    #[test]
    fn test_middle_node() {
        // TODO: 翻译 Java 测试
    }

    // Java: void middleNode2()
    //   ListNode head = LinkedListUtils.createSinglyLinkedList(Arrays.asList(1, 2, 3, 4, 5, 6));
    //   assertThat(
    //   new Solution().middleNode(head).toString(),
    //   equalTo(LinkedListUtils.createSinglyLinkedList(Arrays.asList(4, 5, 6)).toString()));
    #[test]
    fn test_middle_node2() {
        // TODO: 翻译 Java 测试
    }
}
