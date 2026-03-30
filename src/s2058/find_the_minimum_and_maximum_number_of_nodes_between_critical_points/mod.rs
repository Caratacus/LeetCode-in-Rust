// Problem 2058: find the minimum and maximum number of nodes between critical points

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void nodesBetweenCriticalPoints()
    //   ListNode node = LinkedListUtils.createSinglyLinkedList(Arrays.asList(3, 1));
    //   assertThat(new Solution().nodesBetweenCriticalPoints(node), equalTo(new int[] {-1, -1}));
    #[test]
    fn test_nodes_between_critical_points() {
        // TODO: 翻译 Java 测试
    }

    // Java: void nodesBetweenCriticalPoints2()
    //   ListNode node = LinkedListUtils.createSinglyLinkedList(Arrays.asList(5, 3, 1, 2, 5, 1, 2));
    //   assertThat(new Solution().nodesBetweenCriticalPoints(node), equalTo(new int[] {1, 3}));
    #[test]
    fn test_nodes_between_critical_points2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void nodesBetweenCriticalPoints3()
    //   ListNode node =
    //   LinkedListUtils.createSinglyLinkedList(Arrays.asList(1, 3, 2, 2, 3, 2, 2, 2, 7));
    //   assertThat(new Solution().nodesBetweenCriticalPoints(node), equalTo(new int[] {3, 3}));
    #[test]
    fn test_nodes_between_critical_points3() {
        // TODO: 翻译 Java 测试
    }
}
