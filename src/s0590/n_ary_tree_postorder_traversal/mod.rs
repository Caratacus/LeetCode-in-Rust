// Problem 0590: n ary tree postorder traversal

use crate::common::graph_node::GraphNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn postorder(root: Option<Rc<RefCell<GraphNode>>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void postorder()
    //   Node node1 = new Node(1);
    //   Node node2 = new Node(2);
    //   Node node3 = new Node(3);
    //   Node node4 = new Node(4);
    //   node1.neighbors = Arrays.asList(node3, node2, node4);
    //   ... (4 more lines)
    #[test]
    fn test_postorder() {
        // TODO: 翻译 Java 测试
    }

    // Java: void postorder2()
    //   Node node1 = new Node(1);
    //   Node node2 = new Node(2);
    //   Node node3 = new Node(3);
    //   Node node4 = new Node(4);
    //   Node node5 = new Node(5);
    //   ... (20 more lines)
    #[test]
    fn test_postorder2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void postorder3()
    //   assertThat(new Solution().postorder(null), equalTo(Arrays.asList()));
    #[test]
    fn test_postorder3() {
        // TODO: 翻译 Java 测试
    }
}
