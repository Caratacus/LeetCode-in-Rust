// Problem 0142: linked list cycle ii

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn detect_cycle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void detectCycle()
    //   ListNode listNode1 = new ListNode(3);
    //   listNode1.next = new ListNode(2);
    //   listNode1.next.next = new ListNode(0);
    //   listNode1.next.next.next = new ListNode(-4);
    //   listNode1.next.next.next.next = listNode1.next;
    //   ... (1 more lines)
    #[test]
    fn test_detect_cycle() {
        // TODO: 翻译 Java 测试
    }

    // Java: void detectCycle2()
    //   ListNode listNode1 = new ListNode(1);
    //   listNode1.next = new ListNode(2);
    //   listNode1.next.next = listNode1;
    //   assertThat(new Solution().detectCycle(listNode1) == listNode1, equalTo(true));
    #[test]
    fn test_detect_cycle2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void detectCycle3()
    //   ListNode listNode1 = new ListNode(1);
    //   assertThat(new Solution().detectCycle(listNode1), equalTo(null));
    #[test]
    fn test_detect_cycle3() {
        // TODO: 翻译 Java 测试
    }
}
