// Problem 0817: linked list components

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void numComponents()
    //   ListNode listNode = new ListNode(0, new ListNode(1, new ListNode(2, new ListNode(3))));
    //   assertThat(new Solution().numComponents(listNode, new int[] {0, 1, 3}), equalTo(2));
    #[test]
    fn test_num_components() {
        // TODO: 翻译 Java 测试
    }

    // Java: void numComponents2()
    //   ListNode listNode =
    //   new ListNode(0, new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4)))));
    //   assertThat(new Solution().numComponents(listNode, new int[] {0, 3, 1, 4}), equalTo(2));
    #[test]
    fn test_num_components2() {
        // TODO: 翻译 Java 测试
    }
}
