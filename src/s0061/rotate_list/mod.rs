// Problem 0061: rotate list

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void rotateRight()
    //   ListNode headActual = new ListNode(1);
    //   headActual.next = new ListNode(2);
    //   headActual.next.next = new ListNode(3);
    //   headActual.next.next.next = new ListNode(4);
    //   headActual.next.next.next.next = new ListNode(5);
    //   ... (1 more lines)
    #[test]
    fn test_rotate_right() {
        // TODO: 翻译 Java 测试
    }

    // Java: void rotateRight2()
    //   ListNode headActual = new ListNode(0);
    //   headActual.next = new ListNode(1);
    //   headActual.next.next = new ListNode(2);
    //   assertThat(new Solution().rotateRight(headActual, 4).toString(), equalTo("2, 0, 1"));
    #[test]
    fn test_rotate_right2() {
        // TODO: 翻译 Java 测试
    }
}
