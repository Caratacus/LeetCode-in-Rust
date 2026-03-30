// Problem 1932: merge bsts to create single bst

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn can_merge(trees: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void canMerge()
    //   List<TreeNode> nodes =
    //   Arrays.asList(
    //   TreeNode.create(Arrays.asList(2, 1)),
    //   TreeNode.create(Arrays.asList(3, 2, 5)),
    //   TreeNode.create(Arrays.asList(5, 4)));
    //   ... (3 more lines)
    #[test]
    fn test_can_merge() {
        // TODO: 翻译 Java 测试
    }

    // Java: void canMerge2()
    //   List<TreeNode> nodes =
    //   Arrays.asList(
    //   TreeNode.create(Arrays.asList(5, 3, 8)),
    //   TreeNode.create(Arrays.asList(3, 2, 6)));
    //   assertThat(
    //   ... (1 more lines)
    #[test]
    fn test_can_merge2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void canMerge3()
    //   List<TreeNode> nodes =
    //   Arrays.asList(
    //   TreeNode.create(Arrays.asList(5, 4)), TreeNode.create(Arrays.asList(3)));
    //   assertThat(
    //   new Solution().canMerge(nodes), equalTo(TreeNode.create(Collections.emptyList())));
    #[test]
    fn test_can_merge3() {
        // TODO: 翻译 Java 测试
    }
}
