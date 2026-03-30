// Problem 1171: remove zero sum consecutive nodes from linked list

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void removeZeroSumSublists()
    //   ListNode head = LinkedListUtils.contructLinkedList(new int[] {1, 2, -3, 3, 1});
    //   ListNode expected = LinkedListUtils.contructLinkedList(new int[] {3, 1});
    //   assertThat(
    //   new Solution().removeZeroSumSublists(head).toString(),
    //   equalTo(expected.toString()));
    #[test]
    fn test_remove_zero_sum_sublists() {
        // TODO: 翻译 Java 测试
    }

    // Java: void removeZeroSumSublists2()
    //   ListNode head = LinkedListUtils.contructLinkedList(new int[] {1, 2, 3, -3, 4});
    //   ListNode expected = LinkedListUtils.contructLinkedList(new int[] {1, 2, 4});
    //   assertThat(
    //   new Solution().removeZeroSumSublists(head).toString(),
    //   equalTo(expected.toString()));
    #[test]
    fn test_remove_zero_sum_sublists2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void removeZeroSumSublists3()
    //   ListNode head = LinkedListUtils.contructLinkedList(new int[] {1, 2, 3, -3, -2});
    //   ListNode expected = LinkedListUtils.contructLinkedList(new int[] {1});
    //   assertThat(
    //   new Solution().removeZeroSumSublists(head).toString(),
    //   equalTo(expected.toString()));
    #[test]
    fn test_remove_zero_sum_sublists3() {
        // TODO: 翻译 Java 测试
    }
}
