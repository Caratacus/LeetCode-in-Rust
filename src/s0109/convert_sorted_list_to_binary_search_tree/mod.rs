// Problem 0109: convert sorted list to binary search tree

use crate::common::list_node::ListNode;
use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void convertSortedListToBST()
    //   ListNode nodes =
    //   new ListNode(
    //   -10, new ListNode(-3, new ListNode(0, new ListNode(5, new ListNode(9)))));
    //   assertThat(
    //   new Solution().sortedListToBST(nodes).toString(),
    //   ... (1 more lines)
    #[test]
    fn test_convert_sorted_list_to_bst() {
        // TODO: 翻译 Java 测试
    }

    // Java: void convertSortedListToBST2()
    //   assertThat(new Solution().sortedListToBST(null), equalTo(null));
    #[test]
    fn test_convert_sorted_list_to_bst2() {
        // TODO: 翻译 Java 测试
    }
}
