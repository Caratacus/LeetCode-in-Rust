// Problem 0025: reverse nodes in k group

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void reverseKGroup()
    //   ListNode head = LinkedListUtils.contructLinkedList(new int[] {1, 2, 3, 4, 5});
    //   assertThat(new Solution().reverseKGroup(head, 2).toString(), equalTo("2, 1, 4, 3, 5"));
    #[test]
    fn test_reverse_k_group() {
        // TODO: 翻译 Java 测试
    }

    // Java: void reverseKGroup2()
    //   ListNode head = LinkedListUtils.contructLinkedList(new int[] {1, 2, 3, 4, 5});
    //   assertThat(new Solution().reverseKGroup(head, 3).toString(), equalTo("3, 2, 1, 4, 5"));
    #[test]
    fn test_reverse_k_group2() {
        // TODO: 翻译 Java 测试
    }
}
