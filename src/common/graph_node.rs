/// 图节点（clone graph）
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct GraphNode {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<GraphNode>>>,
}

impl GraphNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            neighbors: vec![],
        }
    }
}
