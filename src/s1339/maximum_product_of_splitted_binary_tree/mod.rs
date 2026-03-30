// Problem 1339: maximum product of splitted binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn sum_tree(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }

    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maxProduct()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 2, 3, 4, 5, 6));
    //   assertThat(new Solution().maxProduct(treeNode), equalTo(110));
    #[test]
    fn test_max_product() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxProduct2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, null, 2, 3, 4, null, null, 5, 6));
    //   assertThat(new Solution().maxProduct(treeNode), equalTo(90));
    #[test]
    fn test_max_product2() {
        // TODO: 翻译 Java 测试
    }
}
