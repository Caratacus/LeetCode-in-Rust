// Problem 0021: merge two sorted lists

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void mergeTwoLists()
    //   ListNode l1 = LinkedListUtils.createSinglyLinkedList(Arrays.asList(1, 2, 4));
    //   ListNode l2 = LinkedListUtils.createSinglyLinkedList(Arrays.asList(1, 3, 4));
    //   assertThat(new Solution().mergeTwoLists(l1, l2).toString(), equalTo("1, 1, 2, 3, 4, 4"));
    #[test]
    fn test_merge_two_lists() {
        // TODO: 翻译 Java 测试
    }

    // Java: void mergeTwoLists2()
    //   assertThat(
    //   new Solution().mergeTwoLists(new ListNode(), new ListNode()).toString(),
    //   equalTo("0, 0"));
    #[test]
    fn test_merge_two_lists2() {
        // TODO: 翻译 Java 测试
    }
}
