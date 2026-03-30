// Problem 1290: convert binary number in a linked list to integer

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void getDecimalValue()
    //   ListNode listNode = new ListNode(1, new ListNode(0, new ListNode(1)));
    //   assertThat(new Solution().getDecimalValue(listNode), equalTo(5));
    #[test]
    fn test_get_decimal_value() {
        // TODO: 翻译 Java 测试
    }

    // Java: void getDecimalValue2()
    //   ListNode listNode = new ListNode(0);
    //   assertThat(new Solution().getDecimalValue(listNode), equalTo(0));
    #[test]
    fn test_get_decimal_value2() {
        // TODO: 翻译 Java 测试
    }
}
