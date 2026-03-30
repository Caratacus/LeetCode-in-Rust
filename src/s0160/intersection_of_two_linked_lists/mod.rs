// Problem 0160: intersection of two linked lists

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn get_intersection_node(
        head_a: Option<Box<ListNode>>,
        head_b: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void getIntersectionNode()
    //   ListNode intersectionListNode = new ListNode(8, new ListNode(4, new ListNode(5)));
    //   ListNode nodeA = new ListNode(4, new ListNode(1, intersectionListNode));
    //   ListNode nodeB = new ListNode(5, new ListNode(6, new ListNode(1, intersectionListNode)));
    //   assertThat(new Solution().getIntersectionNode(nodeA, nodeB).val, equalTo(8));
    #[test]
    fn test_get_intersection_node() {
        // TODO: 翻译 Java 测试
    }

    // Java: void getIntersectionNode2()
    //   ListNode nodeA = new ListNode(4, new ListNode(1, new ListNode(2)));
    //   ListNode nodeB = new ListNode(5, new ListNode(6, new ListNode(1, new ListNode(2))));
    //   assertThat(new Solution().getIntersectionNode(nodeA, nodeB), equalTo(null));
    #[test]
    fn test_get_intersection_node2() {
        // TODO: 翻译 Java 测试
    }
}
