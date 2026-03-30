// Problem 0445: add two numbers ii

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
    //   ListNode l1 = LinkedListUtils.contructLinkedList(new int[] {7, 2, 4, 3});
    //   ListNode l2 = LinkedListUtils.contructLinkedList(new int[] {5, 6, 4});
    //   assertThat(new Solution().addTwoNumbers(l1, l2).toString(), equalTo("7, 8, 0, 7"));
    #[test]
    fn test_add_two_numbers() {
        // TODO: 翻译 Java 测试
    }

    // Java: void addTwoNumbers2()
    //   ListNode l1 = LinkedListUtils.contructLinkedList(new int[] {2, 4, 3});
    //   ListNode l2 = LinkedListUtils.contructLinkedList(new int[] {5, 6, 4});
    //   assertThat(new Solution().addTwoNumbers(l1, l2).toString(), equalTo("8, 0, 7"));
    #[test]
    fn test_add_two_numbers2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void addTwoNumbers3()
    //   ListNode l1 = LinkedListUtils.contructLinkedList(new int[] {0});
    //   ListNode l2 = LinkedListUtils.contructLinkedList(new int[] {0});
    //   assertThat(new Solution().addTwoNumbers(l1, l2).toString(), equalTo("0"));
    #[test]
    fn test_add_two_numbers3() {
        // TODO: 翻译 Java 测试
    }
}
