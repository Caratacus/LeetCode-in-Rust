/// 员工（用于员工重要性题目）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Employee {
    pub id: i32,
    pub importance: i32,
    pub subordinates: Vec<i32>,
}

impl Employee {
    pub fn new(id: i32, importance: i32, subordinates: Vec<i32>) -> Self {
        Self {
            id,
            importance,
            subordinates,
        }
    }
}
