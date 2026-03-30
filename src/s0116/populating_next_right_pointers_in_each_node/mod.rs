// Problem 0116: populating next right pointers in each node

use crate::common::left_right_node::LeftRightNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn connect(root: Option<Rc<RefCell<LeftRightNode>>>) -> Option<Rc<RefCell<LeftRightNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void connect()
    //   assertThat(new Solution().connect(null), equalTo(null));
    #[test]
    fn test_connect() {
        // TODO: 翻译 Java 测试
    }

    // Java: void connect2()
    //   Node node =
    //   new Node(
    //   1,
    //   new Node(2, new Node(4), new Node(5), null),
    //   new Node(3, new Node(6), new Node(7), null),
    //   ... (11 more lines)
    #[test]
    fn test_connect2() {
        // TODO: 翻译 Java 测试
    }
}
