// Problem 0206: reverse linked list

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void reverseList()
    //   ListNode headActual = new ListNode(1);
    //   headActual.next = new ListNode(2);
    //   headActual.next.next = new ListNode(3);
    //   headActual.next.next.next = new ListNode(4);
    //   headActual.next.next.next.next = new ListNode(5);
    //   ... (1 more lines)
    #[test]
    fn test_reverse_list() {
        // TODO: 翻译 Java 测试
    }

    // Java: void reverseList2()
    //   ListNode headActual = new ListNode(1);
    //   headActual.next = new ListNode(2);
    //   assertThat(new Solution().reverseList(headActual).toString(), equalTo("2, 1"));
    #[test]
    fn test_reverse_list2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void reverseList3()
    //   assertThat(new Solution().reverseList(null), equalTo(null));
    #[test]
    fn test_reverse_list3() {
        // TODO: 翻译 Java 测试
    }
}
