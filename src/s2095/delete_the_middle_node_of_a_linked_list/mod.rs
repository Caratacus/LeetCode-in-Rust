// Problem 2095: delete the middle node of a linked list

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void deleteMiddle()
    //   ListNode node = LinkedListUtils.createSinglyLinkedList(Arrays.asList(1, 3, 4, 7, 1, 2, 6));
    //   assertThat(new Solution().deleteMiddle(node).toString(), equalTo("1, 3, 4, 1, 2, 6"));
    #[test]
    fn test_delete_middle() {
        // TODO: 翻译 Java 测试
    }

    // Java: void deleteMiddle2()
    //   ListNode node = LinkedListUtils.createSinglyLinkedList(Arrays.asList(1, 2, 3, 4));
    //   assertThat(new Solution().deleteMiddle(node).toString(), equalTo("1, 2, 4"));
    #[test]
    fn test_delete_middle2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void deleteMiddle3()
    //   ListNode node = LinkedListUtils.createSinglyLinkedList(Arrays.asList(2, 1));
    //   assertThat(new Solution().deleteMiddle(node).toString(), equalTo("2"));
    #[test]
    fn test_delete_middle3() {
        // TODO: 翻译 Java 测试
    }

    // Java: void deleteMiddle4()
    //   assertThat(new Solution().deleteMiddle(new ListNode()), equalTo(null));
    #[test]
    fn test_delete_middle4() {
        // TODO: 翻译 Java 测试
    }
}
