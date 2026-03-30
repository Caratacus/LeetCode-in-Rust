// Problem 3217: delete nodes from linked list present in array

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void modifiedList()
    //   assertThat(
    //   new Solution()
    //   .modifiedList(
    //   new int[] {1, 2, 3},
    //   LinkedListUtils.contructLinkedList(new int[] {1, 2, 3, 4, 5}))
    //   ... (2 more lines)
    #[test]
    fn test_modified_list() {
        // TODO: 翻译 Java 测试
    }

    // Java: void modifiedList2()
    //   assertThat(
    //   new Solution()
    //   .modifiedList(
    //   new int[] {1},
    //   LinkedListUtils.contructLinkedList(new int[] {1, 2, 1, 2, 1, 2}))
    //   ... (2 more lines)
    #[test]
    fn test_modified_list2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void modifiedList3()
    //   assertThat(
    //   new Solution()
    //   .modifiedList(
    //   new int[] {5},
    //   LinkedListUtils.contructLinkedList(new int[] {1, 2, 3, 4}))
    //   ... (2 more lines)
    #[test]
    fn test_modified_list3() {
        // TODO: 翻译 Java 测试
    }
}
