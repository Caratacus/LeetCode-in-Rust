// Problem 0203: remove linked list elements

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void removeElements()
    //   ListNode headActual = new ListNode(1);
    //   headActual.next = new ListNode(2);
    //   headActual.next.next = new ListNode(6);
    //   headActual.next.next.next = new ListNode(3);
    //   headActual.next.next.next.next = new ListNode(4);
    //   ... (4 more lines)
    #[test]
    fn test_remove_elements() {
        // TODO: 翻译 Java 测试
    }

    // Java: void removeElements2()
    //   assertThat(new Solution().removeElements(null, 1), equalTo(null));
    #[test]
    fn test_remove_elements2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void removeElements3()
    //   ListNode headActual = new ListNode(7);
    //   headActual.next = new ListNode(7);
    //   headActual.next.next = new ListNode(7);
    //   headActual.next.next.next = new ListNode(7);
    //   assertThat(new Solution().removeElements(headActual, 7), equalTo(null));
    #[test]
    fn test_remove_elements3() {
        // TODO: 翻译 Java 测试
    }
}
