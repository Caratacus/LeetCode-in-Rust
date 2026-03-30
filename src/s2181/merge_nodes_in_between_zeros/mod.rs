// Problem 2181: merge nodes in between zeros

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void mergeNodes()
    //   ListNode head =
    //   LinkedListUtils.createSinglyLinkedList(Arrays.asList(0, 3, 1, 0, 4, 5, 2, 0));
    //   assertThat(new Solution().mergeNodes(head).toString(), equalTo("4, 11"));
    #[test]
    fn test_merge_nodes() {
        // TODO: 翻译 Java 测试
    }

    // Java: void mergeNodes2()
    //   ListNode head =
    //   LinkedListUtils.createSinglyLinkedList(Arrays.asList(0, 1, 0, 3, 0, 2, 2, 0));
    //   assertThat(new Solution().mergeNodes(head).toString(), equalTo("1, 3, 4"));
    #[test]
    fn test_merge_nodes2() {
        // TODO: 翻译 Java 测试
    }
}
