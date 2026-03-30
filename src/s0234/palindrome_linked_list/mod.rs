// Problem 0234: palindrome linked list

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void isPalindrome()
    //   ListNode headActual = new ListNode(1);
    //   headActual.next = new ListNode(2);
    //   headActual.next.next = new ListNode(2);
    //   headActual.next.next.next = new ListNode(1);
    //   assertThat(new Solution().isPalindrome(headActual), equalTo(true));
    #[test]
    fn test_is_palindrome() {
        // TODO: 翻译 Java 测试
    }

    // Java: void isPalindrome2()
    //   ListNode headActual = new ListNode(1);
    //   headActual.next = new ListNode(2);
    //   assertThat(new Solution().isPalindrome(headActual), equalTo(false));
    #[test]
    fn test_is_palindrome2() {
        // TODO: 翻译 Java 测试
    }
}
