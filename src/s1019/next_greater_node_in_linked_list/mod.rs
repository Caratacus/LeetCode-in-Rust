// Problem 1019: next greater node in linked list

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void nextLargerNodes()
    //   ListNode head = LinkedListUtils.contructLinkedList(new int[] {2, 1, 5});
    //   assertThat(new Solution().nextLargerNodes(head), equalTo(new int[] {5, 5, 0}));
    #[test]
    fn test_next_larger_nodes() {
        // TODO: 翻译 Java 测试
    }

    // Java: void nextLargerNodes2()
    //   ListNode head = LinkedListUtils.contructLinkedList(new int[] {2, 7, 4, 3, 5});
    //   assertThat(new Solution().nextLargerNodes(head), equalTo(new int[] {7, 0, 5, 5, 0}));
    #[test]
    fn test_next_larger_nodes2() {
        // TODO: 翻译 Java 测试
    }
}
