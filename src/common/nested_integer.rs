/// 嵌套整数（用于扁平化嵌套列表迭代器等题目）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NestedInteger {
    Integer(i32),
    List(Vec<NestedInteger>),
}

impl NestedInteger {
    pub fn integer(val: i32) -> Self {
        Self::Integer(val)
    }

    pub fn list() -> Self {
        Self::List(vec![])
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn get_integer(&self) -> Option<i32> {
        match self {
            Self::Integer(val) => Some(*val),
            Self::List(_) => None,
        }
    }

    pub fn get_list(&mut self) -> Option<&mut Vec<NestedInteger>> {
        match self {
            Self::List(items) => Some(items),
            Self::Integer(_) => None,
        }
    }

    pub fn add(&mut self, ni: NestedInteger) {
        match self {
            Self::List(items) => items.push(ni),
            Self::Integer(_) => {
                *self = Self::List(vec![ni]);
            }
        }
    }
}
