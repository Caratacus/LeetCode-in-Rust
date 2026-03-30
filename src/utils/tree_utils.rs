use crate::common::tree_node::{TreeLink, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

/// 按层序从 Vec<Option<i32>> 构造二叉树
pub fn tree_from_vec(vals: Vec<Option<i32>>) -> TreeLink {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
    let mut queue = vec![Rc::clone(&root)];
    let mut i = 1;

    while i < vals.len() {
        let node = queue.remove(0);

        if i < vals.len() {
            if let Some(val) = vals[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().left = Some(Rc::clone(&left));
                queue.push(left);
            }
            i += 1;
        }

        if i < vals.len() {
            if let Some(val) = vals[i] {
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().right = Some(Rc::clone(&right));
                queue.push(right);
            }
            i += 1;
        }
    }

    Some(root)
}
