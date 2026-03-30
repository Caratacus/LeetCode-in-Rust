// Problem 0237: delete node in a linked list

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_node(node: Option<Box<ListNode>>) -> () {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void deleteNode()
    //   ListNode headActual = new ListNode(1);
    //   headActual.next = new ListNode(2);
    //   headActual.next.next = new ListNode(3);
    //   headActual.next.next.next = new ListNode(4);
    //   new Solution().deleteNode(headActual.next.next);
    //   ... (1 more lines)
    #[test]
    fn test_delete_node() {
        // TODO: 翻译 Java 测试
    }

    // Java: void deleteNode2()
    //   ListNode headActual = new ListNode(1);
    //   headActual.next = new ListNode(2);
    //   headActual.next.next = new ListNode(3);
    //   headActual.next.next.next = new ListNode(4);
    //   new Solution().deleteNode(headActual);
    //   ... (1 more lines)
    #[test]
    fn test_delete_node2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void deleteNode3()
    //   ListNode headActual = new ListNode(1);
    //   headActual.next = new ListNode(2);
    //   headActual.next.next = new ListNode(3);
    //   headActual.next.next.next = new ListNode(4);
    //   new Solution().deleteNode(headActual.next);
    //   ... (1 more lines)
    #[test]
    fn test_delete_node3() {
        // TODO: 翻译 Java 测试
    }
}
