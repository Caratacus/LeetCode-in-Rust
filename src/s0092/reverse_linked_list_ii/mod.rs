// Problem 0092: reverse linked list ii

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        todo!()
    }

    pub fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void reverseBetween()
    //   ListNode node1 = LinkedListUtils.createSinglyLinkedList(Arrays.asList(1, 2, 3, 4, 5));
    //   assertThat(new Solution().reverseBetween(node1, 2, 4).toString(), equalTo("1, 4, 3, 2, 5"));
    #[test]
    fn test_reverse_between() {
        // TODO: 翻译 Java 测试
    }

    // Java: void reverseBetween2()
    //   ListNode node1 = LinkedListUtils.createSinglyLinkedList(Arrays.asList(5));
    //   assertThat(new Solution().reverseBetween(node1, 1, 1).toString(), equalTo("5"));
    #[test]
    fn test_reverse_between2() {
        // TODO: 翻译 Java 测试
    }
}
