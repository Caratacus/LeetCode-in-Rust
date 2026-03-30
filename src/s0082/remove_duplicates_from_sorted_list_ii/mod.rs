// Problem 0082: remove duplicates from sorted list ii

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void deleteDuplicates()
    //   ListNode head = LinkedListUtils.contructLinkedList(new int[] {1, 2, 3, 3, 4, 4, 5});
    //   assertThat(new Solution().deleteDuplicates(head).toString(), equalTo("1, 2, 5"));
    #[test]
    fn test_delete_duplicates() {
        // TODO: 翻译 Java 测试
    }

    // Java: void deleteDuplicates2()
    //   ListNode head = LinkedListUtils.contructLinkedList(new int[] {1, 1, 1, 2, 3});
    //   assertThat(new Solution().deleteDuplicates(head).toString(), equalTo("2, 3"));
    #[test]
    fn test_delete_duplicates2() {
        // TODO: 翻译 Java 测试
    }
}
