// Problem 0147: insertion sort list

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void insertionSortList()
    //   ListNode listNode1 = new ListNode(4);
    //   listNode1.next = new ListNode(2);
    //   listNode1.next.next = new ListNode(1);
    //   listNode1.next.next.next = new ListNode(3);
    //   assertThat(new Solution().insertionSortList(listNode1).toString(), equalTo("1, 2, 3, 4"));
    #[test]
    fn test_insertion_sort_list() {
        // TODO: 翻译 Java 测试
    }

    // Java: void insertionSortList2()
    //   ListNode listNode1 = new ListNode(-1);
    //   listNode1.next = new ListNode(5);
    //   listNode1.next.next = new ListNode(3);
    //   listNode1.next.next.next = new ListNode(4);
    //   listNode1.next.next.next.next = new ListNode(0);
    //   ... (2 more lines)
    #[test]
    fn test_insertion_sort_list2() {
        // TODO: 翻译 Java 测试
    }
}
