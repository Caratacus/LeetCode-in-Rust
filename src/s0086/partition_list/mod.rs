// Problem 0086: partition list

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void partition()
    //   ListNode head = LinkedListUtils.createSinglyLinkedList(Arrays.asList(1, 4, 3, 2, 5, 2));
    //   assertThat(new Solution().partition(head, 3).toString(), equalTo("1, 2, 2, 4, 3, 5"));
    #[test]
    fn test_partition() {
        // TODO: 翻译 Java 测试
    }

    // Java: void partition2()
    //   ListNode head = LinkedListUtils.createSinglyLinkedList(Arrays.asList(2, 1));
    //   assertThat(new Solution().partition(head, 2).toString(), equalTo("1, 2"));
    #[test]
    fn test_partition2() {
        // TODO: 翻译 Java 测试
    }
}
