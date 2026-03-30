// Problem 0919: complete binary tree inserter

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct CBTInserter {
    root: Option<Rc<RefCell<TreeNode>>>,
}

impl CBTInserter {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        todo!()
    }

    pub fn insert(&mut self, val: i32) -> i32 {
        todo!()
    }

    pub fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void cBTInserterTest()
    //   TreeNode treeNode = TreeNode.create(Arrays.asList(1, 2));
    //   CBTInserter cBTInserter = new CBTInserter(treeNode);
    //   assertThat(cBTInserter.insert(3), equalTo(1));
    //   assertThat(cBTInserter.insert(4), equalTo(2));
    //   TreeNode expected = TreeNode.create(Arrays.asList(1, 2, 3, 4));
    //   ... (1 more lines)
    #[test]
    fn test_c_bt_inserter_test() {
        // TODO: 翻译 Java 测试
    }
}
