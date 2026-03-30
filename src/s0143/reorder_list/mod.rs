// Problem 0143: reorder list

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn reorder_list(head: Option<Box<ListNode>>) -> () {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void reorderList()
    //   ListNode head = LinkedListUtils.createSinglyLinkedList(Arrays.asList(1, 2, 3, 4));
    //   ListNode expected = LinkedListUtils.createSinglyLinkedList(Arrays.asList(1, 4, 2, 3));
    //   new Solution().reorderList(head);
    //   assertThat(head.toString(), equalTo(expected.toString()));
    #[test]
    fn test_reorder_list() {
        // TODO: 翻译 Java 测试
    }

    // Java: void reorderList2()
    //   ListNode head = LinkedListUtils.createSinglyLinkedList(Arrays.asList(1, 2, 3, 4, 5));
    //   ListNode expected = LinkedListUtils.createSinglyLinkedList(Arrays.asList(1, 5, 2, 4, 3));
    //   new Solution().reorderList(head);
    //   assertThat(head.toString(), equalTo(expected.toString()));
    #[test]
    fn test_reorder_list2() {
        // TODO: 翻译 Java 测试
    }
}
