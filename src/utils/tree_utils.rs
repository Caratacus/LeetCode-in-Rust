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

/// 将二叉树按层序转为 Vec<Option<i32>>
pub fn tree_to_vec(root: TreeLink) -> Vec<Option<i32>> {
    if root.is_none() {
        return vec![];
    }

    let mut result = vec![];
    let mut queue = vec![root];

    while !queue.is_empty() {
        let node = queue.remove(0);

        if let Some(n) = node {
            let n = n.borrow();
            result.push(Some(n.val));
            queue.push(n.left.clone());
            queue.push(n.right.clone());
        } else {
            result.push(None);
        }
    }

    // 移除尾部多余的 None
    while let Some(None) = result.last() {
        result.pop();
    }

    result
}
