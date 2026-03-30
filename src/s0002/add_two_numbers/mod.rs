// Problem 0002: add two numbers

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void addTwoNumbers()
    //   ListNode listNode1 = LinkedListUtils.contructLinkedList(new int[] {2, 4, 3});
    //   ListNode listNode2 = LinkedListUtils.contructLinkedList(new int[] {5, 6, 4});
    //   assertThat(
    //   new Solution().addTwoNumbers(listNode1, listNode2).toString(), equalTo("7, 0, 8"));
    #[test]
    fn test_add_two_numbers() {
        // TODO: 翻译 Java 测试
    }

    // Java: void addTwoNumbers2()
    //   assertThat(
    //   new Solution().addTwoNumbers(new ListNode(0), new ListNode(0)).toString(),
    //   equalTo("0"));
    #[test]
    fn test_add_two_numbers2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void addTwoNumbers3()
    //   ListNode listNode1 = LinkedListUtils.contructLinkedList(new int[] {9, 9, 9, 9, 9, 9, 9});
    //   ListNode listNode2 = LinkedListUtils.contructLinkedList(new int[] {9, 9, 9, 9});
    //   assertThat(
    //   new Solution().addTwoNumbers(listNode1, listNode2).toString(),
    //   equalTo("8, 9, 9, 9, 0, 0, 0, 1"));
    #[test]
    fn test_add_two_numbers3() {
        // TODO: 翻译 Java 测试
    }
}
