/// 单向链表节点
pub type ListLink = Option<Box<ListNode>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: ListLink,
}

impl ListNode {
    pub fn new(val: i32, next: ListLink) -> Self {
        Self { val, next }
    }
}
