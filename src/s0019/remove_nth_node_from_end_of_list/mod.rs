// Problem 0019: remove nth node from end of list

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void removeNthFromEnd()
    //   ListNode node1 = LinkedListUtils.contructLinkedList(new int[] {1, 2, 3, 4, 5});
    //   assertThat(new Solution().removeNthFromEnd(node1, 2).toString(), equalTo("1, 2, 3, 5"));
    #[test]
    fn test_remove_nth_from_end() {
        // TODO: 翻译 Java 测试
    }

    // Java: void removeNthFromEnd2()
    //   ListNode node1 = new ListNode();
    //   node1.val = 1;
    //   assertThat(new Solution().removeNthFromEnd(node1, 1), equalTo(null));
    #[test]
    fn test_remove_nth_from_end2() {
        // TODO: 翻译 Java 测试
    }
}
