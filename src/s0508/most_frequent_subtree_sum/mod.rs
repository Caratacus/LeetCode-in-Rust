// Problem 0508: most frequent subtree sum

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void findFrequentTreeSum()
    //   TreeNode treeNode = TreeUtils.constructBinaryTree(Arrays.asList(5, 2, -3));
    //   int[] expected = new int[] {2, -3, 4};
    //   assertThat(new Solution().findFrequentTreeSum(treeNode), equalTo(expected));
    #[test]
    fn test_find_frequent_tree_sum() {
        // TODO: 翻译 Java 测试
    }

    // Java: void findFrequentTreeSum2()
    //   TreeNode treeNode = TreeUtils.constructBinaryTree(Arrays.asList(5, 2, -5));
    //   int[] expected = new int[] {2};
    //   assertThat(new Solution().findFrequentTreeSum(treeNode), equalTo(expected));
    #[test]
    fn test_find_frequent_tree_sum2() {
        // TODO: 翻译 Java 测试
    }
}
