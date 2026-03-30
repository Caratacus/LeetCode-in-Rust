// Problem 2816: double a number represented as a linked list

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void doubleIt()
    //   assertThat(
    //   new Solution()
    //   .doubleIt(LinkedListUtils.contructLinkedList(new int[] {1, 8, 9}))
    //   .toString(),
    //   equalTo("3, 7, 8"));
    #[test]
    fn test_double_it() {
        // TODO: 翻译 Java 测试
    }

    // Java: void doubleIt2()
    //   assertThat(
    //   new Solution()
    //   .doubleIt(LinkedListUtils.contructLinkedList(new int[] {9, 9, 9}))
    //   .toString(),
    //   equalTo("1, 9, 9, 8"));
    #[test]
    fn test_double_it2() {
        // TODO: 翻译 Java 测试
    }
}
