// Problem 0725: split linked list in parts

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void splitListToParts()
    //   ListNode listNode = LinkedListUtils.contructLinkedList(new int[] {1, 2, 3});
    //   ListNode[] expected =
    //   new ListNode[] {
    //   LinkedListUtils.contructLinkedList(new int[] {1}),
    //   LinkedListUtils.contructLinkedList(new int[] {2}),
    //   ... (7 more lines)
    #[test]
    fn test_split_list_to_parts() {
        // TODO: 翻译 Java 测试
    }

    // Java: void splitListToParts2()
    //   ListNode listNode =
    //   LinkedListUtils.contructLinkedList(new int[] {1, 2, 3, 4, 5, 6, 7, 8, 9, 10});
    //   ListNode[] expected =
    //   new ListNode[] {
    //   LinkedListUtils.contructLinkedList(new int[] {1, 2, 3, 4}),
    //   ... (6 more lines)
    #[test]
    fn test_split_list_to_parts2() {
        // TODO: 翻译 Java 测试
    }
}
