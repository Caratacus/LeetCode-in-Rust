// Problem 1305: all elements in two binary search trees

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void getAllElements()
    //   assertThat(
    //   new Solution()
    //   .getAllElements(
    //   TreeNode.create(Arrays.asList(2, 1, 4)),
    //   TreeNode.create(Arrays.asList(1, 0, 3))),
    //   ... (1 more lines)
    #[test]
    fn test_get_all_elements() {
        // TODO: 翻译 Java 测试
    }

    // Java: void getAllElements2()
    //   assertThat(
    //   new Solution()
    //   .getAllElements(
    //   TreeNode.create(Arrays.asList(1, null, 8)),
    //   TreeNode.create(Arrays.asList(8, 1))),
    //   ... (1 more lines)
    #[test]
    fn test_get_all_elements2() {
        // TODO: 翻译 Java 测试
    }
}
