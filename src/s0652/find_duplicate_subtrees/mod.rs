// Problem 0652: find duplicate subtrees

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void findDuplicateSubtrees()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 2, 3, 4, null, 2, 4, null, null, 4));
    //   TreeNode item1 = TreeNode.create(Arrays.asList(2, 4));
    //   TreeNode item2 = TreeNode.create(Collections.singletonList(4));
    //   List<TreeNode> expected = Arrays.asList(item2, item1);
    //   assertThat(
    //   ... (2 more lines)
    #[test]
    fn test_find_duplicate_subtrees() {
        // TODO: 翻译 Java 测试
    }

    // Java: void findDuplicateSubtrees2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(2, 2, 2, 3, null, 3, null));
    //   TreeNode item1 = TreeNode.create(Arrays.asList(2, 3));
    //   TreeNode item2 = TreeNode.create(Collections.singletonList(3));
    //   List<TreeNode> expected = Arrays.asList(item2, item1);
    //   assertThat(
    //   ... (2 more lines)
    #[test]
    fn test_find_duplicate_subtrees2() {
        // TODO: 翻译 Java 测试
    }
}
