use crate::common::list_node::{ListLink, ListNode};

/// 从 Vec 构造链表
pub fn linked_list_from_vec(vals: Vec<i32>) -> ListLink {
    let mut head = None;
    for &val in vals.iter().rev() {
        head = Some(Box::new(ListNode::new(val, head)));
    }
    head
}

/// 将链表转为 Vec
pub fn linked_list_to_vec(mut head: ListLink) -> Vec<i32> {
    let mut result = vec![];
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}
