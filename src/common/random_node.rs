/// 带随机指针的链表节点
pub type RandomListLink = Option<Box<RandomNode>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandomNode {
    pub val: i32,
    pub next: RandomListLink,
    pub random: RandomListLink,
}

impl RandomNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            next: None,
            random: None,
        }
    }
}
