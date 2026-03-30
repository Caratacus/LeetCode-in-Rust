// Problem 0108: convert sorted array to binary search tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn sorted_array_to_bst(num: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void sortedArrayToBST()
    //   assertThat(
    //   new Solution().sortedArrayToBST(new int[] {-10, -3, 0, 5, 9}).toString(),
    //   equalTo("0,-10,null,-3,5,null,9"));
    #[test]
    fn test_sorted_array_to_bst() {
        // TODO: 翻译 Java 测试
    }

    // Java: void sortedArrayToBST2()
    //   assertThat(
    //   new Solution().sortedArrayToBST(new int[] {1, 3}).toString(), equalTo("1,null,3"));
    #[test]
    fn test_sorted_array_to_bst2() {
        // TODO: 翻译 Java 测试
    }
}
