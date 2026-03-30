// Problem 0083: remove duplicates from sorted list

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
    //   ListNode head = LinkedListUtils.createSinglyLinkedList(Arrays.asList(1, 1, 2));
    //   assertThat(new Solution().deleteDuplicates(head).toString(), equalTo("1, 2"));
    #[test]
    fn test_delete_duplicates() {
        // TODO: 翻译 Java 测试
    }

    // Java: void deleteDuplicates2()
    //   ListNode head = LinkedListUtils.createSinglyLinkedList(Arrays.asList(1, 1, 2, 3, 3));
    //   assertThat(new Solution().deleteDuplicates(head).toString(), equalTo("1, 2, 3"));
    #[test]
    fn test_delete_duplicates2() {
        // TODO: 翻译 Java 测试
    }
}
