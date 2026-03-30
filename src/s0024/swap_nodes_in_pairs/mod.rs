// Problem 0024: swap nodes in pairs

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void swapPairs()
    //   ListNode head = LinkedListUtils.createSinglyLinkedList(Arrays.asList(1, 2, 3, 4));
    //   assertThat(new Solution().swapPairs(head).toString(), equalTo("2, 1, 4, 3"));
    #[test]
    fn test_swap_pairs() {
        // TODO: 翻译 Java 测试
    }

    // Java: void swapPairs2()
    //   assertThat(new Solution().swapPairs(new ListNode(1)).toString(), equalTo("1"));
    #[test]
    fn test_swap_pairs2() {
        // TODO: 翻译 Java 测试
    }
}
