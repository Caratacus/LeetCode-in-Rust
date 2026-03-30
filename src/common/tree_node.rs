/// 二叉树节点
use std::cell::RefCell;
use std::rc::Rc;

pub type TreeLink = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeLink,
    pub right: TreeLink,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn new_with_children(val: i32, left: TreeLink, right: TreeLink) -> Self {
        Self { val, left, right }
    }
}
