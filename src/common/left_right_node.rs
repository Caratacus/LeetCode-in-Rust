/// 带 next 指针的树节点（populate right pointers）
use std::cell::RefCell;
use std::rc::Rc;

pub type LeftRightTreeLink = Option<Rc<RefCell<LeftRightNode>>>;

#[derive(Debug)]
pub struct LeftRightNode {
    pub val: i32,
    pub left: LeftRightTreeLink,
    pub right: LeftRightTreeLink,
    pub next: LeftRightTreeLink,
}

impl LeftRightNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
            next: None,
        }
    }
}
