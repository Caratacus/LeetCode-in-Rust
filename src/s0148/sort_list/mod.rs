// Problem 0148: sort list

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void sortList()
    //   ListNode listNode1 = new ListNode(4);
    //   listNode1.next = new ListNode(2);
    //   listNode1.next.next = new ListNode(1);
    //   listNode1.next.next.next = new ListNode(3);
    //   assertThat(new Solution().sortList(listNode1).toString(), equalTo("1, 2, 3, 4"));
    #[test]
    fn test_sort_list() {
        // TODO: 翻译 Java 测试
    }

    // Java: void sortList2()
    //   ListNode listNode1 = new ListNode(-1);
    //   listNode1.next = new ListNode(5);
    //   listNode1.next.next = new ListNode(3);
    //   listNode1.next.next.next = new ListNode(4);
    //   listNode1.next.next.next.next = new ListNode(0);
    //   ... (1 more lines)
    #[test]
    fn test_sort_list2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void sortList3()
    //   assertThat(new Solution().sortList(null), equalTo(null));
    #[test]
    fn test_sort_list3() {
        // TODO: 翻译 Java 测试
    }
}
