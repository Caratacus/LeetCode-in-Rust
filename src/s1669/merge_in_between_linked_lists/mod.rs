// Problem 1669: merge in between linked lists

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void mergeInBetween()
    //   ListNode list1 = LinkedListUtils.contructLinkedList(new int[] {0, 1, 2, 3, 4, 5});
    //   ListNode list2 = LinkedListUtils.contructLinkedList(new int[] {1000000, 1000001, 1000002});
    //   ListNode expected =
    //   LinkedListUtils.contructLinkedList(
    //   new int[] {0, 1, 2, 1000000, 1000001, 1000002, 5});
    //   ... (2 more lines)
    #[test]
    fn test_merge_in_between() {
        // TODO: 翻译 Java 测试
    }

    // Java: void mergeInBetween2()
    //   ListNode list1 = LinkedListUtils.contructLinkedList(new int[] {0, 1, 2, 3, 4, 5, 6});
    //   ListNode list2 =
    //   LinkedListUtils.contructLinkedList(
    //   new int[] {1000000, 1000001, 1000002, 1000003, 1000004});
    //   ListNode expected =
    //   ... (4 more lines)
    #[test]
    fn test_merge_in_between2() {
        // TODO: 翻译 Java 测试
    }
}
