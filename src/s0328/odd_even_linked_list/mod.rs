// Problem 0328: odd even linked list

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void oddEvenList()
    //   ListNode node = new ListNode(1);
    //   node.next = new ListNode(2);
    //   node.next.next = new ListNode(3);
    //   node.next.next.next = new ListNode(4);
    //   node.next.next.next.next = new ListNode(5);
    //   ... (7 more lines)
    #[test]
    fn test_odd_even_list() {
        // TODO: 翻译 Java 测试
    }

    // Java: void oddEvenList2()
    //   ListNode node = new ListNode(2);
    //   node.next = new ListNode(1);
    //   node.next.next = new ListNode(3);
    //   node.next.next.next = new ListNode(5);
    //   node.next.next.next.next = new ListNode(6);
    //   ... (10 more lines)
    #[test]
    fn test_odd_even_list2() {
        // TODO: 翻译 Java 测试
    }
}
