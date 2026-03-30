// Problem 0655: print binary tree

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void printTree()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 2));
    //   List<List<String>> expected =
    //   Arrays.asList(Arrays.asList("", "1", ""), Arrays.asList("2", "", ""));
    //   assertThat(new Solution().printTree(treeNode), equalTo(expected));
    #[test]
    fn test_print_tree() {
        // TODO: 翻译 Java 测试
    }

    // Java: void printTree2()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 2, 3, null, 4));
    //   List<List<String>> expected =
    //   Arrays.asList(
    //   Arrays.asList("", "", "", "1", "", "", ""),
    //   Arrays.asList("", "2", "", "", "", "3", ""),
    //   ... (2 more lines)
    #[test]
    fn test_print_tree2() {
        // TODO: 翻译 Java 测试
    }
}
